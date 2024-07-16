pub mod guards;
pub mod inputs;
pub mod types;
pub mod validators;

use std::str::FromStr;

use crate::{
    helpers::jwt::{Claims, JWT},
    mutations::Mutation,
    queries::Query,
    subscriptions::Subscription,
};
use async_graphql::{Schema};
use service::services::Services;
use uuid::Uuid;

pub type ServiceSchema = Schema<Query, Mutation, Subscription>;

#[derive(Clone)]
pub struct GQLGlobalData {
    pub services: Services,
    pub jwt: JWT,
}

#[derive(Clone)]
pub struct OrganizationId(pub Uuid);

impl From<String> for OrganizationId {
    fn from(id: String) -> Self {
        Self(Uuid::from_str(&id).unwrap())
    }
}

pub struct GQLJWTData {
    pub claims: Option<Claims>,
}

#[buildstructor::buildstructor]
impl GQLGlobalData {
    #[builder(entry = "builder", exit = "build", visibility = "pub")]
    fn builder_new(services: Services, jwt: JWT) -> Self {
        Self { services, jwt }
    }
}

pub fn new_schema(ctx: GQLGlobalData) -> ServiceSchema {
    Schema::build(Query, Mutation, Subscription)
        .data(ctx.clone())
        // .data(DataLoader::new(
        //     VaultLoader::new(ctx.services.store.clone()),
        //     tokio::spawn,
        // ))
        .finish()
}
