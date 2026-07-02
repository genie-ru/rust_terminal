//! シンプルなターミナルエミュレータのエントリーポイント
//!
//! 各コマンドの実装はモジュールに分割している:
//! - shell:    プロンプト表示・入力読み取り
//! - commands: 各種ビルトイン／外部コマンド

mod commands;
mod shell;

use commands::{cd, clear, external, help, ls, mkdir, pwd, rm, rmdir};

fn main() {
    println!("Simple Terminal - Type 'exit' or 'quit' to exit");
    println!("Tip: Type 'help' to see available commands");

    loop {
        shell::print_prompt();

        // None は EOF (Ctrl+D)
        let input = match shell::read_input() {
            Some(line) => line,
            None => {
                println!("\nさようなら!");
                break;
            }
        };

        if input.is_empty() {
            continue;
        }

        // "rm -f file.txt" -> ["rm", "-f", "file.txt"]
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let command = parts[0];
        let args = &parts[1..];

        match command {
            "exit" | "quit" => {
                println!("さようなら!");
                break;
            }
            "cd" => cd::run(args),
            "pwd" => pwd::run(),
            "clear" => clear::run(),
            "help" => help::run(),
            "rm" => rm::run(args),
            "rmdir" => rmdir::run(args),
            "mkdir" => mkdir::run(args),
            "ls" => ls::run(args),
            // ビルトイン以外は外部コマンドとして実行
            _ => external::execute_external_command(command, args),
        }
    }
}
