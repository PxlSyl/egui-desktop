use eframe::egui;
use egui_desktop::{apply_rounded_corners, render_resize_handles, TitleBar, TitleBarOptions};
use egui_extras::install_image_loaders;

struct MyApp {
    name: String,
    click_count: i32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "egui-desktop-ui Demo".to_string(),
            click_count: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Apply native rounded corners (only called once)
        apply_rounded_corners(frame);

        // Render the appropriate title bar for your OS (defaults to light theme)
        TitleBar::new(TitleBarOptions::new().with_title(&self.name)).show(ctx);

        // Render resize handles for manual window resizing
        render_resize_handles(ctx);

        // Your app content here
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello from egui-desktop-ui!");
            ui.separator();

            ui.label("This example demonstrates:");
            ui.label("• macOS: Custom title bar with traffic light buttons");
            ui.label("• Windows/Linux: Generic title bar with standard controls");
            ui.label("• Default app icon (Windows/Linux only)");
            ui.label("• Light theme (default) with white background");
            ui.label("• Auto-detection of your operating system");

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Click me!").clicked() {
                    self.click_count += 1;
                }
                ui.label(format!("Clicked {} times!", self.click_count));
            });

            ui.separator();

            ui.label(format!("Current OS: {}", std::env::consts::OS));
            ui.label(format!(
                "Title bar type: {}",
                if cfg!(target_os = "macos") {
                    "macOS Custom"
                } else {
                    "Generic"
                }
            ));
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([800.0, 600.0])
            .with_decorations(false),
        ..Default::default()
    };

    eframe::run_native(
        "Egui Desktop UI Demo",
        options,
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}
