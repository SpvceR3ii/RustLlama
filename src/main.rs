// Main Script
use reqwest::Client;
// use serde::Deserialize; keep this in for later ;)
use std::{error::Error, io::{self, Write}};

mod config;
mod chat;
mod api;

// 7-4: ...LIKE ANTENNAS TO HEAVEN
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = config::load_config("config/config.json")?;
    let client = Client::new();
    let mut history = vec![];

    history.push(chat::ChatMessage {
        role: "system".to_string(),
        content: config.system_prompt.clone(),
    });

    loop {
        let mut user_input = String::new();
        print!("You: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut user_input)?;

        let user_message = user_input.trim().to_string();
        if user_message.is_empty() {
            println!("You cannot respond with an empty message.");
            continue;
        }

        history.push(chat::ChatMessage {
            role: "user".to_string(),
            content: user_message.clone(),
        });

        let chat_request = chat::ChatRequest {
            model: config.model_name.clone(),
            messages: history.clone(),
            stream: config.do_streaming,
        };

        if config.do_streaming {
            if let Err(e) = api::handle_streaming_response(&client, &chat_request, &mut history).await {
                eprintln!("Error: {}", e);
                println!(
                    "There doesn't seem to be an output. Did you select the correct model and/or setting?"
                );
            }
        } else {
            if let Err(e) = api::handle_non_streaming_response(&client, &chat_request, &mut history).await {
                eprintln!("Error: {}", e);
                println!(
                    "There doesn't seem to be an output. Did you select the correct model and/or setting?"
                );
            }
        }
    }
}
