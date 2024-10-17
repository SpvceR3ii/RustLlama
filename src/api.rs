// API Module
// Contacts the Ollama API and tells it to generate text.
use reqwest::Client;
use serde::Deserialize; 
use std::{error::Error, io};
use std::io::Write;

use crate::chat::{ChatRequest, ChatResponse, ChatMessage};

#[derive(Deserialize, Debug)]
pub struct StreamChunk {
    pub message: Option<ChatMessage>,
    pub done: bool,
}

pub async fn handle_streaming_response(
    client: &Client,
    request: &ChatRequest,
    history: &mut Vec<ChatMessage>, 
) -> Result<(), Box<dyn Error>> {
    let mut resp = client
        .post("http://localhost:11434/api/chat")
        .json(request)
        .send()
        .await?;

    let mut has_output = false;
    print!("Assistant: ");

    while let Some(chunk) = resp.chunk().await? {
        if chunk.is_empty() {
            continue;
        }

        let chunk_text = String::from_utf8_lossy(&chunk);

        if let Ok(stream_chunk) = serde_json::from_str::<StreamChunk>(&chunk_text) {
            if let Some(message) = stream_chunk.message {
                print!("{}", message.content);
                io::stdout().flush()?;
                has_output = true;

                // add in the assistants response to the history
                history.push(ChatMessage {
                    role: "assistant".to_string(),
                    content: message.content.clone(),
                });
            }

            if stream_chunk.done {
                println!();
                break;
            }
        } else {
            eprintln!("Failed to parse chunk: {:?}", chunk_text);
        }
    }

    if !has_output {
        return Err("No output received from the model.".into());
    }

    Ok(())
}

pub async fn handle_non_streaming_response(
    client: &Client,
    request: &ChatRequest,
    history: &mut Vec<ChatMessage>,
) -> Result<(), Box<dyn Error>> {
    let resp = client
        .post("http://localhost:11434/api/chat")
        .json(request)
        .send()
        .await?
        .json::<ChatResponse>()
        .await?;

    println!("Assistant: {}", resp.message.content);

    // Ensures that the assistant has its history logged
    history.push(ChatMessage {
        role: "assistant".to_string(),
        content: resp.message.content.clone(),
    });

    Ok(())
}
