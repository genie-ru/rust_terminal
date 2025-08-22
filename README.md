# Taminal - Simple Terminal Emulator

A simple terminal emulator written in Rust.

## Features

- Execute basic shell commands
- Built-in commands (cd, pwd, clear, help, exit/quit)
- Execute external commands
- Display current directory name in prompt
- Error handling

## Installation

```bash
git clone <repository>
cd taminal
cargo build --release
```

## Usage

```bash
cargo run
```

Or run the built binary directly:

```bash
./target/release/simple_shell
```

## Available Commands

| Command | Description |
|---------|-------------|
| `cd [dir]` | Change directory |
| `pwd` | Print working directory |
| `clear` | Clear screen |
| `help` | Show help message |
| `exit` / `quit` | Exit the terminal |
| Others | Execute as external command |

## Shortcuts

- `Ctrl+C` - Interrupt running command
- `Ctrl+D` - Exit on empty line

## Requirements

- Rust 1.70.0 or higher

## License

MIT