// Configuration module - Central place for application constants and settings

// History file location
pub const HISTORY_FILE: &str = "~/.zorpsh_history";

// AI configuration
pub const MAX_HISTORY_SIZE: usize = 10;
pub const AI_MODEL: &str = "tinyllama";
pub const OLLAMA_API_URL: &str = "http://localhost:11434/api/generate";

// Future config enhancements could include:
// - Loading settings from a config file
// - Environment variable overrides
// - User preferences
// - Theme settings
