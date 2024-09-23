use lib::error::Error;
use service::config::ConfigService;
use crate::server::Server;

#[tokio::main]
pub async fn start(config: ConfigService) {
    let server = Server::new(config);

    server.start().await.unwrap();
}