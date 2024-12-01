use std::io; 
use reqwest; 

#[tokio::main] 
async fn main() {
    let mut webhook = String::new();
    println!("Enter webhook:");
    io::stdin().read_line(&mut webhook).unwrap();

    let client = reqwest::Client::new(); 


    let payload = r#"{"content": "```Github.com/Fogmalol```"}"#;

    // loop gone
    client.post(webhook.trim())
        .header("Content-Type", "application/json") 
        .body(payload)
        .send()
        .await
        .unwrap();
}
