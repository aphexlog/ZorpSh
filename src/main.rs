// Main entry point for ZorpSh - An Intergalactic Command Line
// This file orchestrates the different components but delegates specific
// functionality to specialized modules

// Function to execute a chain of commands with && and || operators
fn execute_command_chain(input: &str) {
    // Split the input by && and || operators while preserving the operators
    let mut commands = Vec::new();
    let mut operators = Vec::new();
    
    // Simple parsing to handle && and || operators
    let mut current_cmd = String::new();
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '&' && chars.peek() == Some(&'&') {
            // Found &&
            chars.next(); // consume the second &
            if !current_cmd.trim().is_empty() {
                commands.push(current_cmd.trim().to_string());
                current_cmd = String::new();
                operators.push("&&");
            }
        } else if c == '|' && chars.peek() == Some(&'|') {
            // Found ||
            chars.next(); // consume the second |
            if !current_cmd.trim().is_empty() {
                commands.push(current_cmd.trim().to_string());
                current_cmd = String::new();
                operators.push("||");
            }
        } else {
            current_cmd.push(c);
        }
    }
    
    // Add the last command if there is one
    if !current_cmd.trim().is_empty() {
        commands.push(current_cmd.trim().to_string());
    }
    
    // Execute the commands in sequence based on the operators
    if commands.is_empty() {
        return;
    }
    
    // Execute the first command
    let mut last_success = shell::execute_command(&commands[0]);
    
    // Execute subsequent commands based on the operators
    for i in 1..commands.len() {
        let operator = operators[i-1];
        
        match operator {
            "&&" => {
                // Only execute if the previous command succeeded
                if last_success {
                    last_success = shell::execute_command(&commands[i]);
                }
            },
            "||" => {
                // Only execute if the previous command failed
                if !last_success {
                    last_success = shell::execute_command(&commands[i]);
                }
            },
            _ => unreachable!()
        }
    }
}

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
    let history_path = config::history_file_path();
    let _ = rl.load_history(&history_path);
    
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
                    let _ = rl.save_history(&config::history_file_path());
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
                    // Execute as shell command, possibly with chaining
                    execute_command_chain(input);
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
    let _ = rl.save_history(&config::history_file_path());
    
    Ok(())
}
