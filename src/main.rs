use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

/// メイン関数 - ターミナルエミュレータのエントリーポイント
/// 無限ループでユーザーからのコマンド入力を待ち受け、
/// 適切なハンドラー関数を呼び出してコマンドを実行する
fn main() {
    // 起動時のウェルカムメッセージを表示
    println!("Simple Terminal - Type 'exit' or 'quit' to exit");
    println!("Tip: Type 'help' to see available commands");
    
    // メインループ - ユーザーが終了コマンドを入力するまで継続
    loop {
        // プロンプトを表示（現在のディレクトリ名を含む）
        print_prompt();
        
        // ユーザー入力を読み取る
        // None が返された場合は EOF (Ctrl+D) なので終了
        let input = match read_input() {
            Some(line) => line,
            None => {
                // EOF (Ctrl+D) が検出されたら終了メッセージを表示して終了
                println!("\nさようなら!");
                break;
            }
        };
        
        // 空の入力は無視して次のループへ
        if input.is_empty() {
            continue;
        }
        
        // 入力をスペースで分割してコマンドと引数に分ける
        // 例: "rm -f file.txt" -> ["rm", "-f", "file.txt"]
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        
        // 最初の要素をコマンド、残りを引数として取得
        let command = parts[0];
        let args = &parts[1..];
        
        // コマンドに応じた処理を実行
        // match文を使用してビルトインコマンドを処理
        match command {
            // 終了コマンド
            "exit" | "quit" => {
                println!("さようなら!");
                break;
            }
            // ディレクトリ変更コマンド
            "cd" => {
                handle_cd_with_completion(args);
            }
            // 現在のディレクトリ表示コマンド
            "pwd" => {
                print_working_directory();
            }
            // 画面クリアコマンド
            "clear" => {
                clear_screen();
            }
            // ヘルプ表示コマンド
            "help" => {
                print_help();
            }
            // ファイル削除コマンド
            "rm" => {
                remove_files(args);
            }
            // ディレクトリ削除コマンド
            "rmdir" => {
                remove_directories(args);
            }
            // ディレクトリ作成コマンド
            "mkdir" => {
                make_directories(args);
            }
            // ファイル一覧表示コマンド（簡易版）
            "ls" => {
                list_directory(args);
            }
            // その他のコマンドは外部コマンドとして実行
            _ => {
                execute_external_command(command, args);
            }
        }
    }
}

/// プロンプトを表示する関数
/// 現在のディレクトリ名を取得し、"ディレクトリ名> " の形式で表示
/// エラーが発生した場合は "?> " を表示
fn print_prompt() {
    // 現在のディレクトリを取得、失敗した場合は "?" を使用
    let current_dir = env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("?"));
    
    // ディレクトリ名のみを取得（フルパスではなく）
    // 例: /home/user/documents -> documents
    let dir_name = current_dir
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("?");
    
    // プロンプトを表示して即座にフラッシュ（バッファリングを防ぐ）
    print!("{}> ", dir_name);
    let _ = io::stdout().flush();
}

/// 標準入力から1行読み取る関数
/// 戻り値:
/// - Some(String): 読み取った文字列（改行を除く）
/// - None: EOF (Ctrl+D) が検出された場合
fn read_input() -> Option<String> {
    let mut input = String::new();
    
    // 標準入力から1行読み取る
    match io::stdin().read_line(&mut input) {
        Ok(0) => None, // EOF: 0バイト読み取った場合
        Ok(_) => Some(input.trim().to_string()), // 成功: 前後の空白を削除して返す
        Err(e) => {
            // エラー: エラーメッセージを表示してNoneを返す
            eprintln!("Error reading input: {}", e);
            None
        }
    }
}

/// cdコマンドをTab補完付きで処理する関数
/// 引数なしの場合はホームディレクトリに移動
/// Tab文字が含まれる場合は補完候補を表示（簡易実装）
fn handle_cd_with_completion(args: &[&str]) {
    if args.is_empty() {
        // 引数なしの場合はホームディレクトリに移動
        change_directory(&[]);
        return;
    }
    
    let path = args[0];
    
    // Tabキーのシミュレーション（実際の実装では、ここで補完候補を表示）
    // 注: 現在の実装では実際のTab入力は検出できない
    if path.ends_with('\t') {
        let path_without_tab = path.trim_end_matches('\t');
        if let Some(completions) = get_path_completions(path_without_tab) {
            if completions.len() == 1 {
                // 1つだけマッチした場合は自動補完して移動
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
        // 通常のcdコマンドとして実行
        change_directory(args);
    }
}

/// パス補完候補を取得する関数
/// 部分的なパス文字列から、マッチする可能性のあるパスのリストを返す
/// 
/// 引数:
/// - partial: 部分的なパス文字列
/// 
/// 戻り値:
/// - Some(Vec<String>): マッチしたパスのリスト
/// - None: マッチするパスがない場合
fn get_path_completions(partial: &str) -> Option<Vec<String>> {
    let path = Path::new(partial);
    
    // ディレクトリ部分とファイル名部分に分ける
    // 例: "src/ma" -> ("src", "ma")
    let (dir_path, file_prefix) = if partial.ends_with('/') {
        // 末尾が/の場合は、そのディレクトリ内の全エントリを対象とする
        (partial, "")
    } else if partial.is_empty() {
        // 空の場合はカレントディレクトリの全エントリ
        (".", "")
    } else {
        // それ以外は親ディレクトリとファイル名に分割
        let parent = path.parent().unwrap_or(Path::new("."));
        let file_name = path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        (parent.to_str().unwrap_or("."), file_name)
    };
    
    // ディレクトリの内容を読み取る
    let entries = fs::read_dir(dir_path).ok()?;
    
    // プレフィックスにマッチするエントリを収集
    let mut matches: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        if let Some(name) = entry.file_name().to_str() {
            // プレフィックスが空、またはマッチする場合
            if file_prefix.is_empty() || name.starts_with(file_prefix) {
                // パスを構築
                let mut completion = if dir_path == "." {
                    // カレントディレクトリの場合はファイル名のみ
                    name.to_string()
                } else if partial.ends_with('/') {
                    // 末尾が/の場合はそのまま結合
                    format!("{}{}", partial, name)
                } else {
                    // それ以外はディレクトリとファイル名を/で結合
                    format!("{}/{}", dir_path, name)
                };
                
                // ディレクトリの場合は末尾に/を追加
                if entry.path().is_dir() {
                    completion.push('/');
                }
                matches.push(completion);
            }
        }
    }
    
    // 結果を返す
    if matches.is_empty() {
        None
    } else {
        matches.sort(); // アルファベット順にソート
        Some(matches)
    }
}

/// カレントディレクトリを変更する関数
/// 
/// 引数:
/// - args: 移動先のディレクトリパス（空の場合はホームディレクトリ）
fn change_directory(args: &[&str]) {
    let new_dir = if args.is_empty() {
        // 引数なしの場合はホームディレクトリに移動
        // HOME環境変数が設定されていない場合は / に移動
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    } else {
        args[0].to_string()
    };
    
    // パスオブジェクトを作成してディレクトリ変更を試みる
    let path = Path::new(&new_dir);
    if let Err(e) = env::set_current_dir(&path) {
        // エラーが発生した場合はエラーメッセージを表示
        eprintln!("cd: {}: {}", new_dir, e);
    }
}

/// 現在の作業ディレクトリを表示する関数
/// pwdコマンドの実装
fn print_working_directory() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("pwd: {}", e),
    }
}

/// 画面をクリアする関数
/// ANSIエスケープシーケンスを使用して画面をクリアし、
/// カーソルを左上（1,1）に移動
fn clear_screen() {
    // \x1B[2J: 画面全体をクリア
    // \x1B[1;1H: カーソルを1行1列目に移動
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

/// ファイルを削除する関数
/// rmコマンドの実装
/// 
/// 引数:
/// - args: 削除するファイルのパス（複数可）
/// 
/// サポートするオプション:
/// - -f: 強制削除（エラーを無視）
/// - -r, -R: 再帰的削除（ディレクトリも削除）
fn remove_files(args: &[&str]) {
    if args.is_empty() {
        eprintln!("rm: missing operand");
        eprintln!("Try 'rm --help' for more information.");
        return;
    }
    
    // オプションフラグの初期化
    let mut force = false;  // -f: 強制削除
    let mut recursive = false;  // -r: 再帰的削除
    let mut files = Vec::new();  // 削除対象のファイルリスト
    
    // 引数を解析してオプションとファイルを分離
    for arg in args {
        if arg.starts_with('-') {
            // オプションの解析
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
            // ファイルパスとして追加
            files.push(*arg);
        }
    }
    
    // 削除対象がない場合はエラー
    if files.is_empty() {
        eprintln!("rm: missing operand");
        return;
    }
    
    // 各ファイルを削除
    for file in files {
        let path = Path::new(file);
        
        // パスが存在しない場合
        if !path.exists() {
            if !force {
                // -fオプションがない場合はエラーを表示
                eprintln!("rm: cannot remove '{}': No such file or directory", file);
            }
            continue;
        }
        
        // ディレクトリの場合
        if path.is_dir() {
            if recursive {
                // -rオプションがある場合は再帰的に削除
                match fs::remove_dir_all(path) {
                    Ok(_) => {
                        // 成功時は何も表示しない（Unix風）
                    }
                    Err(e) => {
                        if !force {
                            eprintln!("rm: cannot remove '{}': {}", file, e);
                        }
                    }
                }
            } else {
                // -rオプションがない場合はエラー
                eprintln!("rm: cannot remove '{}': Is a directory", file);
            }
        } else {
            // ファイルの削除
            match fs::remove_file(path) {
                Ok(_) => {
                    // 成功時は何も表示しない（Unix風）
                }
                Err(e) => {
                    if !force {
                        eprintln!("rm: cannot remove '{}': {}", file, e);
                    }
                }
            }
        }
    }
}

/// ディレクトリを削除する関数
/// rmdirコマンドの実装
/// 
/// 引数:
/// - args: 削除するディレクトリのパス（複数可）
/// 
/// 注意: 空のディレクトリのみ削除可能
fn remove_directories(args: &[&str]) {
    if args.is_empty() {
        eprintln!("rmdir: missing operand");
        eprintln!("Try 'rmdir --help' for more information.");
        return;
    }
    
    // 各ディレクトリを削除
    for dir in args {
        let path = Path::new(dir);
        
        // パスが存在しない場合
        if !path.exists() {
            eprintln!("rmdir: failed to remove '{}': No such file or directory", dir);
            continue;
        }
        
        // ディレクトリでない場合
        if !path.is_dir() {
            eprintln!("rmdir: failed to remove '{}': Not a directory", dir);
            continue;
        }
        
        // ディレクトリを削除（空でない場合は失敗する）
        match fs::remove_dir(path) {
            Ok(_) => {
                // 成功時は何も表示しない（Unix風）
            }
            Err(e) => {
                // エラーの種類に応じてメッセージを変更
                if e.kind() == std::io::ErrorKind::Other {
                    eprintln!("rmdir: failed to remove '{}': Directory not empty", dir);
                } else {
                    eprintln!("rmdir: failed to remove '{}': {}", dir, e);
                }
            }
        }
    }
}

/// ディレクトリを作成する関数
/// mkdirコマンドの実装
/// 
/// 引数:
/// - args: 作成するディレクトリのパス（複数可）
fn make_directories(args: &[&str]) {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
        eprintln!("Try 'mkdir --help' for more information.");
        return;
    }
    
    // 各ディレクトリを作成
    for dir in args {
        let path = Path::new(dir);
        
        // 既に存在する場合
        if path.exists() {
            eprintln!("mkdir: cannot create directory '{}': File exists", dir);
            continue;
        }
        
        // ディレクトリを作成
        match fs::create_dir(path) {
            Ok(_) => {
                // 成功時は何も表示しない（Unix風）
            }
            Err(e) => {
                eprintln!("mkdir: cannot create directory '{}': {}", dir, e);
            }
        }
    }
}

/// ディレクトリの内容を表示する関数
/// lsコマンドの簡易実装
/// 
/// 引数:
/// - args: 表示するディレクトリのパス（省略時はカレントディレクトリ）
fn list_directory(args: &[&str]) {
    // 対象ディレクトリを決定
    let dir = if args.is_empty() {
        "."
    } else {
        args[0]
    };
    
    // ディレクトリの内容を読み取る
    match fs::read_dir(dir) {
        Ok(entries) => {
            // エントリを収集してソート
            let mut files: Vec<String> = Vec::new();
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    // ディレクトリの場合は末尾に/を追加
                    if entry.path().is_dir() {
                        files.push(format!("{}/", name));
                    } else {
                        files.push(name.to_string());
                    }
                }
            }
            
            // アルファベット順にソート
            files.sort();
            
            // 表示（複数列で表示するための簡易実装）
            let mut count = 0;
            for file in files {
                print!("{:<20}", file); // 20文字幅で左寄せ
                count += 1;
                if count % 4 == 0 {
                    println!(); // 4項目ごとに改行
                }
            }
            if count % 4 != 0 {
                println!(); // 最後の行を改行
            }
        }
        Err(e) => {
            eprintln!("ls: cannot access '{}': {}", dir, e);
        }
    }
}

/// 外部コマンドを実行する関数
/// ビルトインコマンド以外のコマンドを子プロセスとして実行
/// 
/// 引数:
/// - command: 実行するコマンド名
/// - args: コマンドの引数
fn execute_external_command(command: &str, args: &[&str]) {
    // 子プロセスを生成してコマンドを実行
    let child = Command::new(command)
        .args(args)
        .stdin(Stdio::inherit())   // 標準入力を継承
        .stdout(Stdio::inherit())  // 標準出力を継承
        .stderr(Stdio::inherit())  // 標準エラー出力を継承
        .spawn();
    
    match child {
        Ok(mut process) => {
            // プロセスの終了を待つ
            match process.wait() {
                Ok(status) => {
                    // 異常終了の場合はステータスコードを表示
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
            // コマンドが見つからない、または実行できない場合
            eprintln!("{}: command not found ({})", command, e);
        }
    }
}

/// ヘルプメッセージを表示する関数
/// 使用可能なコマンドとその説明を表示
fn print_help() {
    println!("=== Simple Terminal - Available Commands ===\n");
    
    println!("File and Directory Operations:");
    println!("  ls [dir]      - List directory contents");
    println!("  cd [dir]      - Change directory");
    println!("  pwd           - Print working directory");
    println!("  mkdir <dir>   - Create directory");
    println!("  rmdir <dir>   - Remove empty directory");
    println!("  rm <file>     - Remove file");
    println!("    -f          - Force removal (ignore errors)");
    println!("    -r, -R      - Remove directories and their contents recursively");
    
    println!("\nTerminal Control:");
    println!("  clear         - Clear screen");
    println!("  help          - Show this help message");
    println!("  exit/quit     - Exit the terminal");
    
    println!("\nOther Commands:");
    println!("  [command]     - Execute as external command");
    
    println!("\nShortcuts:");
    println!("  Ctrl+C        - Interrupt running command");
    println!("  Ctrl+D        - Exit on empty line");
    
    println!("\nExamples:");
    println!("  rm file.txt           - Remove a file");
    println!("  rm -rf directory/     - Remove a directory and all its contents");
    println!("  mkdir new_folder      - Create a new directory");
    println!("  rmdir old_folder      - Remove an empty directory");
}