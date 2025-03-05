// Configuration module - Central place for application constants and settings

// History file location - Using a function to expand the home directory at runtime
pub fn history_file_path() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!("{}/{}", home, ".zorpsh_history")
}

// AI configuration
pub const MAX_HISTORY_SIZE: usize = 10;
pub const AI_MODEL: &str = "tinyllama";
pub const OLLAMA_API_URL: &str = "http://localhost:11434/api/generate";

// Future config enhancements could include:
// - Loading settings from a config file
// - Environment variable overrides
// - User preferences
// - Theme settings
