use egui::{Color32, Visuals};

pub mod api;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

pub struct TitleBarTheme {
    pub background_color: Color32,
    pub hover_color: Color32,
    pub close_hover_color: Color32,
    pub close_icon_color: Color32,
    pub maximize_icon_color: Color32,
    pub restore_icon_color: Color32,
    pub minimize_icon_color: Color32,
    pub title_color: Color32,
    pub menu_text_color: Color32,
    pub menu_text_size: f32,
    pub menu_hover_color: Color32,
    pub keyboard_selection_color: Color32,
    // Submenu customization
    pub submenu_background_color: Color32,
    pub submenu_text_color: Color32,
    pub submenu_text_size: f32,
    pub submenu_hover_color: Color32,
    pub submenu_disabled_color: Color32,
    pub submenu_shortcut_color: Color32,
    pub submenu_border_color: Color32,
    pub submenu_keyboard_selection_color: Color32,
}

/// A provider interface for supplying themes by identifier at runtime.
pub trait ThemeProvider: Send + Sync {
    /// Return a `TitleBarTheme` for the given theme id and mode, if available
    fn get_title_bar_theme(&self, theme_id: &str, mode: ThemeMode) -> Option<TitleBarTheme>;
    /// Return egui Visuals for the given theme id and mode, if available
    fn get_egui_visuals(&self, theme_id: &str, mode: ThemeMode) -> Option<Visuals>;
    /// List all available theme ids
    fn list_available_themes(&self) -> Vec<String>;
}

#[derive(Debug)]
pub enum ThemeError {
    ThemeNotFound,
}

impl Default for TitleBarTheme {
    fn default() -> Self {
        Self::light()
    }
}

impl TitleBarTheme {
    pub fn light() -> Self {
        Self {
            background_color: Color32::WHITE,
            hover_color: Color32::from_rgb(230, 230, 230),
            close_hover_color: Color32::from_rgb(232, 17, 35),
            close_icon_color: Color32::from_rgb(100, 100, 100),
            maximize_icon_color: Color32::from_rgb(100, 100, 100),
            restore_icon_color: Color32::from_rgb(100, 100, 100),
            minimize_icon_color: Color32::from_rgb(100, 100, 100),
            title_color: Color32::from_rgb(50, 50, 50),
            menu_text_color: Color32::from_rgb(50, 50, 50),
            menu_text_size: 12.0,
            menu_hover_color: Color32::from_rgb(230, 230, 230),
            keyboard_selection_color: Color32::from_rgb(0, 120, 215),
            submenu_background_color: Color32::WHITE,
            submenu_text_color: Color32::from_rgb(50, 50, 50),
            submenu_text_size: 11.0,
            submenu_hover_color: Color32::from_rgb(240, 240, 240),
            submenu_disabled_color: Color32::from_rgb(150, 150, 150),
            submenu_shortcut_color: Color32::from_rgb(100, 100, 100),
            submenu_border_color: Color32::from_rgb(200, 200, 200),
            submenu_keyboard_selection_color: Color32::from_rgb(0, 120, 215),
        }
    }

    pub fn dark() -> Self {
        Self {
            background_color: Color32::from_rgb(30, 30, 30),
            hover_color: Color32::from_rgb(60, 60, 60),
            close_hover_color: Color32::from_rgb(232, 17, 35),
            close_icon_color: Color32::from_rgb(200, 200, 200),
            maximize_icon_color: Color32::from_rgb(200, 200, 200),
            restore_icon_color: Color32::from_rgb(200, 200, 200),
            minimize_icon_color: Color32::from_rgb(200, 200, 200),
            title_color: Color32::from_rgb(200, 200, 200),
            menu_text_color: Color32::from_rgb(200, 200, 200),
            menu_text_size: 12.0,
            menu_hover_color: Color32::from_rgb(60, 60, 60),
            keyboard_selection_color: Color32::from_rgb(30, 144, 255),
            submenu_background_color: Color32::from_rgb(40, 40, 40),
            submenu_text_color: Color32::from_rgb(200, 200, 200),
            submenu_text_size: 11.0,
            submenu_hover_color: Color32::from_rgb(70, 70, 70),
            submenu_disabled_color: Color32::from_rgb(120, 120, 120),
            submenu_shortcut_color: Color32::from_rgb(160, 160, 160),
            submenu_border_color: Color32::from_rgb(80, 80, 80),
            submenu_keyboard_selection_color: Color32::from_rgb(30, 144, 255),
        }
    }

    pub fn light_with_overrides(
        background_color: Option<Color32>,
        hover_color: Option<Color32>,
        close_hover_color: Option<Color32>,
        close_icon_color: Option<Color32>,
        maximize_icon_color: Option<Color32>,
        restore_icon_color: Option<Color32>,
        minimize_icon_color: Option<Color32>,
        title_color: Option<Color32>,
        menu_text_color: Option<Color32>,
        menu_text_size: Option<f32>,
        menu_hover_color: Option<Color32>,
        keyboard_selection_color: Option<Color32>,
        submenu_background_color: Option<Color32>,
        submenu_text_color: Option<Color32>,
        submenu_hover_color: Option<Color32>,
        submenu_shortcut_color: Option<Color32>,
        submenu_keyboard_selection_color: Option<Color32>,
    ) -> Self {
        let default = Self::light();
        Self {
            background_color: background_color.unwrap_or(default.background_color),
            hover_color: hover_color.unwrap_or(default.hover_color),
            close_hover_color: close_hover_color.unwrap_or(default.close_hover_color),
            close_icon_color: close_icon_color.unwrap_or(default.close_icon_color),
            maximize_icon_color: maximize_icon_color.unwrap_or(default.maximize_icon_color),
            restore_icon_color: restore_icon_color.unwrap_or(default.restore_icon_color),
            minimize_icon_color: minimize_icon_color.unwrap_or(default.minimize_icon_color),
            title_color: title_color.unwrap_or(default.title_color),
            menu_text_color: menu_text_color.unwrap_or(default.menu_text_color),
            menu_text_size: menu_text_size.unwrap_or(default.menu_text_size),
            menu_hover_color: menu_hover_color.unwrap_or(default.menu_hover_color),
            keyboard_selection_color: keyboard_selection_color
                .unwrap_or(default.keyboard_selection_color),
            submenu_background_color: submenu_background_color
                .unwrap_or(default.submenu_background_color),
            submenu_text_color: submenu_text_color.unwrap_or(default.submenu_text_color),
            submenu_text_size: default.submenu_text_size,
            submenu_hover_color: submenu_hover_color.unwrap_or(default.submenu_hover_color),
            submenu_disabled_color: default.submenu_disabled_color,
            submenu_shortcut_color: submenu_shortcut_color
                .unwrap_or(default.submenu_shortcut_color),
            submenu_border_color: default.submenu_border_color,
            submenu_keyboard_selection_color: submenu_keyboard_selection_color
                .unwrap_or(default.submenu_keyboard_selection_color),
        }
    }

    pub fn dark_with_overrides(
        background_color: Option<Color32>,
        hover_color: Option<Color32>,
        close_hover_color: Option<Color32>,
        close_icon_color: Option<Color32>,
        maximize_icon_color: Option<Color32>,
        restore_icon_color: Option<Color32>,
        minimize_icon_color: Option<Color32>,
        title_color: Option<Color32>,
        menu_text_color: Option<Color32>,
        menu_text_size: Option<f32>,
        menu_hover_color: Option<Color32>,
        keyboard_selection_color: Option<Color32>,
        submenu_background_color: Option<Color32>,
        submenu_text_color: Option<Color32>,
        submenu_hover_color: Option<Color32>,
        submenu_shortcut_color: Option<Color32>,
        submenu_keyboard_selection_color: Option<Color32>,
    ) -> Self {
        let default = Self::dark();
        Self {
            background_color: background_color.unwrap_or(default.background_color),
            hover_color: hover_color.unwrap_or(default.hover_color),
            close_hover_color: close_hover_color.unwrap_or(default.close_hover_color),
            close_icon_color: close_icon_color.unwrap_or(default.close_icon_color),
            maximize_icon_color: maximize_icon_color.unwrap_or(default.maximize_icon_color),
            restore_icon_color: restore_icon_color.unwrap_or(default.restore_icon_color),
            minimize_icon_color: minimize_icon_color.unwrap_or(default.minimize_icon_color),
            title_color: title_color.unwrap_or(default.title_color),
            menu_text_color: menu_text_color.unwrap_or(default.menu_text_color),
            menu_text_size: menu_text_size.unwrap_or(default.menu_text_size),
            menu_hover_color: menu_hover_color.unwrap_or(default.menu_hover_color),
            keyboard_selection_color: keyboard_selection_color
                .unwrap_or(default.keyboard_selection_color),
            submenu_background_color: submenu_background_color
                .unwrap_or(default.submenu_background_color),
            submenu_text_color: submenu_text_color.unwrap_or(default.submenu_text_color),
            submenu_text_size: default.submenu_text_size,
            submenu_hover_color: submenu_hover_color.unwrap_or(default.submenu_hover_color),
            submenu_disabled_color: default.submenu_disabled_color,
            submenu_shortcut_color: submenu_shortcut_color
                .unwrap_or(default.submenu_shortcut_color),
            submenu_border_color: default.submenu_border_color,
            submenu_keyboard_selection_color: submenu_keyboard_selection_color
                .unwrap_or(default.submenu_keyboard_selection_color),
        }
    }
}

pub use ThemeMode::*;

/// Detect if the system is using dark mode
pub fn detect_system_dark_mode() -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        // On Windows, check the registry for the system theme
        match Command::new("reg")
            .args(&["query", "HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize", "/v", "AppsUseLightTheme"])
            .output()
        {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                // If AppsUseLightTheme is 0, then dark mode is enabled
                !output_str.contains("0x1")
            }
            Err(_) => false, // Default to light mode if we can't detect
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        // On macOS, check the system appearance
        match Command::new("defaults")
            .args(&["read", "-g", "AppleInterfaceStyle"])
            .output()
        {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                output_str.contains("Dark")
            }
            Err(_) => false, // Default to light mode if we can't detect
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        // On Linux, try to detect via gsettings (GNOME)
        if let Ok(output) = Command::new("gsettings")
            .args(&["get", "org.gnome.desktop.interface", "gtk-theme"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return output_str.contains("dark") || output_str.contains("Dark");
        }

        // Fallback: check environment variable
        std::env::var("GTK_THEME")
            .map(|theme| theme.contains("dark") || theme.contains("Dark"))
            .unwrap_or(false)
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        false // Default to light mode for unknown platforms
    }
}
