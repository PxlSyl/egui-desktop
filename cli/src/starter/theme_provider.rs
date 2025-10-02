use egui_desktop::detect_system_dark_mode;
use egui_desktop::{ThemeMode, ThemeProvider, TitleBarTheme};

pub struct SimpleThemeProvider;

impl SimpleThemeProvider {
    pub fn new() -> Self {
        Self
    }

    pub fn ocean_light() -> TitleBarTheme {
        TitleBarTheme::light_with_overrides(
            Some(egui::Color32::from_rgb(240, 248, 255)), // bg
            Some(egui::Color32::from_rgb(219, 234, 254)), // hover (faint_bg_color)
            Some(egui::Color32::from_rgb(232, 17, 35)),   // close hover
            Some(egui::Color32::from_rgb(59, 130, 246)),  // close icon
            Some(egui::Color32::from_rgb(59, 130, 246)),  // maximize icon
            Some(egui::Color32::from_rgb(59, 130, 246)),  // restore icon
            Some(egui::Color32::from_rgb(59, 130, 246)),  // minimize icon
            Some(egui::Color32::from_rgb(30, 64, 175)),   // title
            Some(egui::Color32::from_rgb(30, 64, 175)),   // menu text
            Some(14.0),
            Some(egui::Color32::from_rgb(219, 234, 254)), // menu hover (faint_bg_color)
            Some(egui::Color32::from_rgb(59, 130, 246)),  // keyboard selection color
            Some(egui::Color32::from_rgb(240, 248, 255)), // submenu background
            Some(egui::Color32::from_rgb(30, 64, 175)),   // submenu text (same as menu text)
            Some(egui::Color32::from_rgb(219, 234, 254)), // submenu hover
            Some(egui::Color32::from_rgb(100, 116, 139)), // submenu shortcut
            Some(egui::Color32::from_rgb(59, 130, 246)),  // submenu keyboard selection color
        )
    }

    pub fn ocean_dark() -> TitleBarTheme {
        TitleBarTheme::dark_with_overrides(
            Some(egui::Color32::from_rgb(30, 30, 46)),    // bg
            Some(egui::Color32::from_rgb(60, 60, 80)),    // hover (faint_bg_color)
            Some(egui::Color32::from_rgb(239, 68, 68)),   // close hover
            Some(egui::Color32::from_rgb(147, 197, 253)), // close icon
            Some(egui::Color32::from_rgb(147, 197, 253)), // maximize icon
            Some(egui::Color32::from_rgb(147, 197, 253)), // restore icon
            Some(egui::Color32::from_rgb(147, 197, 253)), // minimize icon
            Some(egui::Color32::from_rgb(191, 219, 254)), // title
            Some(egui::Color32::from_rgb(191, 219, 254)), // menu text
            Some(14.0),
            Some(egui::Color32::from_rgb(60, 60, 80)), // menu hover
            Some(egui::Color32::from_rgb(147, 197, 253)), // keyboard selection color
            Some(egui::Color32::from_rgb(30, 30, 46)), // submenu background
            Some(egui::Color32::from_rgb(191, 219, 254)), // submenu text
            Some(egui::Color32::from_rgb(60, 60, 80)), // submenu hover
            Some(egui::Color32::from_rgb(148, 163, 184)), // submenu shortcut
            Some(egui::Color32::from_rgb(147, 197, 253)), // submenu keyboard selection color
        )
    }

    pub fn forest_light() -> TitleBarTheme {
        TitleBarTheme::light_with_overrides(
            Some(egui::Color32::from_rgb(239, 246, 239)), // bg
            Some(egui::Color32::from_rgb(209, 250, 229)), // hover (faint_bg_color)
            Some(egui::Color32::from_rgb(232, 17, 35)),   // close hover
            Some(egui::Color32::from_rgb(16, 185, 129)),  // teal icons
            Some(egui::Color32::from_rgb(16, 185, 129)),
            Some(egui::Color32::from_rgb(16, 185, 129)),
            Some(egui::Color32::from_rgb(16, 185, 129)),
            Some(egui::Color32::from_rgb(22, 101, 52)), // title
            Some(egui::Color32::from_rgb(22, 101, 52)), // menu text
            Some(14.0),
            Some(egui::Color32::from_rgb(209, 250, 229)), // menu hover
            Some(egui::Color32::from_rgb(16, 185, 129)),  // keyboard selection color
            Some(egui::Color32::from_rgb(239, 246, 239)), // submenu background
            Some(egui::Color32::from_rgb(22, 101, 52)),   // submenu text
            Some(egui::Color32::from_rgb(209, 250, 229)), // submenu hover
            Some(egui::Color32::from_rgb(100, 116, 139)), // submenu shortcut
            Some(egui::Color32::from_rgb(16, 185, 129)),  // submenu keyboard selection color
        )
    }

    pub fn forest_dark() -> TitleBarTheme {
        TitleBarTheme::dark_with_overrides(
            Some(egui::Color32::from_rgb(20, 30, 20)),   // bg
            Some(egui::Color32::from_rgb(31, 41, 31)),   // hover (faint_bg_color)
            Some(egui::Color32::from_rgb(239, 68, 68)),  // close hover
            Some(egui::Color32::from_rgb(52, 211, 153)), // teal icons
            Some(egui::Color32::from_rgb(52, 211, 153)),
            Some(egui::Color32::from_rgb(52, 211, 153)),
            Some(egui::Color32::from_rgb(52, 211, 153)),
            Some(egui::Color32::from_rgb(187, 247, 208)), // title
            Some(egui::Color32::from_rgb(187, 247, 208)), // menu text
            Some(14.0),
            Some(egui::Color32::from_rgb(31, 41, 31)), // menu hover
            Some(egui::Color32::from_rgb(52, 211, 153)), // keyboard selection color
            Some(egui::Color32::from_rgb(20, 30, 20)), // submenu background
            Some(egui::Color32::from_rgb(187, 247, 208)), // submenu text
            Some(egui::Color32::from_rgb(31, 41, 31)), // submenu hover
            Some(egui::Color32::from_rgb(148, 163, 184)), // submenu shortcut
            Some(egui::Color32::from_rgb(52, 211, 153)), // submenu keyboard selection color
        )
    }
}

impl ThemeProvider for SimpleThemeProvider {
    fn get_title_bar_theme(&self, theme_id: &str, mode: ThemeMode) -> Option<TitleBarTheme> {
        match (theme_id, mode) {
            ("ocean", ThemeMode::Light) | ("ocean", ThemeMode::System)
                if !detect_system_dark_mode() =>
            {
                Some(Self::ocean_light())
            }
            ("ocean", ThemeMode::Dark) | ("ocean", ThemeMode::System)
                if detect_system_dark_mode() =>
            {
                Some(Self::ocean_dark())
            }
            ("forest", ThemeMode::Light) | ("forest", ThemeMode::System)
                if !detect_system_dark_mode() =>
            {
                Some(Self::forest_light())
            }
            ("forest", ThemeMode::Dark) | ("forest", ThemeMode::System)
                if detect_system_dark_mode() =>
            {
                Some(Self::forest_dark())
            }
            ("ocean", ThemeMode::Light) => Some(Self::ocean_light()),
            ("ocean", ThemeMode::Dark) => Some(Self::ocean_dark()),
            ("forest", ThemeMode::Light) => Some(Self::forest_light()),
            ("forest", ThemeMode::Dark) => Some(Self::forest_dark()),
            _ => None,
        }
    }

    fn get_egui_visuals(&self, theme_id: &str, mode: ThemeMode) -> Option<egui::Visuals> {
        let is_dark = match mode {
            ThemeMode::Light => false,
            ThemeMode::Dark => true,
            ThemeMode::System => detect_system_dark_mode(),
        };

        let mut v = if is_dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };

        match (theme_id, is_dark) {
            ("ocean", false) => {
                v.window_fill = egui::Color32::from_rgb(240, 248, 255);
                v.panel_fill = egui::Color32::from_rgb(240, 248, 255);
                v.faint_bg_color = egui::Color32::from_rgb(219, 234, 254);
                v.extreme_bg_color = egui::Color32::from_rgb(220, 235, 255);
                v.window_stroke.color = egui::Color32::from_rgb(59, 130, 246);
                v.window_stroke.width = 1.0;
                v.weak_text_color = Some(egui::Color32::from_rgb(59, 130, 246));
                v.hyperlink_color = egui::Color32::from_rgb(59, 130, 246);
                v.selection.bg_fill = egui::Color32::from_rgb(219, 234, 254);
                v.selection.stroke.color = egui::Color32::from_rgb(59, 130, 246);
                v.selection.stroke.width = 2.0;
                v.button_frame = true;
                v.widgets.inactive.bg_fill = egui::Color32::from_rgb(220, 235, 255);
                v.widgets.inactive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(30, 64, 175));
                v.widgets.hovered.bg_fill = egui::Color32::from_rgb(219, 234, 254);
                v.widgets.hovered.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(59, 130, 246));
                v.widgets.active.bg_fill = egui::Color32::from_rgb(59, 130, 246);
                v.widgets.active.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255));
                v.widgets.open.bg_fill = egui::Color32::from_rgb(219, 234, 254);
                v.widgets.open.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(59, 130, 246));
            }
            ("ocean", true) => {
                v.window_fill = egui::Color32::from_rgb(30, 30, 46);
                v.panel_fill = egui::Color32::from_rgb(30, 30, 46);
                v.faint_bg_color = egui::Color32::from_rgb(60, 60, 80);
                v.extreme_bg_color = egui::Color32::from_rgb(50, 50, 70);
                v.window_stroke.color = egui::Color32::from_rgb(147, 197, 253);
                v.window_stroke.width = 1.0;
                v.weak_text_color = Some(egui::Color32::from_rgb(147, 197, 253));
                v.hyperlink_color = egui::Color32::from_rgb(147, 197, 253);
                v.selection.bg_fill = egui::Color32::from_rgb(60, 60, 80);
                v.selection.stroke.color = egui::Color32::from_rgb(147, 197, 253);
                v.selection.stroke.width = 2.0;
                v.button_frame = true;
                v.widgets.inactive.bg_fill = egui::Color32::from_rgb(50, 50, 70);
                v.widgets.inactive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(191, 219, 254));
                v.widgets.hovered.bg_fill = egui::Color32::from_rgb(60, 60, 80);
                v.widgets.hovered.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(147, 197, 253));
                v.widgets.active.bg_fill = egui::Color32::from_rgb(147, 197, 253);
                v.widgets.active.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(30, 30, 46));
                v.widgets.open.bg_fill = egui::Color32::from_rgb(60, 60, 80);
                v.widgets.open.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(147, 197, 253));
            }
            ("forest", false) => {
                v.window_fill = egui::Color32::from_rgb(239, 246, 239);
                v.panel_fill = egui::Color32::from_rgb(239, 246, 239);
                v.faint_bg_color = egui::Color32::from_rgb(209, 250, 229);
                v.extreme_bg_color = egui::Color32::from_rgb(220, 235, 225);
                v.window_stroke.color = egui::Color32::from_rgb(16, 185, 129);
                v.window_stroke.width = 1.0;
                v.weak_text_color = Some(egui::Color32::from_rgb(22, 101, 52));
                v.hyperlink_color = egui::Color32::from_rgb(16, 185, 129);
                v.selection.bg_fill = egui::Color32::from_rgb(209, 250, 229);
                v.selection.stroke.color = egui::Color32::from_rgb(16, 185, 129);
                v.selection.stroke.width = 2.0;
                v.button_frame = true;
                v.widgets.inactive.bg_fill = egui::Color32::from_rgb(220, 252, 231);
                v.widgets.inactive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(22, 101, 52));
                v.widgets.hovered.bg_fill = egui::Color32::from_rgb(209, 250, 229);
                v.widgets.hovered.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(16, 185, 129));
                v.widgets.active.bg_fill = egui::Color32::from_rgb(16, 185, 129);
                v.widgets.active.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255));
                v.widgets.open.bg_fill = egui::Color32::from_rgb(209, 250, 229);
                v.widgets.open.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(16, 185, 129));
            }
            ("forest", true) => {
                v.window_fill = egui::Color32::from_rgb(20, 30, 20);
                v.panel_fill = egui::Color32::from_rgb(20, 30, 20);
                v.faint_bg_color = egui::Color32::from_rgb(31, 41, 31);
                v.extreme_bg_color = egui::Color32::from_rgb(25, 35, 25);
                v.window_stroke.color = egui::Color32::from_rgb(52, 211, 153);
                v.window_stroke.width = 1.0;
                v.weak_text_color = Some(egui::Color32::from_rgb(187, 247, 208));
                v.hyperlink_color = egui::Color32::from_rgb(52, 211, 153);
                v.selection.bg_fill = egui::Color32::from_rgb(31, 41, 31);
                v.selection.stroke.color = egui::Color32::from_rgb(52, 211, 153);
                v.selection.stroke.width = 2.0;
                v.button_frame = true;
                v.widgets.inactive.bg_fill = egui::Color32::from_rgb(30, 45, 30);
                v.widgets.inactive.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(187, 247, 208));
                v.widgets.hovered.bg_fill = egui::Color32::from_rgb(31, 41, 31);
                v.widgets.hovered.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(52, 211, 153));
                v.widgets.active.bg_fill = egui::Color32::from_rgb(52, 211, 153);
                v.widgets.active.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(20, 30, 20));
                v.widgets.open.bg_fill = egui::Color32::from_rgb(31, 41, 31);
                v.widgets.open.fg_stroke =
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(52, 211, 153));
            }
            _ => {}
        }

        Some(v)
    }

    fn list_available_themes(&self) -> Vec<String> {
        vec!["ocean".to_string(), "forest".to_string()]
    }
}
