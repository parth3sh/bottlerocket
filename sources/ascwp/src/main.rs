mod error;
use imdsclient::ImdsClient;
use crate::error::Result;
use imdsclient::Error;
use snafu::{OptionExt, ResultExt};
use tokio::runtime::Runtime;
use std::env;



//Uses function from imds client to fetch lifecycle state
async fn get_lifecycle_state(client: &mut ImdsClient) -> Result<String> {
    client.fetch_lifecycle_state().await
    .context(error::ImdsRequestSnafu)?
    .context(error::ImdsNoneSnafu {
        what: "instance-id",
    })
}

async fn waiter() -> Result<()>{
    let mut client = ImdsClient::new();
    let mut lifecycle_state = get_lifecycle_state(&mut client).await?;
    while lifecycle_state.ne("InService") {
        println!("WAITING -> state is: {}", lifecycle_state);
        lifecycle_state = get_lifecycle_state(&mut client).await?;
    }
    println!("Finished waiting state is {}", lifecycle_state);
    Ok(())
}


#[tokio::main]
async fn main() {
    waiter().await;
}

