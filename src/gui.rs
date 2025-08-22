use eframe::egui;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

/// GUIターミナルアプリケーションの状態を管理する構造体
pub struct TerminalApp {
    /// コマンド入力フィールドの内容
    input: String,
    /// ターミナル出力の履歴（最大1000行保持）
    output: VecDeque<String>,
    /// 現在の作業ディレクトリ
    current_dir: String,
    /// コマンド履歴
    command_history: Vec<String>,
    /// 履歴のインデックス
    history_index: usize,
    /// オートスクロールの有効/無効
    auto_scroll: bool,
}

impl Default for TerminalApp {
    fn default() -> Self {
        let current_dir = env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("/"))
            .to_string_lossy()
            .to_string();
        
        let mut output = VecDeque::new();
        output.push_back("=== Taminal GUI Terminal ===".to_string());
        output.push_back("Type 'help' for available commands".to_string());
        output.push_back("".to_string());
        
        Self {
            input: String::new(),
            output,
            current_dir,
            command_history: Vec::new(),
            history_index: 0,
            auto_scroll: true,
        }
    }
}

impl TerminalApp {
    /// 新しいアプリケーションインスタンスを作成
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // カスタムフォントの設定（オプション）
        Self::default()
    }
    
    /// コマンドを実行する
    fn execute_command(&mut self, command: String) {
        // コマンドを履歴に追加
        if !command.is_empty() {
            self.command_history.push(command.clone());
            self.history_index = self.command_history.len();
        }
        
        // プロンプトとコマンドを出力に追加
        let prompt = format!("{}> {}", self.get_dir_name(), command);
        self.output.push_back(prompt);
        
        // 空白で分割してコマンドと引数を取得
        let parts: Vec<&str> = command.trim().split_whitespace().collect();
        if parts.is_empty() {
            return;
        }
        
        let cmd = parts[0];
        let args = &parts[1..];
        
        // ビルトインコマンドの処理
        match cmd {
            "exit" | "quit" => {
                self.output.push_back("Use the window close button to exit".to_string());
            }
            "clear" => {
                self.output.clear();
                self.output.push_back("=== Terminal Cleared ===".to_string());
            }
            "cd" => {
                self.change_directory(args);
            }
            "pwd" => {
                self.output.push_back(self.current_dir.clone());
            }
            "ls" => {
                self.list_directory(args);
            }
            "mkdir" => {
                self.make_directory(args);
            }
            "rmdir" => {
                self.remove_directory(args);
            }
            "rm" => {
                self.remove_file(args);
            }
            "help" => {
                self.show_help();
            }
            _ => {
                // 外部コマンドの実行
                self.execute_external_command(cmd, args);
            }
        }
        
        // 出力が1000行を超えたら古いものを削除
        while self.output.len() > 1000 {
            self.output.pop_front();
        }
    }
    
    /// ディレクトリを変更
    fn change_directory(&mut self, args: &[&str]) {
        let new_dir = if args.is_empty() {
            env::var("HOME").unwrap_or_else(|_| String::from("/"))
        } else {
            args[0].to_string()
        };
        
        let path = if Path::new(&new_dir).is_absolute() {
            Path::new(&new_dir).to_path_buf()
        } else {
            Path::new(&self.current_dir).join(&new_dir)
        };
        
        if path.exists() && path.is_dir() {
            match path.canonicalize() {
                Ok(canonical_path) => {
                    self.current_dir = canonical_path.to_string_lossy().to_string();
                    self.output.push_back(format!("Changed to: {}", self.current_dir));
                }
                Err(e) => {
                    self.output.push_back(format!("cd: {}: {}", new_dir, e));
                }
            }
        } else {
            self.output.push_back(format!("cd: {}: No such directory", new_dir));
        }
    }
    
    /// ディレクトリの内容を表示
    fn list_directory(&mut self, args: &[&str]) {
        let dir = if args.is_empty() {
            &self.current_dir
        } else {
            args[0]
        };
        
        let path = if Path::new(dir).is_absolute() {
            Path::new(dir).to_path_buf()
        } else {
            Path::new(&self.current_dir).join(dir)
        };
        
        match fs::read_dir(&path) {
            Ok(entries) => {
                let mut files = Vec::new();
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
                
                // 複数列で表示
                let mut line = String::new();
                for (i, file) in files.iter().enumerate() {
                    line.push_str(&format!("{:<20}", file));
                    if (i + 1) % 4 == 0 {
                        self.output.push_back(line.clone());
                        line.clear();
                    }
                }
                if !line.is_empty() {
                    self.output.push_back(line);
                }
            }
            Err(e) => {
                self.output.push_back(format!("ls: {}: {}", dir, e));
            }
        }
    }
    
    /// ディレクトリを作成
    fn make_directory(&mut self, args: &[&str]) {
        if args.is_empty() {
            self.output.push_back("mkdir: missing operand".to_string());
            return;
        }
        
        for dir in args {
            let path = if Path::new(dir).is_absolute() {
                Path::new(dir).to_path_buf()
            } else {
                Path::new(&self.current_dir).join(dir)
            };
            
            match fs::create_dir(&path) {
                Ok(_) => {
                    self.output.push_back(format!("Created directory: {}", dir));
                }
                Err(e) => {
                    self.output.push_back(format!("mkdir: {}: {}", dir, e));
                }
            }
        }
    }
    
    /// ディレクトリを削除
    fn remove_directory(&mut self, args: &[&str]) {
        if args.is_empty() {
            self.output.push_back("rmdir: missing operand".to_string());
            return;
        }
        
        for dir in args {
            let path = if Path::new(dir).is_absolute() {
                Path::new(dir).to_path_buf()
            } else {
                Path::new(&self.current_dir).join(dir)
            };
            
            match fs::remove_dir(&path) {
                Ok(_) => {
                    self.output.push_back(format!("Removed directory: {}", dir));
                }
                Err(e) => {
                    self.output.push_back(format!("rmdir: {}: {}", dir, e));
                }
            }
        }
    }
    
    /// ファイルを削除
    fn remove_file(&mut self, args: &[&str]) {
        if args.is_empty() {
            self.output.push_back("rm: missing operand".to_string());
            return;
        }
        
        let mut force = false;
        let mut recursive = false;
        let mut files = Vec::new();
        
        for arg in args {
            if arg.starts_with('-') {
                for ch in arg.chars().skip(1) {
                    match ch {
                        'f' => force = true,
                        'r' | 'R' => recursive = true,
                        _ => {}
                    }
                }
            } else {
                files.push(*arg);
            }
        }
        
        for file in files {
            let path = if Path::new(file).is_absolute() {
                Path::new(file).to_path_buf()
            } else {
                Path::new(&self.current_dir).join(file)
            };
            
            if !path.exists() && !force {
                self.output.push_back(format!("rm: {}: No such file or directory", file));
                continue;
            }
            
            if path.is_dir() && recursive {
                match fs::remove_dir_all(&path) {
                    Ok(_) => {
                        self.output.push_back(format!("Removed: {}", file));
                    }
                    Err(e) if !force => {
                        self.output.push_back(format!("rm: {}: {}", file, e));
                    }
                    _ => {}
                }
            } else if path.is_file() {
                match fs::remove_file(&path) {
                    Ok(_) => {
                        self.output.push_back(format!("Removed: {}", file));
                    }
                    Err(e) if !force => {
                        self.output.push_back(format!("rm: {}: {}", file, e));
                    }
                    _ => {}
                }
            } else if path.is_dir() {
                self.output.push_back(format!("rm: {}: Is a directory (use -r)", file));
            }
        }
    }
    
    /// 外部コマンドを実行
    fn execute_external_command(&mut self, cmd: &str, args: &[&str]) {
        let output = Command::new(cmd)
            .args(args)
            .current_dir(&self.current_dir)
            .output();
        
        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                
                for line in stdout.lines() {
                    self.output.push_back(line.to_string());
                }
                for line in stderr.lines() {
                    self.output.push_back(format!("[ERROR] {}", line));
                }
            }
            Err(e) => {
                self.output.push_back(format!("{}: command not found ({})", cmd, e));
            }
        }
    }
    
    /// ヘルプを表示
    fn show_help(&mut self) {
        let help_text = vec![
            "=== Available Commands ===",
            "",
            "File and Directory Operations:",
            "  ls [dir]      - List directory contents",
            "  cd [dir]      - Change directory",
            "  pwd           - Print working directory",
            "  mkdir <dir>   - Create directory",
            "  rmdir <dir>   - Remove empty directory",
            "  rm <file>     - Remove file",
            "    -f          - Force removal",
            "    -r          - Remove directories recursively",
            "",
            "Terminal Control:",
            "  clear         - Clear terminal",
            "  help          - Show this help",
            "  exit/quit     - (Use window close button)",
            "",
            "Shortcuts:",
            "  Up/Down       - Navigate command history",
            "  Ctrl+L        - Clear terminal",
            "  Enter         - Execute command",
        ];
        
        for line in help_text {
            self.output.push_back(line.to_string());
        }
    }
    
    /// 現在のディレクトリ名を取得
    fn get_dir_name(&self) -> String {
        Path::new(&self.current_dir)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("?")
            .to_string()
    }
}

impl eframe::App for TerminalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ダークテーマを設定
        ctx.set_visuals(egui::Visuals::dark());
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // タイトル
            ui.heading("🖥️ Taminal GUI Terminal");
            ui.separator();
            
            // 現在のディレクトリを表示
            ui.horizontal(|ui| {
                ui.label("Current Directory:");
                ui.monospace(&self.current_dir);
            });
            ui.separator();
            
            // ターミナル出力エリア
            let text_height = ui.available_height() - 60.0;
            egui::ScrollArea::vertical()
                .max_height(text_height)
                .auto_shrink([false; 2])
                .stick_to_bottom(self.auto_scroll)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        for line in &self.output {
                            ui.monospace(line);
                        }
                    });
                });
            
            ui.separator();
            
            // コマンド入力エリア
            ui.horizontal(|ui| {
                ui.label(format!("{}> ", self.get_dir_name()));
                
                let response = ui.add_sized(
                    [ui.available_width() - 100.0, 20.0],
                    egui::TextEdit::singleline(&mut self.input)
                );
                
                // フォーカスを維持
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    let command = self.input.clone();
                    self.input.clear();
                    if !command.is_empty() {
                        self.execute_command(command);
                    }
                    response.request_focus();
                }
                
                // 履歴ナビゲーション
                if response.has_focus() {
                    if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                        if self.history_index > 0 {
                            self.history_index -= 1;
                            self.input = self.command_history[self.history_index].clone();
                        }
                    }
                    if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                        if self.history_index < self.command_history.len() {
                            self.history_index += 1;
                            if self.history_index == self.command_history.len() {
                                self.input.clear();
                            } else {
                                self.input = self.command_history[self.history_index].clone();
                            }
                        }
                    }
                }
                
                if ui.button("Execute").clicked() {
                    let command = self.input.clone();
                    self.input.clear();
                    if !command.is_empty() {
                        self.execute_command(command);
                    }
                }
            });
            
            // ショートカット処理
            if ui.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::L)) {
                self.output.clear();
                self.output.push_back("=== Terminal Cleared ===".to_string());
            }
        });
    }
}