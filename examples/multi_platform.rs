use eframe::egui;
use egui_desktop::{
    apply_rounded_corners, render_resize_handles, supports_native_rounded_corners, TitleBar,
    TitleBarOptions,
};
use egui_extras::install_image_loaders;

struct MultiPlatformApp {
    platform_info: String,
}

impl Default for MultiPlatformApp {
    fn default() -> Self {
        Self {
            platform_info: format!(
                "Running on: {} ({})",
                std::env::consts::OS,
                std::env::consts::ARCH
            ),
        }
    }
}

impl eframe::App for MultiPlatformApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Apply native rounded corners (only called once)
        apply_rounded_corners(frame);

        // Platform-appropriate title bar with default icon
        TitleBar::new(TitleBarOptions::new().with_title("Multi-Platform Demo"))
            .with_background_color(egui::Color32::from_rgb(35, 35, 35)) // Dark theme
            .show(ctx);

        // Render resize handles for manual window resizing
        render_resize_handles(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Cross-Platform egui-desktop-ui Demo");
            ui.separator();

            ui.label(&self.platform_info);
            ui.label(format!(
                "Title bar type: {}",
                if cfg!(target_os = "macos") {
                    "macOS Custom (Traffic Lights)"
                } else {
                    "Generic (Standard Controls)"
                }
            ));

            ui.separator();

            ui.label("Features by platform:");
            ui.label("• macOS: Authentic traffic light buttons (red, yellow, green)");
            ui.label(
                "• Windows: Standard minimize/maximize/close buttons + Windows 11 rounded corners",
            );
            ui.label("• Linux: Generic controls compatible with all window managers");
            ui.label("• Default app icon displayed on Windows/Linux (not macOS)");

            ui.separator();

            ui.label("Advanced features:");
            ui.label(format!(
                "• Native rounded corners: {}",
                if supports_native_rounded_corners() {
                    "✅ Supported"
                } else {
                    "❌ Not supported"
                }
            ));
            ui.label("• Custom resize handles for precise window control");
            ui.label("• Authentic Windows 11 icons and hover effects");

            ui.separator();

            ui.label("Try the window controls:");
            ui.label("• Drag the title bar to move the window");
            ui.label("• Double-click the title bar to maximize/restore");
            ui.label("• Use the control buttons to minimize/maximize/close");
            ui.label("• Drag window edges/corners to resize manually");

            ui.separator();

            if ui.button("Show Platform Details").clicked() {
                self.show_platform_details(ui);
            }
        });
    }
}

impl MultiPlatformApp {
    fn show_platform_details(&self, ui: &mut egui::Ui) {
        egui::Window::new("Platform Details")
            .open(&mut true)
            .show(ui.ctx(), |ui| {
                ui.label(format!("OS: {}", std::env::consts::OS));
                ui.label(format!("Architecture: {}", std::env::consts::ARCH));
                ui.label(format!("Family: {}", std::env::consts::FAMILY));
                ui.label(format!(
                    "Title bar height: {}px",
                    if cfg!(target_os = "macos") { 28 } else { 32 }
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
        "Multi-Platform Demo",
        options,
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MultiPlatformApp::default()))
        }),
    )
}
