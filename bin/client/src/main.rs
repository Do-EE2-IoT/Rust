use calproto_rust::client::Client;
use stdin_kb::stdin::{Console, ConsoleInput, Input};
use tokio;
use tokio::sync::mpsc::{Receiver, Sender};

async fn console_input_handle(tx: Sender<ConsoleInput>) {
    
    let mut console_input = Console::default();
    while let Ok(in_data) = console_input.pop().await {
        if let Err(e) = tx.send(in_data).await {
            println!("Get input error: {:?}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx): (Sender<ConsoleInput>, Receiver<ConsoleInput>) =
        tokio::sync::mpsc::channel(10);

    tokio::spawn(console_input_handle(tx));
//     let client = Client::config((127, 0, 0, 1), 7878, "Do NV".to_string()).await;

    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));

    while let Some(input) = rx.recv().await {
        println!("Get Input = {:?}", input);
    }
}
