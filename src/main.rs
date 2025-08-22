use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

fn main() {
    println!("Simple Terminal - Type 'exit' or 'quit' to exit");
    println!("Tip: Press Tab for directory completion with 'cd' command");
    
    loop {
        print_prompt();
        
        // ユーザー入力を読み取る
        let input = match read_input() {
            Some(line) => line,
            None => {
                // EOF (Ctrl+D) が検出されたら終了
                println!("\nさようなら!");
                break;
            }
        };
        
        if input.is_empty() {
            continue;
        }
        
        // 入力をスペースで分割してコマンドと引数に分ける
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        
        let command = parts[0];
        let args = &parts[1..];
        
        // コマンドに応じた処理を実行
        match command {
            "exit" | "quit" => {
                println!("さようなら!");
                break;
            }
            "cd" => {
                handle_cd_with_completion(args);
            }
            "pwd" => {
                print_working_directory();
            }
            "clear" => {
                clear_screen();
            }
            "help" => {
                print_help();
            }
            _ => {
                // 外部コマンドとして実行
                execute_external_command(command, args);
            }
        }
    }
}

/// プロンプトを表示する（現在のディレクトリ名を表示）
fn print_prompt() {
    let current_dir = env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("?"));
    
    let dir_name = current_dir
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("?");
    
    print!("{}> ", dir_name);
    let _ = io::stdout().flush();
}

/// 標準入力から1行読み取る
fn read_input() -> Option<String> {
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

/// cdコマンドをTab補完付きで処理する
fn handle_cd_with_completion(args: &[&str]) {
    if args.is_empty() {
        // 引数なしの場合はホームディレクトリに移動
        change_directory(&[]);
        return;
    }
    
    let path = args[0];
    
    // Tabキーのシミュレーション（実際の実装では、ここで補完候補を表示）
    if path.ends_with('\t') {
        let path_without_tab = path.trim_end_matches('\t');
        if let Some(completions) = get_path_completions(path_without_tab) {
            if completions.len() == 1 {
                // 1つだけマッチした場合は自動補完
                change_directory(&[&completions[0]]);
            } else if !completions.is_empty() {
                // 複数マッチの場合は候補を表示
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

/// パス補完候補を取得する
fn get_path_completions(partial: &str) -> Option<Vec<String>> {
    let path = Path::new(partial);
    
    // ディレクトリ部分とファイル名部分に分ける
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
    
    // ディレクトリの内容を読み取る
    let entries = fs::read_dir(dir_path).ok()?;
    
    // プレフィックスにマッチするエントリを探す
    let mut matches: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        if let Some(name) = entry.file_name().to_str() {
            if file_prefix.is_empty() || name.starts_with(file_prefix) {
                // パスを構築
                let mut completion = if dir_path == "." {
                    name.to_string()
                } else if partial.ends_with('/') {
                    format!("{}{}", partial, name)
                } else {
                    format!("{}/{}", dir_path, name)
                };
                
                // ディレクトリの場合は/を追加
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

/// カレントディレクトリを変更する
fn change_directory(args: &[&str]) {
    let new_dir = if args.is_empty() {
        // 引数なしの場合はホームディレクトリに移動
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    } else {
        args[0].to_string()
    };
    
    let path = Path::new(&new_dir);
    if let Err(e) = env::set_current_dir(&path) {
        eprintln!("cd: {}: {}", new_dir, e);
    }
}

/// 現在の作業ディレクトリを表示する
fn print_working_directory() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("pwd: {}", e),
    }
}

/// 画面をクリアする
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

/// 外部コマンドを実行する
fn execute_external_command(command: &str, args: &[&str]) {
    let child = Command::new(command)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();
    
    match child {
        Ok(mut process) => {
            // プロセスの終了を待つ
            match process.wait() {
                Ok(status) => {
                    if !status.success() {
                        if let Some(code) = status.code() {
                            eprintln!("Command exited with status: {}", code);
                        }
                    }
                }
                Err(e) => eprintln!("Failed to wait for command: {}", e),
            }
        }
        Err(e) => {
            eprintln!("{}: command not found ({})", command, e);
        }
    }
}

/// ヘルプメッセージを表示する
fn print_help() {
    println!("Available commands:");
    println!("  cd [dir]  - Change directory");
    println!("  pwd       - Print working directory");
    println!("  clear     - Clear screen");
    println!("  help      - Show this help message");
    println!("  exit/quit - Exit the terminal");
    println!("  [other]   - Execute external command");
    println!("\nTips:");
    println!("  - Use Ctrl+C to interrupt running commands");
    println!("  - Use Ctrl+D on empty line to exit");
}