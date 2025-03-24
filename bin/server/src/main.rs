use calproto_rust::server::Server;

#[tokio::main]
async fn main(){
    let server = Server::config((127,0,0,1), 7878).await;

}