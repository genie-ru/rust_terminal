mod gui;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // アプリケーションの設定
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0])
            .with_title("Taminal - GUI Terminal"),
        ..Default::default()
    };
    
    // アプリケーションを起動
    eframe::run_native(
        "Taminal GUI",
        options,
        Box::new(|cc| Ok(Box::new(gui::TerminalApp::new(cc)))),
    )
}