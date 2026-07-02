//! 外部コマンド実行
//! ビルトインコマンド以外を子プロセスとして実行する

use std::process::{Command, Stdio};

/// 外部コマンドを子プロセスとして起動し、終了を待つ
pub fn execute_external_command(command: &str, args: &[&str]) {
    let child = Command::new(command)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    match child {
        Ok(mut process) => match process.wait() {
            Ok(status) => {
                if !status.success() {
                    if let Some(code) = status.code() {
                        eprintln!("Command exited with status: {}", code);
                    }
                }
            }
            Err(e) => eprintln!("Failed to wait for command: {}", e),
        },
        Err(e) => {
            eprintln!("{}: command not found ({})", command, e);
        }
    }
}
