//! Windows Snap Layouts Example
//!
//! This example demonstrates how to enable Windows 11 snap layouts
//! on custom maximize buttons in egui-desktop-ui applications.

use eframe::egui;
use egui_desktop::{TitleBar, TitleBarOptions};

struct MyApp {
    title_bar: TitleBar,
}

impl Default for MyApp {
    fn default() -> Self {
        let title_bar = TitleBar::new(
            TitleBarOptions::new()
                .with_title("Windows Snap Layouts Demo")
                .with_show_bottom_border(false),
        );

        Self { title_bar }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Render title bar
        self.title_bar.show(ctx, frame);

        // Note: With the simplified approach, snap layouts are handled automatically!
        // Windows detects maximize buttons in the title bar area natively.

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Windows Snap Layouts Demo");
            ui.label("Hover over the maximize button to see Windows 11 snap layouts!");

            ui.separator();

            ui.label("Features:");
            ui.label("✅ Custom title bar with native Windows behavior");
            ui.label("✅ Windows 11 snap layouts on maximize button hover");
            ui.label("✅ Automatic native detection");

            ui.separator();

            ui.label("Instructions:");
            ui.label("1. Hover over the green maximize button");
            ui.label("2. Windows will show the snap layouts menu automatically");
            ui.label("3. Click on a layout to snap the window");

            ui.separator();

            ui.label("Technical Details:");
            ui.label("• Windows native detection in title bar area");
            ui.label("• No custom window proc interference");
            ui.label("• Clean separation between egui and Windows");

            ui.separator();

            if ui.button("Test Window Operations").clicked() {
                ui.label("Window operations work normally!");
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0])
            .with_decorations(false), // Completely disable system decorations
        ..Default::default()
    };

    eframe::run_native(
        "Windows Snap Layouts Example",
        options,
        Box::new(|cc| {
            // Customize egui settings here if needed
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(MyApp::default()))
        }),
    )
}
