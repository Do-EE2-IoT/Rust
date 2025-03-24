
use calproto_rust::client::Client;



#[tokio::main]
async fn main(){
     let client = Client::config((127,0,0,1), 7878, "Do NV".to_string()).await;

}