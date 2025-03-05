// AI module - Handles interaction with the AI assistant

use reqwest::Client;
use serde_json::{json, Value};
use std::collections::VecDeque;
use std::io::{self, Write};
use crate::config;

/// Chat with AI assistant and maintain conversation history
///
/// # Arguments
/// * `message` - The user's message to send to the AI
/// * `history` - The conversation history to provide context
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error
pub async fn chat_with_ai(message: &str, history: &mut VecDeque<(String, String)>) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize HTTP client
    let client = Client::new();
    
    // Build context from history
    let mut context = String::new();
    for (user_msg, ai_msg) in history.iter() {
        context.push_str(&format!("User: {}\nAssistant: {}\n", user_msg, ai_msg));
    }
    
    // Create the prompt with context and clear instructions
    let prompt = if context.is_empty() {
        format!("You are ZorpAI, a helpful AI assistant. Respond directly to the user's question without including any 'User:' prefixes or meta-instructions in your response.\n\nUser: {}\nAssistant:", message)
    } else {
        format!("You are ZorpAI, a helpful AI assistant. Respond directly to the user's question without including any 'User:' prefixes or meta-instructions in your response.\n\n{}\nUser: {}\nAssistant:", context, message)
    };
    
    println!("\n🤖 ZorpAI is thinking...");
    
    // Create the request payload
    let payload = json!({
        "model": config::AI_MODEL,
        "prompt": prompt,
        "stream": true
    });
    
    // Send the request to Ollama API
    let response = client
        .post(config::OLLAMA_API_URL)
        .json(&payload)
        .send()
        .await?;
    
    // Process the response
    if response.status().is_success() {
        let response_text = response.text().await?;
        let mut full_response = String::new();
        
        // Handle streaming response format
        println!("\n🤖 ZorpAI: ");
        let lines: Vec<&str> = response_text.split('\n').collect();
        for line in lines {
            if !line.is_empty() {
                if let Ok(json_line) = serde_json::from_str::<Value>(line) {
                    if let Some(response_part) = json_line.get("response").and_then(Value::as_str) {
                        // Clean up the response - remove any "User:" prefixes or instructions
                        if !response_part.trim().starts_with("User:") {
                            print!("{}", response_part);
                            io::stdout().flush()?;
                            full_response.push_str(response_part);
                        }
                    }
                }
            }
        }
        println!("\n"); // Add a newline at the end
        
        // Add to history
        history.push_back((message.to_string(), full_response));
        
        // Keep history to a reasonable size
        if history.len() > config::MAX_HISTORY_SIZE {
            history.pop_front();
        }
    } else {
        println!("Failed to get AI response: HTTP {}", response.status());
    }
    
    Ok(())
}

// Future AI module enhancements could include:
// - Support for different AI providers
// - Caching of responses
// - Offline mode with local models
// - Specialized AI commands for different tasks
