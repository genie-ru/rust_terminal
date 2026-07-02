//! mkdirコマンド。指定したディレクトリを作成する

use std::fs;
use std::path::Path;

/// コマンド名
pub const NAME: &str = "mkdir";

pub fn run(args: &[&str]) {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
        eprintln!("Try 'mkdir --help' for more information.");
        return;
    }

    for dir in args {
        let path = Path::new(dir);

        if path.exists() {
            eprintln!("mkdir: cannot create directory '{}': File exists", dir);
            continue;
        }

        if let Err(e) = fs::create_dir(path) {
            eprintln!("mkdir: cannot create directory '{}': {}", dir, e);
        }
    }
}
