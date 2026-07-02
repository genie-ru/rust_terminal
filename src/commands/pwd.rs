//! pwdコマンド。現在の作業ディレクトリを表示する

use std::env;

/// コマンド名
pub const NAME: &str = "pwd";

pub fn run(_args: &[&str]) {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("pwd: {}", e),
    }
}
