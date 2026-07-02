//! ビルトインコマンドをまとめるモジュール
//!
//! 1コマンド1ファイル構成。各モジュールは自身の名前 `NAME` と
//! 実行関数 `run(&[&str])` を公開する。ここでそれらをテーブル化し、
//! 名前からの検索・実行（dispatch）を提供する。

pub mod cd;
pub mod clear;
pub mod help;
pub mod ls;
pub mod mkdir;
pub mod pwd;
pub mod rm;
pub mod rmdir;

/// 1つのビルトインコマンドを表す（名前と実行関数の組）
pub struct Command {
    pub name: &'static str,
    pub run: fn(&[&str]),
}

/// 登録済みビルトインコマンド一覧。
/// 新しいコマンドはモジュールを追加し、ここに1行足すだけでよい。
pub const COMMANDS: &[Command] = &[
    Command { name: cd::NAME, run: cd::run },
    Command { name: pwd::NAME, run: pwd::run },
    Command { name: ls::NAME, run: ls::run },
    Command { name: rm::NAME, run: rm::run },
    Command { name: rmdir::NAME, run: rmdir::run },
    Command { name: mkdir::NAME, run: mkdir::run },
    Command { name: clear::NAME, run: clear::run },
    Command { name: help::NAME, run: help::run },
];

/// 名前に一致するコマンドを実行する。
/// 該当コマンドがなければ何もせず false を返す。
pub fn dispatch(name: &str, args: &[&str]) -> bool {
    for cmd in COMMANDS {
        if cmd.name == name {
            (cmd.run)(args);
            return true;
        }
    }
    false
}
