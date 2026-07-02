//! cdコマンド。ディレクトリ移動（Tab補完の簡易実装付き）

use std::env;
use std::fs;
use std::path::Path;

/// cdコマンドを処理する。引数なしならホームディレクトリへ移動する。
///
/// 末尾に `\t` を含む場合は補完候補を表示する簡易実装。
/// 実際のTabキー入力は行単位読み取りのため検知できない。
pub fn run(args: &[&str]) {
    if args.is_empty() {
        change_directory(&[]);
        return;
    }

    let path = args[0];

    if path.ends_with('\t') {
        let path_without_tab = path.trim_end_matches('\t');
        if let Some(completions) = get_path_completions(path_without_tab) {
            if completions.len() == 1 {
                // 候補が1つなら自動補完して移動
                change_directory(&[&completions[0]]);
            } else if !completions.is_empty() {
                println!("Possible completions:");
                for comp in &completions {
                    println!("  {}", comp);
                }
            }
        }
    } else {
        change_directory(args);
    }
}

/// 部分パス文字列にマッチする補完候補を返す。
/// 例: "src/ma" -> ["src/main.rs", ...]
fn get_path_completions(partial: &str) -> Option<Vec<String>> {
    let path = Path::new(partial);

    // ディレクトリ部分とファイル名プレフィックスに分割する
    let (dir_path, file_prefix) = if partial.ends_with('/') {
        (partial, "")
    } else if partial.is_empty() {
        (".", "")
    } else {
        let parent = path.parent().unwrap_or(Path::new("."));
        let file_name = path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        (parent.to_str().unwrap_or("."), file_name)
    };

    let entries = fs::read_dir(dir_path).ok()?;

    let mut matches: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        if let Some(name) = entry.file_name().to_str() {
            if file_prefix.is_empty() || name.starts_with(file_prefix) {
                let mut completion = if dir_path == "." {
                    name.to_string()
                } else if partial.ends_with('/') {
                    format!("{}{}", partial, name)
                } else {
                    format!("{}/{}", dir_path, name)
                };

                // ディレクトリは末尾に / を付ける
                if entry.path().is_dir() {
                    completion.push('/');
                }
                matches.push(completion);
            }
        }
    }

    if matches.is_empty() {
        None
    } else {
        matches.sort();
        Some(matches)
    }
}

/// カレントディレクトリを変更する。引数が空ならホーム（未設定なら /）へ。
fn change_directory(args: &[&str]) {
    let new_dir = if args.is_empty() {
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    } else {
        args[0].to_string()
    };

    let path = Path::new(&new_dir);
    if let Err(e) = env::set_current_dir(&path) {
        eprintln!("cd: {}: {}", new_dir, e);
    }
}
