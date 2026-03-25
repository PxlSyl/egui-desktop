use eframe::egui;
use egui::{CentralPanel, Context, ViewportBuilder, ViewportCommand, ViewportId, pos2, vec2};
use egui_desktop::{
    TitleBar, TitleBarOptions, apply_rounded_corners, apply_rounded_corners_to_viewport,
    render_resize_handles,
};
use egui_extras::install_image_loaders;
use std::sync::{Arc, Mutex};

// Shared app state
struct AppState {
    counter: i32,
    name: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            counter: 0,
            name: "egui-desktop-ui".to_string(),
        }
    }
}

struct MultiWindowApp {
    // Title bars for each window
    main_title_bar: TitleBar,
    settings_title_bar: Arc<Mutex<TitleBar>>,
    about_title_bar: Arc<Mutex<TitleBar>>,

    // Window state (shared)
    settings_open: Arc<Mutex<bool>>,
    about_open: Arc<Mutex<bool>>,

    // Shared app state
    app_state: Arc<Mutex<AppState>>,
}

impl Default for MultiWindowApp {
    fn default() -> Self {
        Self {
            main_title_bar: TitleBar::new(
                TitleBarOptions::new()
                    .with_title("Multi-Window Demo")
                    .with_show_close_button(true)
                    .with_show_maximize_button(true)
                    .with_show_minimize_button(true),
            ),
            settings_title_bar: Arc::new(Mutex::new(TitleBar::new(
                TitleBarOptions::new()
                    .with_title("Settings")
                    .with_show_close_button(true)
                    .with_show_maximize_button(false)
                    .with_show_minimize_button(false),
            ))),
            about_title_bar: Arc::new(Mutex::new(TitleBar::new(
                TitleBarOptions::new()
                    .with_title("About")
                    .with_show_close_button(true)
                    .with_show_maximize_button(false)
                    .with_show_minimize_button(false),
            ))),
            settings_open: Arc::new(Mutex::new(false)),
            about_open: Arc::new(Mutex::new(false)),
            app_state: Arc::new(Mutex::new(AppState::default())),
        }
    }
}

impl eframe::App for MultiWindowApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        // Apply native rounded corners to the main window
        apply_rounded_corners(frame);

        // Render main window title bar
        self.main_title_bar.show(ctx, frame);

        // Render resize handles for the main window
        render_resize_handles(ctx);

        // Main window content
        let app_state = self.app_state.clone();
        let settings_open = self.settings_open.clone();
        let about_open = self.about_open.clone();

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Multi-Window Application Demo");
            ui.separator();

            ui.label("This example demonstrates multiple windows with independent title bars.");
            ui.label("Each window has its own TitleBar instance with isolated state.");

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Open Settings Window").clicked() {
                    *settings_open.lock().unwrap() = true;
                }
                if ui.button("Open About Window").clicked() {
                    *about_open.lock().unwrap() = true;
                }
            });

            ui.separator();

            let mut state = app_state.lock().unwrap();
            ui.horizontal(|ui| {
                if ui.button("Increment").clicked() {
                    state.counter += 1;
                }
                if ui.button("Decrement").clicked() {
                    state.counter -= 1;
                }
                if ui.button("Reset").clicked() {
                    state.counter = 0;
                }
            });

            ui.label(format!("Counter: {}", state.counter));

            ui.separator();

            ui.label("Window Status:");
            ui.label(format!(
                "• Settings window: {}",
                if *settings_open.lock().unwrap() {
                    "Open"
                } else {
                    "Closed"
                }
            ));
            ui.label(format!(
                "• About window: {}",
                if *about_open.lock().unwrap() {
                    "Open"
                } else {
                    "Closed"
                }
            ));
        });

        // Create Settings window
        if *self.settings_open.lock().unwrap() {
            let settings_size = vec2(500.0, 400.0);
            let viewport_id = ViewportId::from_hash_of("settings_window");

            // Calculate initial position more reliably
            let initial_pos = if let Some(inner_rect) = ctx.input(|i| i.viewport().inner_rect) {
                inner_rect.center() - settings_size * 0.5
            } else {
                // Fallback to screen center if we can't get main window position
                egui::pos2(100.0, 100.0)
            };

            let viewport_builder = ViewportBuilder::default()
                .with_title("Settings")
                .with_inner_size(settings_size)
                .with_min_inner_size(vec2(400.0, 300.0))
                .with_decorations(false)
                .with_resizable(true)
                .with_position(initial_pos);

            let app_state = self.app_state.clone();
            let settings_title_bar = self.settings_title_bar.clone();
            let settings_open = self.settings_open.clone();

            ctx.show_viewport_deferred(viewport_id, viewport_builder, move |ctx, _frame| {
                // Check if window was closed by user
                if ctx.input(|i| i.viewport().close_requested()) {
                    *settings_open.lock().unwrap() = false;
                    return;
                }

                // Apply rounded corners to this viewport (if supported)
                apply_rounded_corners_to_viewport(ctx);

                // Settings window title bar - thread-safe version
                settings_title_bar.lock().unwrap().show_without_frame(ctx);

                // Render resize handles for settings window
                render_resize_handles(ctx);

                // Settings window content
                let app_state = app_state.clone();
                let settings_open = settings_open.clone();
                CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Settings");
                    ui.separator();

                    ui.label("Application Settings");
                    ui.separator();

                    let mut state = app_state.lock().unwrap();
                    ui.horizontal(|ui| {
                        ui.label("Application Name:");
                        ui.text_edit_singleline(&mut state.name);
                    });

                    ui.separator();

                    ui.label("Theme Options:");
                    if ui.button("Light Theme").clicked() {
                        // You can set theme per window here
                    }
                    if ui.button("Dark Theme").clicked() {
                        // You can set theme per window here
                    }

                    ui.separator();

                    ui.label(format!("Current counter value: {}", state.counter));

                    ui.separator();

                    if ui.button("Close Settings").clicked() {
                        *settings_open.lock().unwrap() = false;
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });
            });
        }

        // Create About window
        if *self.about_open.lock().unwrap() {
            let about_size = vec2(400.0, 300.0);
            let viewport_id = ViewportId::from_hash_of("about_window");

            // Calculate initial position more reliably
            let initial_pos = if let Some(inner_rect) = ctx.input(|i| i.viewport().inner_rect) {
                inner_rect.center() - about_size * 0.5
            } else {
                // Fallback to screen center if we can't get main window position
                pos2(150.0, 150.0)
            };

            let viewport_builder = ViewportBuilder::default()
                .with_title("About")
                .with_inner_size(about_size)
                .with_min_inner_size(egui::vec2(350.0, 250.0))
                .with_decorations(false)
                .with_resizable(true)
                .with_position(initial_pos);

            let app_state = self.app_state.clone();
            let about_title_bar = self.about_title_bar.clone();
            let about_open = self.about_open.clone();

            ctx.show_viewport_deferred(viewport_id, viewport_builder, move |ctx, _frame| {
                // Check if window was closed by user
                if ctx.input(|i| i.viewport().close_requested()) {
                    *about_open.lock().unwrap() = false;
                    return;
                }

                // Apply rounded corners to this viewport (if supported)
                apply_rounded_corners_to_viewport(ctx);

                // About window title bar - thread-safe version
                about_title_bar.lock().unwrap().show_without_frame(ctx);

                // Render resize handles for about window
                render_resize_handles(ctx);

                // About window content
                let app_state = app_state.clone();
                let about_open = about_open.clone();
                CentralPanel::default().show(ctx, |ui| {
                    ui.heading("About Multi-Window Demo");
                    ui.separator();

                    let state = app_state.lock().unwrap();
                    ui.label(format!("Application: {}", state.name));
                    ui.label("Version: 1.0.0");
                    ui.separator();

                    ui.label("Features:");
                    ui.label("• Multiple independent windows");
                    ui.label("• Each window has its own TitleBar");
                    ui.label("• Isolated state per window");
                    ui.label("• Custom window controls");
                    ui.separator();

                    ui.label("Built with:");
                    ui.label("• egui-desktop");
                    ui.label("• eframe");
                    ui.label("• egui");

                    ui.separator();

                    if ui.button("Close About").clicked() {
                        *about_open.lock().unwrap() = false;
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });
            });
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0])
            .with_decorations(false),
        ..Default::default()
    };

    eframe::run_native(
        "Multi-Window Demo",
        options,
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MultiWindowApp::default()))
        }),
    )
}
