# ZorpSh

A lightweight interactive shell with an intergalactic theme.

## About

ZorpSh is a custom REPL (Read-Eval-Print Loop) shell that provides command-line functionality with a unique space-themed interface. It aims to be responsive and fun to use while maintaining a clean, distinctive aesthetic.

## Features

- Custom command parsing and execution
- Command history and navigation
- Distinctive space-themed interface
- Lightweight and fast operation

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
ZorpSh> exit                   # Exit the shell
```

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
