use rustyline::completion::{Completer, Pair};
use rustyline::Context;
use std::fs;
use std::env;
use std::path::Path;

/// Custom Tab Completer for ZorpSh
pub struct ZorpCompleter;

impl Completer for ZorpCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let mut matches = Vec::new();

        // Handle directory completion for cd command
        if line.starts_with("cd ") {
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() > 1 {
                let dir_part = parts[1].trim();
                let start_pos = "cd ".len();
                
                // Determine the directory to look in
                let search_dir = if dir_part.starts_with('/') {
                    // Absolute path
                    if dir_part.contains('/') {
                        let parent = Path::new(dir_part).parent().unwrap_or(Path::new("/"));
                        parent.to_path_buf()
                    } else {
                        Path::new("/").to_path_buf()
                    }
                } else if dir_part.starts_with("~/") || dir_part == "~" {
                    // Home directory
                    if let Some(home) = home::home_dir() {
                        if dir_part.len() > 2 {
                            home.join(&dir_part[2..])
                        } else {
                            home
                        }
                    } else {
                        env::current_dir().unwrap_or_default()
                    }
                } else {
                    // Relative to current directory
                    if dir_part.contains('/') && !dir_part.ends_with('/') {
                        let parent = Path::new(dir_part).parent().unwrap_or(Path::new(""));
                        env::current_dir().unwrap_or_default().join(parent)
                    } else {
                        env::current_dir().unwrap_or_default()
                    }
                };
                
                // Get the prefix to match against
                let prefix = if dir_part.contains('/') && !dir_part.ends_with('/') {
                    Path::new(dir_part)
                        .file_name()
                        .map(|f| f.to_string_lossy().to_string())
                        .unwrap_or_default()
                } else {
                    String::new()
                };
                
                // Read directory entries
                if let Ok(entries) = fs::read_dir(search_dir) {
                    for entry in entries.flatten() {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_dir() {
                                if let Ok(file_name) = entry.file_name().into_string() {
                                    if file_name.starts_with(&prefix) {
                                        // For directory completion, add a trailing slash
                                        let replacement = if dir_part.contains('/') && !dir_part.ends_with('/') {
                                            let mut path = dir_part.to_string();
                                            if let Some(last_part) = Path::new(&path).file_name() {
                                                let last_part_str = last_part.to_string_lossy();
                                                path = path.trim_end_matches(&*last_part_str).to_string();
                                            }
                                            format!("{}{}/", path, file_name)
                                        } else {
                                            format!("{}/", file_name)
                                        };
                                        
                                        matches.push(Pair {
                                            display: format!("{}/", file_name),
                                            replacement,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                
                return Ok((start_pos, matches));
            }
        } else if line.starts_with("ls ") {
            // Directory completion for ls command
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() > 1 {
                let dir_part = parts[1].trim();
                let start_pos = "ls ".len();
                
                // Determine the directory to look in
                let search_dir = if dir_part.starts_with('/') {
                    // Absolute path
                    if dir_part.contains('/') {
                        let parent = Path::new(dir_part).parent().unwrap_or(Path::new("/"));
                        parent.to_path_buf()
                    } else {
                        Path::new("/").to_path_buf()
                    }
                } else if dir_part.starts_with("~/") || dir_part == "~" {
                    // Home directory
                    if let Some(home) = home::home_dir() {
                        if dir_part.len() > 2 {
                            home.join(&dir_part[2..])
                        } else {
                            home
                        }
                    } else {
                        env::current_dir().unwrap_or_default()
                    }
                } else {
                    // Relative to current directory
                    if dir_part.contains('/') && !dir_part.ends_with('/') {
                        let parent = Path::new(dir_part).parent().unwrap_or(Path::new(""));
                        env::current_dir().unwrap_or_default().join(parent)
                    } else {
                        env::current_dir().unwrap_or_default()
                    }
                };
                
                // Get the prefix to match against
                let prefix = if dir_part.contains('/') && !dir_part.ends_with('/') {
                    Path::new(dir_part)
                        .file_name()
                        .map(|f| f.to_string_lossy().to_string())
                        .unwrap_or_default()
                } else {
                    String::new()
                };
                
                // Read directory entries
                if let Ok(entries) = fs::read_dir(search_dir) {
                    for entry in entries.flatten() {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_dir() {
                                if let Ok(file_name) = entry.file_name().into_string() {
                                    if file_name.starts_with(&prefix) {
                                        // For directory completion, add a trailing slash
                                        let replacement = if dir_part.contains('/') && !dir_part.ends_with('/') {
                                            let mut path = dir_part.to_string();
                                            if let Some(last_part) = Path::new(&path).file_name() {
                                                let last_part_str = last_part.to_string_lossy();
                                                path = path.trim_end_matches(&*last_part_str).to_string();
                                            }
                                            format!("{}{}/", path, file_name)
                                        } else {
                                            format!("{}/", file_name)
                                        };
                                        
                                        matches.push(Pair {
                                            display: format!("{}/", file_name),
                                            replacement,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                
                return Ok((start_pos, matches));
            }
        } else if line.is_empty() || line.starts_with("./") {
            // Directory and file completion for current directory
            if let Ok(entries) = fs::read_dir(env::current_dir().unwrap_or_default()) {
                for entry in entries.flatten() {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        if let Ok(file_type) = entry.file_type() {
                            let display = if file_type.is_dir() {
                                format!("{}/", file_name)
                            } else {
                                file_name.clone()
                            };
                            
                            matches.push(Pair {
                                display,
                                replacement: file_name,
                            });
                        }
                    }
                }
            }
        } else {
            // Command completion from $PATH
            if let Ok(path) = env::var("PATH") {
                for dir in path.split(':') {
                    if let Ok(entries) = fs::read_dir(dir) {
                        for entry in entries.flatten() {
                            if let Ok(file_type) = entry.file_type() {
                                if file_type.is_file() {
                                    if let Ok(cmd) = entry.file_name().into_string() {
                                        if cmd.starts_with(line) {
                                            matches.push(Pair {
                                                display: cmd.clone(),
                                                replacement: cmd,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok((0, matches))
    }
}
