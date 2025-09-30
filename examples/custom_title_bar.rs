use eframe::egui;
use egui_desktop::{apply_rounded_corners, render_resize_handles, TitleBar, TitleBarOptions};
use egui_extras::install_image_loaders;

struct CustomApp {
    counter: i32,
}

impl Default for CustomApp {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

impl eframe::App for CustomApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Apply native rounded corners (only called once)
        apply_rounded_corners(frame);

        // Custom title bar with integrated menu, custom colors, and custom icon
        TitleBar::new(TitleBarOptions::new().with_title("Custom App"))
            .with_background_color(egui::Color32::from_rgb(45, 45, 65)) // Dark blue background
            .with_hover_color(egui::Color32::from_rgb(65, 65, 85)) // Lighter blue hover
            .with_close_hover_color(egui::Color32::from_rgb(220, 20, 40)) // Custom red for close
            .with_title_color(egui::Color32::from_rgb(200, 200, 255)) // Light blue title text
            .with_menu_text_color(egui::Color32::from_rgb(200, 200, 255)) // Light blue menu text
            .with_menu_hover_color(egui::Color32::from_rgb(75, 75, 95)) // Darker blue menu hover
            .with_app_icon(
                b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"20\" height=\"20\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"#4CAF50\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><circle cx=\"12\" cy=\"12\" r=\"10\"></circle><path d=\"M8 14s1.5 2 4 2 4-2 4-2\"></path><line x1=\"9\" y1=\"9\" x2=\"9.01\" y2=\"9\"></line><line x1=\"15\" y1=\"9\" x2=\"15.01\" y2=\"9\"></line></svg>",
                "custom-smiley.svg"
            ) // Custom green smiley icon
            .add_menu_item("File", Some(Box::new(|| println!("File menu clicked!"))))
            .add_menu_item("Edit", Some(Box::new(|| println!("Edit menu clicked!"))))
            .add_menu_item("Help", Some(Box::new(|| println!("Help menu clicked!"))))
            .show(ctx);

        // Render resize handles for manual window resizing
        render_resize_handles(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Custom Title Bar Demo");
            ui.separator();

            ui.label("This app shows how to use custom title bars with integrated menus.");
            ui.label("• Native rounded corners applied");
            ui.label("• Custom title bar colors (dark blue theme)");
            ui.label("• Custom app icon (green smiley face)");
            ui.label("• Menu items integrated directly in the title bar");
            ui.label("• Resize handles for manual window resizing");

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Increment").clicked() {
                    self.counter += 1;
                }
                if ui.button("Decrement").clicked() {
                    self.counter -= 1;
                }
                if ui.button("Reset").clicked() {
                    self.counter = 0;
                }
            });

            ui.label(format!("Counter: {}", self.counter));
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
        "Custom Title Bar Demo",
        options,
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(CustomApp::default()))
        }),
    )
}
