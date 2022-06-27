use imdsclient::ImdsClient;
use imdsclient::Error;
use snafu::{OptionExt, ResultExt};
use tokio::runtime::Runtime;
use std::env;

//Uses function from imds client to fetch lifecycle state
async fn get_lifecycle_state(client: &mut ImdsClient) -> Result<Option<String>, Error> {
    client.fetch_lifecycle_state().await
}

//Unwraps result and option
fn unwrap_state(state: Result<Option<String>, Error>) -> String{
    match state{
        Err(err) => "Error!".to_string(),
        Ok(noError) => {
            match noError{
                Some(state) => state,
                None => "No state!".to_string(),
            }
        },
    }
}

fn main() {
    let mut runtime = Runtime::new().unwrap();
    let mut client = ImdsClient::new();
    let mut lifecycle_state = unwrap_state(runtime.block_on(get_lifecycle_state(&mut client)));
    while lifecycle_state.ne("InService") {
        println!("Waiting because state is {}", lifecycle_state);
        println!("WAITING -> state is: {}", lifecycle_state);
        lifecycle_state = unwrap_state(runtime.block_on(get_lifecycle_state(&mut client)));
    }
    println!("FINISHED waiting -> state is: {}", lifecycle_state);
}
