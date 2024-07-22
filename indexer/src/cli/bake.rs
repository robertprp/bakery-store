use log::info;
use service::config::ConfigService;

#[tokio:main]
pub async fn start(config: ConfigService) {
    info!("Starting to bake...")
}
