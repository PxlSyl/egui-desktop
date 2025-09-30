use egui::{Color32, ImageSource};

use crate::theme::ThemeMode;

/// Configuration options for the title bar component.
#[derive(Debug, Clone)]
pub struct TitleBarOptions {
    /// Optional title text.
    pub title: Option<String>,
    /// Desired theme mode.
    pub theme_mode: ThemeMode,
    /// Show title text on macOS.
    pub show_title_on_macos: bool,
    /// Show title text on Windows.
    pub show_title_on_windows: bool,
    /// Show title text on Linux.
    pub show_title_on_linux: bool,
    /// Background color override.
    pub background_color: Option<Color32>,
    /// Hover color for window controls.
    pub hover_color: Option<Color32>,
    /// Close button hover color.
    pub close_hover_color: Option<Color32>,
    /// Close icon color.
    pub close_icon_color: Option<Color32>,
    /// Maximize icon color.
    pub maximize_icon_color: Option<Color32>,
    /// Restore icon color.
    pub restore_icon_color: Option<Color32>,
    /// Minimize icon color.
    pub minimize_icon_color: Option<Color32>,
    /// Title text color.
    pub title_color: Option<Color32>,
    /// Title font size in points.
    pub title_font_size: Option<f32>,
    /// Menu text color.
    pub menu_text_color: Option<Color32>,
    /// Menu text size in points.
    pub menu_text_size: Option<f32>,
    /// Menu hover background color.
    pub menu_hover_color: Option<Color32>,
    /// Keyboard selection highlight color for menus.
    pub keyboard_selection_color: Option<Color32>,
    /// Optional app icon image.
    pub app_icon: Option<ImageSource<'static>>,
    /// Show the close button.
    pub show_close_button: Option<bool>,
    /// Show the maximize button.
    pub show_maximize_button: Option<bool>,
    /// Show the minimize button.
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
    /// Create default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set window title text (empty becomes `None`).
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        let title_str = title.into();
        self.title = if title_str.is_empty() {
            None
        } else {
            Some(title_str)
        };
        self
    }

    /// Select the theme mode.
    pub fn with_theme_mode(mut self, theme_mode: ThemeMode) -> Self {
        self.theme_mode = theme_mode;
        self
    }

    /// Configure on which platforms the title text is displayed.
    pub fn with_title_visibility(mut self, macos: bool, windows: bool, linux: bool) -> Self {
        self.show_title_on_macos = macos;
        self.show_title_on_windows = windows;
        self.show_title_on_linux = linux;
        self
    }

    /// Override background color.
    pub fn with_background_color(mut self, color: Color32) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Override hover color for window controls.
    pub fn with_hover_color(mut self, color: Color32) -> Self {
        self.hover_color = Some(color);
        self
    }

    /// Override hover color for close button.
    pub fn with_close_hover_color(mut self, color: Color32) -> Self {
        self.close_hover_color = Some(color);
        self
    }

    /// Override close icon color.
    pub fn with_close_icon_color(mut self, color: Color32) -> Self {
        self.close_icon_color = Some(color);
        self
    }

    /// Override maximize icon color.
    pub fn with_maximize_icon_color(mut self, color: Color32) -> Self {
        self.maximize_icon_color = Some(color);
        self
    }

    /// Override restore icon color.
    pub fn with_restore_icon_color(mut self, color: Color32) -> Self {
        self.restore_icon_color = Some(color);
        self
    }

    /// Override minimize icon color.
    pub fn with_minimize_icon_color(mut self, color: Color32) -> Self {
        self.minimize_icon_color = Some(color);
        self
    }

    /// Override title text color.
    pub fn with_title_color(mut self, color: Color32) -> Self {
        self.title_color = Some(color);
        self
    }

    /// Override title font size.
    pub fn with_title_font_size(mut self, size: f32) -> Self {
        self.title_font_size = Some(size);
        self
    }

    /// Override menu text color.
    pub fn with_menu_text_color(mut self, color: Color32) -> Self {
        self.menu_text_color = Some(color);
        self
    }

    /// Override menu hover background color.
    pub fn with_menu_hover_color(mut self, color: Color32) -> Self {
        self.menu_hover_color = Some(color);
        self
    }

    /// Override keyboard selection highlight color.
    pub fn with_keyboard_selection_color(mut self, color: Color32) -> Self {
        self.keyboard_selection_color = Some(color);
        self
    }

    /// Override menu text size.
    pub fn with_menu_text_size(mut self, size: f32) -> Self {
        self.menu_text_size = Some(size);
        self
    }

    /// Set an app icon from embedded image bytes.
    pub fn with_app_icon(mut self, bytes: &'static [u8], uri: &'static str) -> Self {
        let icon = ImageSource::Bytes {
            uri: std::borrow::Cow::Borrowed(uri),
            bytes: egui::load::Bytes::Static(bytes),
        };
        self.app_icon = Some(icon);
        self
    }

    /// Show or hide the close button.
    pub fn with_show_close_button(mut self, show: bool) -> Self {
        self.show_close_button = Some(show);
        self
    }

    /// Show or hide the maximize button.
    pub fn with_show_maximize_button(mut self, show: bool) -> Self {
        self.show_maximize_button = Some(show);
        self
    }

    /// Show or hide the minimize button.
    pub fn with_show_minimize_button(mut self, show: bool) -> Self {
        self.show_minimize_button = Some(show);
        self
    }
}
