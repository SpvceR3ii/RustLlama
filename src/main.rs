use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Deserialize, Debug)]
struct GenerateResponse {
    response: String,
    done: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let model = "llama3.2";
    let prompt = "What is Rust programming?";
    
    let request_body = GenerateRequest {
        model,
        prompt,
        stream: false, // Change to true if you want streaming responses
    };

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&request_body)
        .send()
        .await?
        .json::<GenerateResponse>()
        .await?;

    if response.done {
        println!("Model Response: {}", response.response);
    } else {
        println!("Response stream incomplete.");
    }

    Ok(())
}
