use egui::{Color32, ImageSource};

use crate::theme::ThemeMode;

#[derive(Debug, Clone)]
pub struct TitleBarOptions {
    pub title: Option<String>,
    pub theme_mode: ThemeMode,
    pub show_title_on_macos: bool,
    pub show_title_on_windows: bool,
    pub show_title_on_linux: bool,
    pub background_color: Option<Color32>,
    pub hover_color: Option<Color32>,
    pub close_hover_color: Option<Color32>,
    pub close_icon_color: Option<Color32>,
    pub maximize_icon_color: Option<Color32>,
    pub restore_icon_color: Option<Color32>,
    pub minimize_icon_color: Option<Color32>,
    pub title_color: Option<Color32>,
    pub title_font_size: Option<f32>,
    pub menu_text_color: Option<Color32>,
    pub menu_text_size: Option<f32>,
    pub menu_hover_color: Option<Color32>,
    pub keyboard_selection_color: Option<Color32>,
    pub app_icon: Option<ImageSource<'static>>,
    pub show_close_button: Option<bool>,
    pub show_maximize_button: Option<bool>,
    pub show_minimize_button: Option<bool>,
}

impl Default for TitleBarOptions {
    fn default() -> Self {
        Self {
            title: None,
            theme_mode: ThemeMode::Light,
            show_title_on_macos: true,
            show_title_on_windows: true,
            show_title_on_linux: true,
            background_color: None,
            hover_color: None,
            close_hover_color: None,
            close_icon_color: None,
            maximize_icon_color: None,
            restore_icon_color: None,
            minimize_icon_color: None,
            title_color: None,
            title_font_size: None,
            menu_text_color: None,
            menu_text_size: None,
            menu_hover_color: None,
            keyboard_selection_color: None,
            app_icon: None,
            show_close_button: None,
            show_maximize_button: None,
            show_minimize_button: None,
        }
    }
}

impl TitleBarOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        let title_str = title.into();
        self.title = if title_str.is_empty() {
            None
        } else {
            Some(title_str)
        };
        self
    }

    pub fn with_theme_mode(mut self, theme_mode: ThemeMode) -> Self {
        self.theme_mode = theme_mode;
        self
    }

    pub fn with_title_visibility(mut self, macos: bool, windows: bool, linux: bool) -> Self {
        self.show_title_on_macos = macos;
        self.show_title_on_windows = windows;
        self.show_title_on_linux = linux;
        self
    }

    pub fn with_background_color(mut self, color: Color32) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn with_hover_color(mut self, color: Color32) -> Self {
        self.hover_color = Some(color);
        self
    }

    pub fn with_close_hover_color(mut self, color: Color32) -> Self {
        self.close_hover_color = Some(color);
        self
    }

    pub fn with_close_icon_color(mut self, color: Color32) -> Self {
        self.close_icon_color = Some(color);
        self
    }

    pub fn with_maximize_icon_color(mut self, color: Color32) -> Self {
        self.maximize_icon_color = Some(color);
        self
    }

    pub fn with_restore_icon_color(mut self, color: Color32) -> Self {
        self.restore_icon_color = Some(color);
        self
    }

    pub fn with_minimize_icon_color(mut self, color: Color32) -> Self {
        self.minimize_icon_color = Some(color);
        self
    }

    pub fn with_title_color(mut self, color: Color32) -> Self {
        self.title_color = Some(color);
        self
    }

    pub fn with_title_font_size(mut self, size: f32) -> Self {
        self.title_font_size = Some(size);
        self
    }

    pub fn with_menu_text_color(mut self, color: Color32) -> Self {
        self.menu_text_color = Some(color);
        self
    }

    pub fn with_menu_hover_color(mut self, color: Color32) -> Self {
        self.menu_hover_color = Some(color);
        self
    }

    pub fn with_keyboard_selection_color(mut self, color: Color32) -> Self {
        self.keyboard_selection_color = Some(color);
        self
    }

    pub fn with_menu_text_size(mut self, size: f32) -> Self {
        self.menu_text_size = Some(size);
        self
    }

    pub fn with_app_icon(mut self, bytes: &'static [u8], uri: &'static str) -> Self {
        let icon = ImageSource::Bytes {
            uri: std::borrow::Cow::Borrowed(uri),
            bytes: egui::load::Bytes::Static(bytes),
        };
        self.app_icon = Some(icon);
        self
    }

    pub fn with_show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = Some(show);
        self
    }

    pub fn with_show_maximize_button(mut self, show: bool) -> Self {
        self.show_maximize_button = Some(show);
        self
    }

    pub fn with_show_minimize_button(mut self, show: bool) -> Self {
        self.show_minimize_button = Some(show);
        self
    }
}
