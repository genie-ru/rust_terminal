# Taminal - Simple Terminal Emulator

A feature-rich terminal emulator written in Rust with built-in file management commands.

## Features

- Execute basic shell commands
- Built-in file and directory operations
- Execute external commands
- Display current directory name in prompt
- Comprehensive error handling
- Directory completion hints for cd command

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

### File and Directory Operations

| Command | Description | Example |
|---------|-------------|---------|
| `ls [dir]` | List directory contents | `ls`, `ls src/` |
| `cd [dir]` | Change directory | `cd src`, `cd ..`, `cd` (home) |
| `pwd` | Print working directory | `pwd` |
| `mkdir <dir>` | Create directory | `mkdir new_folder` |
| `rmdir <dir>` | Remove empty directory | `rmdir old_folder` |
| `rm <file>` | Remove file or directory | `rm file.txt` |
| `rm -f <file>` | Force removal (ignore errors) | `rm -f temp.txt` |
| `rm -r <dir>` | Remove directory recursively | `rm -r folder/` |
| `rm -rf <dir>` | Force recursive removal | `rm -rf build/` |

### Terminal Control

| Command | Description |
|---------|-------------|
| `clear` | Clear screen |
| `help` | Show help message |
| `exit` / `quit` | Exit the terminal |

### External Commands

Any command not listed above will be executed as an external command (e.g., `echo`, `cat`, `grep`, etc.)

## Shortcuts

- `Ctrl+C` - Interrupt running command
- `Ctrl+D` - Exit on empty line

## Examples

```bash
# Create a new directory
mkdir my_project

# Navigate into it
cd my_project

# Create some subdirectories
mkdir src
mkdir docs

# List contents
ls

# Remove a file
rm unwanted.txt

# Remove a directory and all its contents
rm -rf old_project/

# Go back to parent directory
cd ..

# Remove empty directory
rmdir empty_folder
```

## Requirements

- Rust 1.70.0 or higher

## License

MIT