// Shell module - Handles execution of system commands

use std::env;
use std::path::PathBuf;
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
        // Handle built-in commands
        if command == "cd" {
            // Collect the remaining parts as arguments
            let args: Vec<&str> = parts.collect();
            change_directory(&args);
            return;
        }
        
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

/// Changes the current working directory
///
/// # Arguments
/// * `args` - Arguments to the cd command
///
/// Supports:
/// - cd with no args (go to home directory)
/// - cd ~ (go to home directory)
/// - cd .. (go up one directory)
/// - cd $HOME or other env vars (expands environment variables)
fn change_directory(args: &[&str]) {
    let target_dir = if args.is_empty() {
        // cd with no args goes to home directory
        match home::home_dir() {
            Some(path) => path,
            None => {
                println!("Zorp error: Could not determine home directory");
                return;
            }
        }
    } else {
        let dir = args[0];
        
        if dir == "~" {
            // cd ~ goes to home directory
            match home::home_dir() {
                Some(path) => path,
                None => {
                    println!("Zorp error: Could not determine home directory");
                    return;
                }
            }
        } else if dir == ".." {
            // cd .. goes up one directory
            match env::current_dir() {
                Ok(path) => {
                    match path.parent() {
                        Some(parent) => parent.to_path_buf(),
                        None => {
                            println!("Zorp error: Already at root directory");
                            return;
                        }
                    }
                },
                Err(e) => {
                    println!("Zorp error: Failed to get current directory: {}", e);
                    return;
                }
            }
        } else if dir.starts_with("$") {
            // Handle environment variables like $HOME
            let env_var = &dir[1..];
            match env::var(env_var) {
                Ok(value) => PathBuf::from(value),
                Err(_) => {
                    println!("Zorp error: Environment variable {} not found", env_var);
                    return;
                }
            }
        } else {
            // Handle regular paths (absolute or relative)
            PathBuf::from(dir)
        }
    };

    // Attempt to change directory
    if let Err(e) = env::set_current_dir(&target_dir) {
        println!("Zorp error: Failed to change directory: {}", e);
    }
}

// Future shell module enhancements could include:
// - Command history management
// - Command autocompletion
// - More built-in shell commands
// - Environment variable management
