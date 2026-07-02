//! rmdirコマンド。空のディレクトリのみ削除できる

use std::fs;
use std::path::Path;

/// コマンド名
pub const NAME: &str = "rmdir";

pub fn run(args: &[&str]) {
    if args.is_empty() {
        eprintln!("rmdir: missing operand");
        eprintln!("Try 'rmdir --help' for more information.");
        return;
    }

    for dir in args {
        let path = Path::new(dir);

        if !path.exists() {
            eprintln!("rmdir: failed to remove '{}': No such file or directory", dir);
            continue;
        }

        if !path.is_dir() {
            eprintln!("rmdir: failed to remove '{}': Not a directory", dir);
            continue;
        }

        match fs::remove_dir(path) {
            Ok(_) => {}
            Err(e) => {
                if e.kind() == std::io::ErrorKind::Other {
                    eprintln!("rmdir: failed to remove '{}': Directory not empty", dir);
                } else {
                    eprintln!("rmdir: failed to remove '{}': {}", dir, e);
                }
            }
        }
    }
}
