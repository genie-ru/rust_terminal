//! rmコマンド。オプション -f（強制）, -r/-R（再帰）に対応する

use std::fs;
use std::path::Path;

pub fn run(args: &[&str]) {
    if args.is_empty() {
        eprintln!("rm: missing operand");
        eprintln!("Try 'rm --help' for more information.");
        return;
    }

    let mut force = false;
    let mut recursive = false;
    let mut files = Vec::new();

    // 引数をオプションとファイルパスに振り分ける
    for arg in args {
        if arg.starts_with('-') {
            for ch in arg.chars().skip(1) {
                match ch {
                    'f' => force = true,
                    'r' | 'R' => recursive = true,
                    _ => {
                        eprintln!("rm: invalid option -- '{}'", ch);
                        return;
                    }
                }
            }
        } else {
            files.push(*arg);
        }
    }

    if files.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }

    for file in files {
        let path = Path::new(file);

        if !path.exists() {
            if !force {
                eprintln!("rm: cannot remove '{}': No such file or directory", file);
            }
            continue;
        }

        if path.is_dir() {
            if recursive {
                if let Err(e) = fs::remove_dir_all(path) {
                    if !force {
                        eprintln!("rm: cannot remove '{}': {}", file, e);
                    }
                }
            } else {
                eprintln!("rm: cannot remove '{}': Is a directory", file);
            }
        } else if let Err(e) = fs::remove_file(path) {
            if !force {
                eprintln!("rm: cannot remove '{}': {}", file, e);
            }
        }
    }
}
