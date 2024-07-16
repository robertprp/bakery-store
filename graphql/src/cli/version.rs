use log::info;
use service::config::ConfigService;

#[tokio:main]
pub async fn version(config: ConfigService) {
    info!("Bakery GraphQL server version {version}", version = env!("CARGO_PKG_VERSION"));
}