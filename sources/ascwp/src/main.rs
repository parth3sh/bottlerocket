mod config;
mod error;
use crate::config::Config;
use crate::error::Result;
use imdsclient::Error;
use imdsclient::ImdsClient;
use snafu::{OptionExt, ResultExt};
use std::env;
use std::path::{Path, PathBuf};
use tokio::runtime::Runtime;
const CONFIG_PATH: &str = "/etc/ascwp.toml";

//Uses function from imds client to fetch lifecycle state
async fn get_lifecycle_state(client: &mut ImdsClient) -> Result<String> {
    client
        .fetch_lifecycle_state()
        .await
        .context(error::ImdsRequestSnafu)?
        .context(error::ImdsNoneSnafu {
            what: "instance-id",
        })
}

async fn waiter() -> Result<()> {
    let mut client = ImdsClient::new();
    let mut lifecycle_state = get_lifecycle_state(&mut client).await?;
    while lifecycle_state.ne("InService") {
        println!("WAITING -> state is: {}", lifecycle_state);
        lifecycle_state = get_lifecycle_state(&mut client).await?;
    }
    println!("Finished waiting state is {}", lifecycle_state);
    Ok(())
}

fn getConfig() -> bool {
    let tempConfig = Config::from_file(CONFIG_PATH).unwrap();
    println!("SHOULD WAIT VALUE IS {}", tempConfig.should_wait);
    return tempConfig.should_wait;
}

#[tokio::main]
async fn main() {
    if (getConfig()) {
        waiter().await;
    }
}
