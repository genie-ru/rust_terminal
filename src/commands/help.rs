//! helpコマンド。使用可能なコマンド一覧を表示する

pub fn run() {
    println!("=== Simple Terminal - Available Commands ===\n");

    println!("File and Directory Operations:");
    println!("  ls [dir]      - List directory contents");
    println!("  cd [dir]      - Change directory");
    println!("  pwd           - Print working directory");
    println!("  mkdir <dir>   - Create directory");
    println!("  rmdir <dir>   - Remove empty directory");
    println!("  rm <file>     - Remove file");
    println!("    -f          - Force removal (ignore errors)");
    println!("    -r, -R      - Remove directories and their contents recursively");

    println!("\nTerminal Control:");
    println!("  clear         - Clear screen");
    println!("  help          - Show this help message");
    println!("  exit/quit     - Exit the terminal");

    println!("\nOther Commands:");
    println!("  [command]     - Execute as external command");

    println!("\nShortcuts:");
    println!("  Ctrl+C        - Interrupt running command");
    println!("  Ctrl+D        - Exit on empty line");

    println!("\nExamples:");
    println!("  rm file.txt           - Remove a file");
    println!("  rm -rf directory/     - Remove a directory and all its contents");
    println!("  mkdir new_folder      - Create a new directory");
    println!("  rmdir old_folder      - Remove an empty directory");
}
