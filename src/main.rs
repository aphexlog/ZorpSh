// Main entry point for ZorpSh - An Intergalactic Command Line
// This file orchestrates the different components but delegates specific
// functionality to specialized modules

// Import our custom modules
mod ui;
mod shell;
mod ai;
mod config;

// Re-export types that are used in the main module
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::collections::VecDeque;

// The main function is marked with tokio::main to enable async/await syntax
// at the top level of our application
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Display the welcome logo
    ui::print_logo();
    
    // Initialize chat history with a fixed capacity to prevent unbounded memory growth
    let mut chat_history: VecDeque<(String, String)> = VecDeque::with_capacity(config::MAX_HISTORY_SIZE);
    
    // Initialize the line editor for command input with history support
    let mut rl = DefaultEditor::new()?;
    
    // Load command history from disk if available
    let _ = rl.load_history(&config::HISTORY_FILE);
    
    // Main input loop
    loop {
        // Prompt for user input
        let readline = rl.readline("ZorpSh> ");
        
        match readline {
            Ok(line) => {
                let input = line.trim();
                
                // Add non-empty commands to history
                if !input.is_empty() {
                    let _ = rl.add_history_entry(input);
                }

                // Process the command
                if input.is_empty() {
                    continue;
                } else if input == "exit" {
                    println!("Goodbye, Zorp!");
                    // Save history before exiting
                    let _ = rl.save_history(&config::HISTORY_FILE);
                    break;
                } else if input.starts_with("chat") {
                    // Handle AI chat command
                    let message = input.trim_start_matches("chat").trim();
                    if !message.is_empty() {
                        if let Err(e) = ai::chat_with_ai(message, &mut chat_history).await {
                            println!("AI chat unavailable: {}", e);
                        }
                    } else {
                        println!("Usage: chat <your message>");
                    }
                } else {
                    // Execute as shell command
                    shell::execute_command(input);
                }
            },
            // Handle various exit conditions
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C pressed, exiting...");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    
    // Save history when exiting normally
    let _ = rl.save_history(&config::HISTORY_FILE);
    
    Ok(())
}
