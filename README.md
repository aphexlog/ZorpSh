# ZorpSh

A lightweight interactive shell with an intergalactic theme and built-in AI assistant capabilities.

## About

ZorpSh is a completely free AI assistant-enabled shell alternative to zsh. This custom REPL (Read-Eval-Print Loop) shell provides command-line functionality with a unique space-themed interface. It aims to be responsive and fun to use while maintaining a clean, distinctive aesthetic, while giving you the power of AI assistance right in your terminal.

## Features

- Built-in AI assistant for command help and natural language interactions
- Custom command parsing and execution
- Command history and navigation
- Distinctive space-themed interface
- Lightweight and fast operation
- Free alternative to traditional shells like zsh with modern AI capabilities

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) - Install with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### From crates.io (Recommended)

ZorpSh is now available as an official package on crates.io:

```bash
# Install directly from crates.io
cargo install zorpsh

# After installation, simply run:
zorpsh
```

### From Source

```bash
# Clone the repository
git clone https://github.com/aphexlog/ZorpSh.git
cd ZorpSh

# Build and run
cargo run

# Or install from local source
cargo install --path .
```

## Usage

Once running, ZorpSh provides a prompt where you can enter commands:

```
ZorpSh> help                   # Display available commands
ZorpSh> cd /some/directory     # Change directory
ZorpSh> ls                     # List files
ZorpSh> chat                   # Start AI assistant conversation
ZorpSh> exit                   # Exit the shell
```

### AI Assistant

To use the built-in AI assistant:
1. Make sure you have an Ollama server running with a model loaded
2. Type `chat` followed by your question or command

```
ZorpSh> chat How do I list all files including hidden ones?
```

Note: The chat functionality requires an active Ollama server. Automation to simplify this setup process with a single command is coming soon.

ZorpSh supports standard shell commands and maintains command history between sessions.

## Development

```bash
# Run tests
cargo test

# Build in release mode
cargo build --release
```

## License

This project is licensed under the [MIT License](LICENSE).
