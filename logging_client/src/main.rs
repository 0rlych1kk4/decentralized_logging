use tokio::time::{sleep, Duration};
use reqwest::Client;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct LogMessage {
    level: String,
    message: String,
}

#[tokio::main]
async fn main() {
    let server_url = env::args().nth(1).expect("No server URL provided");
    let client = Client::new();
    
    loop {
        let log_message = LogMessage {
            level: "INFO".to_string(),
            message: "This is a log message".to_string(),
        };

        let res = client.post(&server_url)
            .json(&log_message)
            .send()
            .await;

        match res {
            Ok(response) => println!("Sent log message: {:?}", response.status()),
            Err(err) => eprintln!("Error sending log message: {:?}", err),
        }

        sleep(Duration::from_secs(5)).await; // Send a log message every 5 seconds
    }
}
