use eframe::egui;
use egui_extras::install_image_loaders;

use PROJECT_NAME_PLACEHOLDER::CustomThemeDemoApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([800.0, 600.0])
            .with_decorations(false),
        ..Default::default()
    };

    eframe::run_native(
        "Egui Desktop UI Demo",
        options,
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(CustomThemeDemoApp::default()))
        }),
    )
}
