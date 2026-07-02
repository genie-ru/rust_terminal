//! シンプルなターミナルエミュレータのエントリーポイント
//!
//! 各コマンドの実装はモジュールに分割している:
//! - shell:    プロンプト表示・入力読み取り
//! - commands: 各ビルトインコマンド（外部コマンドは実行しない）

mod commands;
mod shell;

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

        // 終了だけはループ制御が絡むためここで処理する
        if command == "exit" || command == "quit" {
            println!("さようなら!");
            break;
        }

        // それ以外はコマンドテーブルから名前で引いて実行する
        if !commands::dispatch(command, args) {
            eprintln!("{}: command not found", command);
            eprintln!("Type 'help' to see available commands");
        }
    }
}
