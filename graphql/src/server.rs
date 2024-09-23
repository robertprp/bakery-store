use std::net::{Shutdown, TcpListener};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use async_graphql::{http::ALL_WEBSOCKET_PROTOCOLS, Data, ServerError};
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::{extract::{State, WebSocketUpgrade}, http::HeaderMap, response::{Html, IntoResponse, Response}, routing::get, Router, serve::Serve as AxumServer, serve};
use axum::handler::HandlerWithoutStateExt;
use error_stack::{Result, ResultExt, Report};
use lib::error::Error;
use log::{info, warn};
use serde::Deserialize;
use service::{
    message_broker::service::MessageBrokerService as BroadcastService,
    services::Services,
    store::service::StoreService,
};
use service::{cache::service::CacheService, event_queue::service::EventQueueService};
use tower::{MakeService, ServiceBuilder};
use tower_http::cors;
use service::config::ConfigService;
use crate::schema::{GQLJWTData, ServiceSchema};
use crate::{
    helpers::jwt::JWT,
    schema::{new_schema, GQLGlobalData},
    LOG_TARGET,
};
use crate::ide::altair::AltairGraphQL;

#[derive(Clone)]
struct AppState {
    config: ConfigService,
    schema: ServiceSchema,
    jwt: JWT,
}

pub struct Server {
    config: ConfigService,
}

impl Server {
    pub fn new(config: ConfigService) -> Self {
        Server { config }
    }

    pub async fn start(self) -> Result<(), Error> {
        info!(
            target: LOG_TARGET,
            "Starting GraphQL, version {}",
            env!("CARGO_PKG_VERSION")
        );

        info!(
            target: LOG_TARGET,
            "Service is listening at {}", self.config.graphql.listen
        );
        info!(
            target: LOG_TARGET,
            "GraphQL endpoint exposed at {}", self.config.graphql.endpoint
        );
        info!(
            target: LOG_TARGET,
            "GraphQL subscription endpoint exposed at {}",
            self.config.graphql.subscription_endpoint
        );
        info!(
            target: LOG_TARGET,
            "Playground endpoint available at {}", self.config.graphql.endpoint
        );
        info!(
            target: LOG_TARGET,
            "Health-check endpoint available at {}/health", self.config.graphql.endpoint
        );

        let store = StoreService::new(self.config.database.clone()).await.unwrap();

        let services = Services {
            config: self.config.clone(),
            store: store.clone(),
            message_broker: BroadcastService::new(LOG_TARGET, self.config.redis.clone()).await.unwrap(),
            cache: CacheService::new(self.config.redis.clone()).unwrap(),
            event_queue: EventQueueService::new(store.clone()),
        };

        info!("Keys {pk} and {pub} are used for JWT", pk = self.config.jwt.private_key, pub = self.config.jwt.public_key);
        let jwt = JWT::new_from_pem(
            self.config.jwt.private_key.as_bytes(),
            self.config.jwt.public_key.as_bytes(),
        )
            .expect("Failed to init JWT");

        let schema = new_schema(
            GQLGlobalData::builder()
                .services(services)
                .jwt(jwt.clone())
                .build(),
        );

        let app_state = AppState {
            config: self.config.clone(),
            schema: schema.clone(),
            jwt,
        };

        let app = Router::new()
            .route("/", get(graphql_playground).post(graphql_handler))
            .route("/health", get(health))
            .route("/ws", get(graphql_ws_handler))
            .with_state(app_state)
            .layer(
                ServiceBuilder::new()
                    .layer(
                        cors::CorsLayer::new()
                            .allow_origin(cors::Any)
                            .allow_headers(cors::Any)
                            .allow_methods(cors::Any),
                    )
                    .into_inner(),
            );

        let listener = tokio::net::TcpListener::bind(&self.config.graphql.listen)
            .await.change_context(Error::Unknown)?;

        serve(listener, app)
            .await
            .map_err(|e| Report::new(e))
            .change_context(Error::Unknown)?;

        Ok(())
    }
}

fn extract_token_from_str(value: &str) -> Option<String> {
    value.split_once(' ').map(|s| s.1).map(|s| s.to_owned())
}

fn get_auth_token_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(extract_token_from_str)
}

async fn graphql_ws_handler(
    State(state): State<AppState>,
    protocol: GraphQLProtocol,
    websocket: WebSocketUpgrade,
) -> Response {
    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |stream| {
            GraphQLWebSocket::new(stream, state.schema.clone(), protocol)
                .on_connection_init(|value| async move {
                    #[derive(Deserialize)]
                    #[serde(rename_all = "PascalCase")]
                    struct Payload {
                        authorization: String,
                    }

                    let claims = {
                        if let Ok(payload) = serde_json::from_value::<Payload>(value) {
                            let token = extract_token_from_str(&payload.authorization);
                            if token.is_none() {
                                return Err(async_graphql::Error::new("Token is invalid"));
                            }
                            let token = token.unwrap();

                            match state.jwt.decode(token) {
                                Ok(token) => Ok(Some(token.claims)),
                                Err(err) => match *err.kind() {
                                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                                        Err(async_graphql::Error::new("Token is invalid"))
                                    }
                                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                                        Err(async_graphql::Error::new("Token has expired"))
                                    }
                                    _ => {
                                        warn!("Token validation error: {}", err);
                                        Err(async_graphql::Error::new(
                                            "Unable to validate auth token",
                                        ))
                                    }
                                },
                            }?
                        } else {
                            None
                        }
                    };

                    let mut context = Data::default();
                    context.insert(GQLJWTData { claims });

                    Ok(context)
                })
                .serve()
        })
}

async fn graphql_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_req.into_inner();

    if let Some(token) = get_auth_token_from_headers(&headers) {
        let err_msg_response = |msg: &str| -> GraphQLResponse {
            async_graphql::Response::from_errors(vec![ServerError::new(msg, None)]).into()
        };

        match state.jwt.decode(token) {
            Ok(token) => {
                request = request.data(GQLJWTData {
                    claims: Some(token.claims),
                })
            }
            Err(err) => match *err.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    return err_msg_response("Token is invalid");
                }
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    return err_msg_response("Token has expired");
                }
                _ => {
                    warn!("Token validation error: {}", err);
                    return err_msg_response("Unable to validate auth token");
                }
            },
        }
    }

    state.schema.execute(request).await.into()

    // state.schema.execute_batch(request.into()).await.into()
}

async fn graphql_playground(State(state): State<AppState>) -> impl IntoResponse {
    Html(
        AltairGraphQL::build()
            .endpoint(&state.config.graphql.endpoint)
            .subscription_endpoint(&state.config.graphql.subscription_endpoint)
            .title("Bakery GQL Explorer")
            .finish(),
    )
}

async fn health() -> impl IntoResponse {
    Html("OK")
}
