//! 対話シェルの入出力を担当するモジュール
//! プロンプト表示と標準入力の読み取りを提供する

use std::env;
use std::io::{self, Write};

/// "ディレクトリ名> " の形式でプロンプトを表示する
pub fn print_prompt() {
    let current_dir = env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("?"));

    let dir_name = current_dir
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("?");

    print!("{}> ", dir_name);
    let _ = io::stdout().flush();
}

/// 標準入力から1行読み取る。EOF (Ctrl+D) の場合は None を返す
pub fn read_input() -> Option<String> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(0) => None, // EOF
        Ok(_) => Some(input.trim().to_string()),
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            None
        }
    }
}
