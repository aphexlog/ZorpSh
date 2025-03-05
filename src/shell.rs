// Shell module - Handles execution of system commands

use std::process::Command;

/// Executes a shell command and handles the result
/// 
/// # Arguments
/// * `input` - The full command string to execute
///
/// # Example
/// ```
/// execute_command("ls -la");
/// ```
pub fn execute_command(input: &str) {
    // Split the input into command and arguments
    let mut parts = input.split_whitespace();
    
    // The first part is the command
    if let Some(command) = parts.next() {
        // Collect the remaining parts as arguments
        let args: Vec<&str> = parts.collect();

        // Spawn a new process for the command
        let status = Command::new(command)
            .args(&args)
            .spawn()
            .and_then(|mut child| child.wait());

        // Handle the result
        match status {
            Ok(status) if !status.success() => println!("Process exited with: {}", status),
            Ok(_) => {}, // Don't print anything for successful commands
            Err(e) => println!("Zorp error: {}", e),
        }
    }
}

// Future shell module enhancements could include:
// - Command history management
// - Command autocompletion
// - Built-in shell commands that don't require spawning a process
// - Environment variable management
