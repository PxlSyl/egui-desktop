use eframe::egui;
use egui_desktop::{
    apply_rounded_corners, detect_system_dark_mode, render_resize_handles, CustomIcon,
    KeyboardShortcut, MenuItem, SubMenuItem, ThemeMode, ThemeProvider, TitleBar, TitleBarOptions,
    TitleBarTheme,
};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Mutex;

use crate::content::render_main_content;
use crate::icons::draw_gear_icon;
use crate::sidebar::render_sidebar;
use crate::theme_provider::SimpleThemeProvider;

static TOGGLE_SIDEBAR: AtomicBool = AtomicBool::new(false);
static THEME_CHANGE_REQUEST: AtomicU8 = AtomicU8::new(0); // 0 = none, 1 = Light, 2 = Dark, 3 = Auto, 4 = CustomLight, 5 = CustomDark
static CUSTOM_THEME_ID_REQUEST: Mutex<Option<String>> = Mutex::new(None);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppTheme {
    Light,
    Dark,
    CustomLight,
    CustomDark,
    Auto,
}

#[derive(Debug, Clone)]
pub struct SidebarAnimation {
    pub current_position: f32,
    pub target_position: f32,
    pub animation_speed: f32,
}

pub struct CustomThemeDemoApp {
    pub app_theme: AppTheme,
    pub title_bar: TitleBar,
    pub show_sidebar: bool,
    pub title_bar_initialized: bool,
    pub selected_custom_id: String,
    pub sidebar_animation: SidebarAnimation,
}

impl Default for CustomThemeDemoApp {
    fn default() -> Self {
        Self {
            app_theme: AppTheme::Light,
            title_bar: TitleBar::new(TitleBarOptions::new().with_title("Custom Theme"))
                .with_theme_mode(ThemeMode::Light)
                .add_menu_item("File", None)
                .add_menu_item("Edit", None)
                .add_menu_item("View", None),
            show_sidebar: true,
            title_bar_initialized: false,
            selected_custom_id: "ocean".to_string(),
            sidebar_animation: SidebarAnimation {
                current_position: 1.0,
                target_position: 1.0,
                animation_speed: 12.0,
            },
        }
    }
}

fn ease_linear(t: f32) -> f32 {
    t.clamp(0.0, 1.0)
}

impl CustomThemeDemoApp {
    pub fn get_text_color(&self, ui: &egui::Ui) -> egui::Color32 {
        match self.app_theme {
            AppTheme::Light => egui::Color32::BLACK,
            AppTheme::Dark => egui::Color32::WHITE,
            AppTheme::Auto => {
                if ui.ctx().style().visuals.dark_mode {
                    egui::Color32::WHITE
                } else {
                    egui::Color32::BLACK
                }
            }
            AppTheme::CustomLight => {
                match self.selected_custom_id.as_str() {
                    "ocean" => egui::Color32::from_rgb(30, 58, 138), // Dark blue for ocean light
                    "forest" => egui::Color32::from_rgb(34, 68, 34), // Dark green for forest light
                    _ => egui::Color32::BLACK,
                }
            }
            AppTheme::CustomDark => {
                match self.selected_custom_id.as_str() {
                    "ocean" => egui::Color32::from_rgb(147, 197, 253), // Light blue for ocean dark
                    "forest" => egui::Color32::from_rgb(134, 239, 172), // Light green for forest dark
                    _ => egui::Color32::WHITE,
                }
            }
        }
    }

    pub fn update_sidebar_animation(&mut self, ctx: &egui::Context) {
        self.sidebar_animation.target_position = if self.show_sidebar { 1.0 } else { 0.0 };

        let delta_time = ctx.input(|i| i.unstable_dt);
        let diff = self.sidebar_animation.target_position - self.sidebar_animation.current_position;

        if diff.abs() > 0.001 {
            let step = self.sidebar_animation.animation_speed * delta_time;
            if diff.abs() <= step {
                self.sidebar_animation.current_position = self.sidebar_animation.target_position;
            } else {
                self.sidebar_animation.current_position += diff.signum() * step;
            }

            ctx.request_repaint();
        }
    }

    pub fn toggle_sidebar(&mut self) {
        self.show_sidebar = !self.show_sidebar;
    }

    pub fn initialize_title_bar(&mut self) {
        let file_menu = MenuItem::new("File")
            .add_subitem(
                SubMenuItem::new("New")
                    .with_shortcut(KeyboardShortcut::parse("ctrl+n"))
                    .with_callback(Box::new(|| println!("New file!"))),
            )
            .add_subitem(
                SubMenuItem::new("Open")
                    .with_shortcut(KeyboardShortcut::parse("ctrl+o"))
                    .with_callback(Box::new(|| println!("Open file!"))),
            )
            .add_subitem(
                SubMenuItem::new("Recent")
                    .add_child(
                        SubMenuItem::new("Project A")
                            .with_callback(Box::new(|| println!("Open Project A"))),
                    )
                    .add_child(
                        SubMenuItem::new("Project B")
                            .with_callback(Box::new(|| println!("Open Project B")))
                            .with_separator(),
                    ),
            )
            .add_subitem(
                SubMenuItem::new("Save")
                    .with_shortcut(KeyboardShortcut::parse("ctrl+s"))
                    .with_callback(Box::new(|| println!("Save file!")))
                    .with_separator(),
            )
            .add_subitem(
                SubMenuItem::new("Exit")
                    .with_shortcut(KeyboardShortcut::parse("ctrl+q"))
                    .with_callback(Box::new(|| println!("Exit app!"))),
            );

        let edit_menu = MenuItem::new("Edit")
            .add_subitem(
                SubMenuItem::new("Undo")
                    .with_shortcut(KeyboardShortcut::parse("ctrl+z"))
                    .with_callback(Box::new(|| println!("Undo!"))),
            )
            .add_subitem(
                SubMenuItem::new("Redo")
                    .with_shortcut(KeyboardShortcut::parse("ctrl+y"))
                    .with_callback(Box::new(|| println!("Redo!")))
                    .with_separator(),
            )
            .add_subitem(
                SubMenuItem::new("Find")
                    .add_child(
                        SubMenuItem::new("Find Next")
                            .with_shortcut(KeyboardShortcut::parse("f3"))
                            .with_callback(Box::new(|| println!("Find Next"))),
                    )
                    .add_child(
                        SubMenuItem::new("Find Previous")
                            .with_shortcut(KeyboardShortcut::parse("shift+f3"))
                            .with_callback(Box::new(|| println!("Find Previous"))),
                    ),
            )
            .add_subitem(
                SubMenuItem::new("Cut")
                    .with_shortcut(KeyboardShortcut::parse("ctrl+x"))
                    .with_callback(Box::new(|| println!("Cut!"))),
            )
            .add_subitem(
                SubMenuItem::new("Copy")
                    .with_shortcut(KeyboardShortcut::parse("ctrl+c"))
                    .with_callback(Box::new(|| println!("Copy!"))),
            )
            .add_subitem(
                SubMenuItem::new("Paste")
                    .with_shortcut(KeyboardShortcut::parse("ctrl+v"))
                    .with_callback(Box::new(|| println!("Paste!"))),
            );

        let view_menu = MenuItem::new("View")
            .add_subitem(
                SubMenuItem::new("Zoom In")
                    .with_shortcut(KeyboardShortcut::parse("ctrl+="))
                    .with_callback(Box::new(|| println!("Zoom in!"))),
            )
            .add_subitem(
                SubMenuItem::new("Zoom Out")
                    .with_shortcut(KeyboardShortcut::parse("-"))
                    .with_callback(Box::new(|| println!("Zoom out!"))),
            )
            .add_subitem(
                SubMenuItem::new("Reset Zoom")
                    .with_shortcut(KeyboardShortcut::parse("0"))
                    .with_callback(Box::new(|| println!("Reset zoom!")))
                    .with_separator(),
            )
            .add_subitem(SubMenuItem::new("Light Theme").with_callback(Box::new(|| {
                THEME_CHANGE_REQUEST.store(1, Ordering::Relaxed);
            })))
            .add_subitem(SubMenuItem::new("Dark Theme").with_callback(Box::new(|| {
                THEME_CHANGE_REQUEST.store(2, Ordering::Relaxed);
            })))
            .add_subitem(
                SubMenuItem::new("Auto Theme")
                    .with_callback(Box::new(|| {
                        THEME_CHANGE_REQUEST.store(3, Ordering::Relaxed);
                    }))
                    .with_separator(),
            )
            .add_subitem(
                SubMenuItem::new("🌊 Ocean Light").with_callback(Box::new(|| {
                    THEME_CHANGE_REQUEST.store(4, Ordering::Relaxed);
                    if let Ok(mut id_request) = CUSTOM_THEME_ID_REQUEST.lock() {
                        *id_request = Some("ocean".to_string());
                    }
                })),
            )
            .add_subitem(
                SubMenuItem::new("🌊 Ocean Dark").with_callback(Box::new(|| {
                    THEME_CHANGE_REQUEST.store(5, Ordering::Relaxed);
                    if let Ok(mut id_request) = CUSTOM_THEME_ID_REQUEST.lock() {
                        *id_request = Some("ocean".to_string());
                    }
                })),
            )
            .add_subitem(
                SubMenuItem::new("🌲 Forest Light").with_callback(Box::new(|| {
                    THEME_CHANGE_REQUEST.store(4, Ordering::Relaxed);
                    if let Ok(mut id_request) = CUSTOM_THEME_ID_REQUEST.lock() {
                        *id_request = Some("forest".to_string());
                    }
                })),
            )
            .add_subitem(
                SubMenuItem::new("🌲 Forest Dark").with_callback(Box::new(|| {
                    THEME_CHANGE_REQUEST.store(5, Ordering::Relaxed);
                    if let Ok(mut id_request) = CUSTOM_THEME_ID_REQUEST.lock() {
                        *id_request = Some("forest".to_string());
                    }
                })),
            );

        self.title_bar = TitleBar::new(TitleBarOptions::new().with_title("Egui Desktop"))
            .with_theme_mode(ThemeMode::Light)
            .with_theme_provider(SimpleThemeProvider::new())
            .add_menu_with_submenu(file_menu)
            .add_menu_with_submenu(edit_menu)
            .add_menu_with_submenu(view_menu)
            .add_icon(
                CustomIcon::Drawn(Box::new(draw_gear_icon)),
                Some(Box::new(|| {
                    TOGGLE_SIDEBAR.store(true, Ordering::Relaxed);
                })),
                Some("Toggle Sidebar".to_string()),
                Some(KeyboardShortcut::parse("ctrl+b")),
            );

        // Set larger title font size for custom themes
        self.title_bar.title_font_size = 16.0;
    }
}

impl eframe::App for CustomThemeDemoApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        apply_rounded_corners(frame);

        // Handle theme change requests from menu
        let theme_request = THEME_CHANGE_REQUEST.swap(0, Ordering::Relaxed);
        if theme_request != 0 {
            match theme_request {
                1 => {
                    self.app_theme = AppTheme::Light;
                    ctx.request_repaint();
                }
                2 => {
                    self.app_theme = AppTheme::Dark;
                    ctx.request_repaint();
                }
                3 => {
                    self.app_theme = AppTheme::Auto;
                    ctx.request_repaint();
                }
                4 => {
                    self.app_theme = AppTheme::CustomLight;
                    ctx.request_repaint();
                }
                5 => {
                    self.app_theme = AppTheme::CustomDark;
                    ctx.request_repaint();
                }
                _ => {}
            }
        }

        if let Ok(mut id_request) = CUSTOM_THEME_ID_REQUEST.lock() {
            if let Some(new_id) = id_request.take() {
                self.selected_custom_id = new_id;
                ctx.request_repaint();
            }
        }

        if TOGGLE_SIDEBAR.swap(false, Ordering::Relaxed) {
            self.toggle_sidebar();
        }

        self.update_sidebar_animation(ctx);

        match self.app_theme {
            AppTheme::CustomLight | AppTheme::CustomDark => {
                let _ = self.title_bar.switch_theme(ctx, &self.selected_custom_id);
            }
            AppTheme::Light => ctx.set_visuals(egui::Visuals::light()),
            AppTheme::Dark => ctx.set_visuals(egui::Visuals::dark()),
            AppTheme::Auto => {
                if detect_system_dark_mode() {
                    ctx.set_visuals(egui::Visuals::dark());
                } else {
                    ctx.set_visuals(egui::Visuals::light());
                }
            }
        }

        if !self.title_bar_initialized {
            self.initialize_title_bar();
            self.title_bar_initialized = true;
        }

        match self.app_theme {
            AppTheme::Light => {
                self.title_bar.update_theme_mode(ThemeMode::Light);
                self.title_bar.set_custom_icon_color(0, None);
                self.title_bar.title_font_size = 12.0; // Default size
            }
            AppTheme::Dark => {
                self.title_bar.update_theme_mode(ThemeMode::Dark);
                self.title_bar.set_custom_icon_color(0, None);
                self.title_bar.title_font_size = 12.0; // Default size
            }
            AppTheme::CustomLight => {
                self.title_bar.update_theme_mode(ThemeMode::Light);
                let _ = self.title_bar.switch_theme(ctx, &self.selected_custom_id);
                self.title_bar.title_font_size = 16.0; // Larger size for custom themes
                                                       // Adapt custom icon color based on theme
                match self.selected_custom_id.as_str() {
                    "ocean" => {
                        self.title_bar
                            .set_custom_icon_color(0, Some(egui::Color32::from_rgb(59, 130, 246)));
                    }
                    "forest" => {
                        self.title_bar
                            .set_custom_icon_color(0, Some(egui::Color32::from_rgb(16, 185, 129)));
                    }
                    _ => {
                        self.title_bar.set_custom_icon_color(0, None);
                    }
                }
            }
            AppTheme::CustomDark => {
                self.title_bar.update_theme_mode(ThemeMode::Dark);
                let _ = self.title_bar.switch_theme(ctx, &self.selected_custom_id);
                self.title_bar.title_font_size = 16.0; // Larger size for custom themes
                                                       // Adapt custom icon color based on theme
                match self.selected_custom_id.as_str() {
                    "ocean" => {
                        self.title_bar
                            .set_custom_icon_color(0, Some(egui::Color32::from_rgb(147, 197, 253)));
                    }
                    "forest" => {
                        self.title_bar
                            .set_custom_icon_color(0, Some(egui::Color32::from_rgb(52, 211, 153)));
                    }
                    _ => {
                        self.title_bar.set_custom_icon_color(0, None);
                    }
                }
            }
            AppTheme::Auto => {
                self.title_bar.update_theme_mode(ThemeMode::System);
                self.title_bar.sync_with_system_theme();
                self.title_bar.title_font_size = 12.0; // Default size
                                                       // Reset custom icon color to follow system visuals
                self.title_bar.set_custom_icon_color(0, None);
            }
        }

        self.title_bar.handle_icon_shortcuts(ctx);

        self.title_bar.show(ctx);

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(ctx.style().visuals.panel_fill))
            .show(ctx, |_ui| {});

        let eased_position = ease_linear(self.sidebar_animation.current_position);
        let sidebar_width = 300.0 * eased_position;

        if self.sidebar_animation.current_position > 0.01 {
            egui::SidePanel::left("sidebar")
                .resizable(false)
                .exact_width(sidebar_width)
                .show(ctx, |ui| {
                    if self.sidebar_animation.current_position < 0.8 {
                        ui.allocate_ui(ui.available_size(), |_ui| {});
                    } else {
                        render_sidebar(self, ui);
                    }
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            render_main_content(self, ui);
        });

        render_resize_handles(ctx);
    }
}
