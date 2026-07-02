//! pwdコマンド。現在の作業ディレクトリを表示する

use std::env;

pub fn run() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("pwd: {}", e),
    }
}
