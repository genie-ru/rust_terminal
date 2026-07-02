//! clearコマンド。ANSIエスケープで画面をクリアする

use std::io::{self, Write};

pub fn run() {
    // \x1B[2J: 画面クリア / \x1B[1;1H: カーソルを左上へ
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}
