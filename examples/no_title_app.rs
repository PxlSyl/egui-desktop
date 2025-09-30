use eframe::egui;
use egui_desktop::{apply_rounded_corners, render_resize_handles, TitleBar, TitleBarOptions};
use egui_extras::install_image_loaders;

struct MyApp {
    name: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "egui-desktop-ui No Title Demo".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Apply native rounded corners (only called once)
        apply_rounded_corners(frame);

        // Title bar WITHOUT title text (only icon and controls)
        TitleBar::new(
            TitleBarOptions::new()
                .with_background_color(egui::Color32::from_rgb(40, 40, 40)) // Dark background
                .with_hover_color(egui::Color32::from_rgb(60, 60, 60)) // Dark gray hover
                .with_app_icon(
                    b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"#4A90E2\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><rect x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" fill=\"#E8F4FD\"/><rect x=\"3\" y=\"3\" width=\"18\" height=\"6\" rx=\"2\" ry=\"2\" fill=\"#4A90E2\"/><circle cx=\"6\" cy=\"6\" r=\"1\" fill=\"#FF5F57\"/><circle cx=\"9\" cy=\"6\" r=\"1\" fill=\"#FFBD2E\"/><circle cx=\"12\" cy=\"6\" r=\"1\" fill=\"#28CA42\"/><rect x=\"6\" y=\"12\" width=\"3\" height=\"3\" fill=\"#4A90E2\"/><rect x=\"12\" y=\"12\" width=\"6\" height=\"3\" fill=\"#4A90E2\"/></svg>",
                    "window-no-title.svg"
                )
        )
        .show(ctx);

        // Render resize handles for manual window resizing
        render_resize_handles(ctx);

        // Your app content here
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Title Bar Without Title Text!");
            ui.separator();

            ui.label("This example demonstrates:");
            ui.label("• Title bar with NO title text");
            ui.label("• Custom window icon");
            ui.label("• Platform-specific behavior:");
            ui.label("  - macOS: Traffic light buttons only");
            ui.label("  - Windows/Linux: App icon + control buttons");
            ui.label("• Useful for minimal UI designs");
            ui.label("• Cross-platform title bar without text");

            ui.separator();

            ui.label("Platform differences:");
            ui.label("• macOS: Traffic lights (red, yellow, green) on the left");
            ui.label("• Windows/Linux: Custom app icon on left, controls on right");
            ui.label("• No title text is displayed on any platform");
            ui.label("• Perfect for apps that want minimal title bars");

            ui.separator();

            if ui.button("Click me!").clicked() {
                self.name = format!(
                    "Clicked {} times!",
                    self.name
                        .split_whitespace()
                        .last()
                        .unwrap_or("0")
                        .parse::<i32>()
                        .unwrap_or(0)
                        + 1
                );
            }

            ui.label(&format!("App name: {}", self.name));
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
        "egui-desktop-ui No Title Example",
        options,
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}
