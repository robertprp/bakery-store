use std::ops::{Add, AddAssign, SubAssign};
use log::{error, info};
use service::config::ConfigService;
use lib::error::Error;
use error_stack::{Report, Result};

#[tokio::main]
pub async fn start(config: ConfigService, product: String) -> Result<(), Error> {
    info!("Starting to bake...");

    info!("Baking product: {}", product);
    let result = computation(1, 2).await;
    info!("Config {:?}", config.to_string());
    info!("Result: {:?}", result);

    let result = if (result == 3) {
        Err(Report::new(Error::Unknown).attach_printable("Failed to bake"))
    } else {
        info!("Baking completed");
        Ok(())
    };

    if let Err(e) = &result {
        error!("{e:?}");
    };

    result
}


pub async fn computation<T>(var1: T, var2: T) -> T::Output
where T: Add<Output = T>
{
     var1 + var2
}
