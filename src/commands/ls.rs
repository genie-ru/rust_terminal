//! lsコマンドの簡易実装。ディレクトリの内容を表示する

use std::fs;

/// コマンド名
pub const NAME: &str = "ls";

/// 引数省略時はカレントディレクトリを表示する
pub fn run(args: &[&str]) {
    let dir = if args.is_empty() { "." } else { args[0] };

    match fs::read_dir(dir) {
        Ok(entries) => {
            let mut files: Vec<String> = Vec::new();
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if entry.path().is_dir() {
                        files.push(format!("{}/", name));
                    } else {
                        files.push(name.to_string());
                    }
                }
            }

            files.sort();

            // 20文字幅・4項目ごとに改行して表示する
            let mut count = 0;
            for file in files {
                print!("{:<20}", file);
                count += 1;
                if count % 4 == 0 {
                    println!();
                }
            }
            if count % 4 != 0 {
                println!();
            }
        }
        Err(e) => {
            eprintln!("ls: cannot access '{}': {}", dir, e);
        }
    }
}
