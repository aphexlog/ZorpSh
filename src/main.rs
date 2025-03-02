use std::io::{self, Write};
use std::process::Command;
use reqwest::Client;
use serde_json::{json, Value};
use std::collections::VecDeque;


fn print_logo() {
    println!(r#"
    ███████╗ ██████╗ ██████╗ ██████╗ ███████╗██╗  ██╗
    ╚══███╔╝██╔═══██╗██╔══██╗██╔══██╗██╔════╝██║  ██║
      ███╔╝ ██║   ██║██████╔╝██████╔╝███████╗███████║
     ███╔╝  ██║   ██║██╔══██╗██╔═══╝ ╚════██║██╔══██║
    ███████╗╚██████╔╝██║  ██║██║     ███████║██║  ██║
    ╚══════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚══════╝╚═╝  ╚═╝
    
    Welcome to ZorpSh - Your Intergalactic Command Line!
    "#);
}

    
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_logo();
    
    // Initialize chat history
    let mut chat_history: VecDeque<(String, String)> = VecDeque::with_capacity(10);
    
    loop {
        print!("ZorpSh> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        } else if input == "exit" {
            println!("Goodbye, Zorp!");
            break;
        } else if input.starts_with("chat") {
            // Extract the message part after "chat"
            let message = input.trim_start_matches("chat").trim();
            if !message.is_empty() {
                // Send message to AI assistant
                if let Err(e) = chat_with_ai(message, &mut chat_history).await {
                    println!("AI chat unavailable: {}", e);
                }
            } else {
                println!("Usage: chat <your message>");
            }
            continue;
        } else {
            // Execute shell command
            let mut parts = input.split_whitespace();
            let command = parts.next().unwrap();
            let args: Vec<&str> = parts.collect();

            let status = Command::new(command)
                .args(&args)
                .spawn()
                .and_then(|mut child| child.wait());

            match status {
                Ok(status) if !status.success() => println!("Process exited with: {}", status),
                Ok(_) => {}, // Don't print anything for successful commands
                Err(e) => println!("Zorp error: {}", e),
            }
        }
    }
    
    Ok(())
}

/// Chat with AI assistant and maintain conversation history
async fn chat_with_ai(message: &str, history: &mut VecDeque<(String, String)>) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize HTTP client
    let client = Client::new();
    
    // Build context from history
    let mut context = String::new();
    for (user_msg, ai_msg) in history.iter() {
        context.push_str(&format!("User: {}\nAssistant: {}\n", user_msg, ai_msg));
    }
    
    // Create the prompt with context
    let prompt = if context.is_empty() {
        format!("User: {}\nAssistant:", message)
    } else {
        format!("{}\nUser: {}\nAssistant:", context, message)
    };
    
    println!("\n🤖 ZorpAI is thinking...");
    
    // Create the request payload
    let payload = json!({
        "model": "tinyllama",
        "prompt": prompt,
        "stream": true
    });
    
    // Send the request to Ollama API
    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&payload)
        .send()
        .await?;
    
    // Check if the request was successful
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
                        print!("{}", response_part);
                        io::stdout().flush()?;
                        full_response.push_str(response_part);
                    }
                }
            }
        }
        println!("\n"); // Add a newline at the end
        
        // Add to history
        history.push_back((message.to_string(), full_response));
        
        // Keep history to a reasonable size
        if history.len() > 10 {
            history.pop_front();
        }
    } else {
        println!("Failed to get AI response: HTTP {}", response.status());
    }
    
    Ok(())
}
