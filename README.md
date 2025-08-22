# Taminal - シンプルなターミナルエミュレータ

Rustで実装されたターミナルエミュレータです。CLIとGUIの両方のインターフェースを提供します。

## 特徴

- 基本的なシェルコマンドの実行
- ビルトインファイル/ディレクトリ操作コマンド
- 外部コマンドの実行
- 現在のディレクトリ名をプロンプトに表示
- 包括的なエラーハンドリング
- cdコマンドのディレクトリ補完ヒント
- **GUI版とCLI版の両方を提供**

## インストール

```bash
git clone <repository>
cd taminal
cargo build --release
```

## 実行方法

### CLI版（ターミナル内で実行）

開発時の実行：
```bash
cargo run --bin taminal
```

ビルド済みバイナリの実行：
```bash
./target/release/taminal
```

### GUI版（独立したウィンドウアプリケーション）

開発時の実行：
```bash
cargo run --bin taminal_gui
```

ビルド済みバイナリの実行：
```bash
./target/release/taminal_gui
```

## 使用可能なコマンド

### ファイル・ディレクトリ操作

| コマンド | 説明 | 使用例 |
|---------|------|--------|
| `ls [dir]` | ディレクトリの内容を表示 | `ls`, `ls src/` |
| `cd [dir]` | ディレクトリを移動 | `cd src`, `cd ..`, `cd` (ホーム) |
| `pwd` | 現在のディレクトリを表示 | `pwd` |
| `mkdir <dir>` | ディレクトリを作成 | `mkdir new_folder` |
| `rmdir <dir>` | 空のディレクトリを削除 | `rmdir old_folder` |
| `rm <file>` | ファイルやディレクトリを削除 | `rm file.txt` |
| `rm -f <file>` | 強制削除（エラー無視） | `rm -f temp.txt` |
| `rm -r <dir>` | ディレクトリを再帰的に削除 | `rm -r folder/` |
| `rm -rf <dir>` | 強制的に再帰削除 | `rm -rf build/` |

### ターミナル制御

| コマンド | 説明 |
|---------|------|
| `clear` | 画面をクリア |
| `help` | ヘルプメッセージを表示 |
| `exit` / `quit` | ターミナルを終了 |

### 外部コマンド

上記以外のコマンドは外部コマンドとして実行されます（例：`echo`, `cat`, `grep` など）

## ショートカットキー

### CLI版
- `Ctrl+C` - 実行中のコマンドを中断
- `Ctrl+D` - 空行で終了

### GUI版
- `Enter` - コマンドを実行
- `↑/↓` - コマンド履歴を参照
- `Ctrl+L` - 画面をクリア
- マウスクリックで「Execute」ボタンも使用可能

## GUI版の特徴

- **ネイティブウィンドウアプリケーション** - macOS/Windows/Linuxで動作
- **ビジュアルインターフェース** - 見やすいテキストエリアとコマンド入力フィールド
- **マウス操作対応** - クリックでコマンド実行可能
- **コマンド履歴機能** - 上下矢印キーで過去のコマンドを参照
- **スクロール可能な出力** - 長い出力も確認可能
- **ダークテーマ** - 目に優しいダークモード
- **自動スクロール** - 新しい出力に自動でスクロール

## 使用例

```bash
# 新しいディレクトリを作成
mkdir my_project

# ディレクトリに移動
cd my_project

# サブディレクトリを作成
mkdir src
mkdir docs

# 内容を確認
ls

# ファイルを削除
rm unwanted.txt

# ディレクトリとその内容をすべて削除
rm -rf old_project/

# 親ディレクトリに戻る
cd ..

# 空のディレクトリを削除
rmdir empty_folder
```

## システム要件

- Rust 1.70.0以上
- GUI版の場合：
  - macOS 10.12以上
  - Windows 10以上
  - Linux（X11またはWayland）

## ビルドオプション

### CLI版のみビルド
```bash
cargo build --release --bin taminal
```

### GUI版のみビルド
```bash
cargo build --release --bin taminal_gui
```

### 両方ビルド
```bash
cargo build --release
```

## トラブルシューティング

### GUI版が起動しない場合

Linux環境では以下のパッケージが必要な場合があります：
```bash
# Ubuntu/Debian
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev

# Fedora
sudo dnf install libxcb-devel
```

### macOSでの権限エラー

初回実行時に「開発元が未確認」エラーが出る場合：
1. システム環境設定 → セキュリティとプライバシー
2. 「このまま開く」をクリック

## ライセンス

MIT