use tokio::runtime::Runtime;
use imdsclient::ImdsClient;
use snafu::{OptionExt, ResultExt};

async fn get_lifecycle_state(client: &mut ImdsClient) -> String {
    client
        .fetch_lifecycle_state().await
            .unwrap()
            .unwrap()
}
fn main(){
    let mut runtime = Runtime::new().unwrap();
    let mut client = ImdsClient::new();
    let lifecycle_state = runtime.block_on(get_lifecycle_state(&mut client));    
    println!("{:?}", lifecycle_state);
}

//Result<Option<String>, imdsclient::Error>