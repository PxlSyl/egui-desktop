use egui::{Color32, Context};

use crate::theme::{detect_system_dark_mode, ThemeError, ThemeMode, ThemeProvider, TitleBarTheme};
use crate::TitleBar;

impl TitleBar {
    /// Attach a ThemeProvider to this TitleBar
    pub fn with_theme_provider<T: ThemeProvider + 'static>(mut self, provider: T) -> Self {
        self.theme_provider = Some(Box::new(provider));
        self
    }

    /// Switch theme using the provider by id, applying both TitleBar theme and egui Visuals
    pub fn switch_theme(&mut self, ctx: &Context, theme_id: &str) -> Result<(), ThemeError> {
        let mode = self.theme_mode;
        // Collect provider outputs first to avoid overlapping borrows of `self`
        let (tb_theme_opt, visuals_opt) = if let Some(provider) = &self.theme_provider {
            (
                provider.get_title_bar_theme(theme_id, mode),
                provider.get_egui_visuals(theme_id, mode),
            )
        } else {
            (None, None)
        };

        if let Some(tb_theme) = tb_theme_opt {
            self.apply_theme(tb_theme);
            if let Some(visuals) = visuals_opt {
                ctx.set_visuals(visuals);
            }
            self.current_theme_id = Some(theme_id.to_string());
            Ok(())
        } else {
            Err(ThemeError::ThemeNotFound)
        }
    }

    /// Set the theme mode (Light, Dark, or System)
    ///
    /// This method changes the theme mode and immediately applies the corresponding
    /// theme colors. For System mode, it detects the current system theme.
    ///
    /// # Arguments
    /// * `mode` - The theme mode to apply
    ///
    /// # Examples
    ///
    /// ```rust
    /// title_bar.with_theme_mode(ThemeMode::Dark)
    ///     .with_theme_mode(ThemeMode::System)
    /// ```
    pub fn with_theme_mode(mut self, mode: ThemeMode) -> Self {
        self.theme_mode = mode;
        self.apply_theme_mode();
        self
    }

    /// Update the theme mode without recreating the title bar
    pub fn update_theme_mode(&mut self, theme_mode: ThemeMode) {
        self.theme_mode = theme_mode;
        self.apply_theme_mode();
    }

    /// Update with custom light theme without recreating the title bar
    pub fn update_custom_light_theme(
        &mut self,
        overrides: (
            Option<Color32>, // background_color
            Option<Color32>, // hover_color
            Option<Color32>, // close_hover_color
            Option<Color32>, // close_icon_color
            Option<Color32>, // maximize_icon_color
            Option<Color32>, // restore_icon_color
            Option<Color32>, // minimize_icon_color
            Option<Color32>, // title_color
            Option<Color32>, // menu_text_color
            Option<f32>,     // menu_text_size
            Option<Color32>, // menu_hover_color
            Option<Color32>, // submenu_background_color
            Option<Color32>, // submenu_text_color
            Option<Color32>, // submenu_hover_color
            Option<Color32>, // submenu_shortcut_color
            Option<Color32>, // keyboard_selection_color
            Option<Color32>, // submenu_keyboard_selection_color
        ),
    ) {
        self.theme_mode = ThemeMode::Light;
        let theme = TitleBarTheme::light_with_overrides(
            overrides.0,
            overrides.1,
            overrides.2,
            overrides.3,
            overrides.4,
            overrides.5,
            overrides.6,
            overrides.7,
            overrides.8,
            overrides.9,
            overrides.10,
            overrides.11,
            overrides.12,
            overrides.13,
            overrides.14,
            overrides.15,
            overrides.16,
        );
        self.apply_theme(theme);
    }

    /// Update with custom dark theme without recreating the title bar
    pub fn update_custom_dark_theme(
        &mut self,
        overrides: (
            Option<Color32>, // background_color
            Option<Color32>, // hover_color
            Option<Color32>, // close_hover_color
            Option<Color32>, // close_icon_color
            Option<Color32>, // maximize_icon_color
            Option<Color32>, // restore_icon_color
            Option<Color32>, // minimize_icon_color
            Option<Color32>, // title_color
            Option<Color32>, // menu_text_color
            Option<f32>,     // menu_text_size
            Option<Color32>, // menu_hover_color
            Option<Color32>, // submenu_background_color
            Option<Color32>, // submenu_text_color
            Option<Color32>, // submenu_hover_color
            Option<Color32>, // submenu_shortcut_color
            Option<Color32>, // keyboard_selection_color
            Option<Color32>, // submenu_keyboard_selection_color
        ),
    ) {
        self.theme_mode = ThemeMode::Dark;
        let theme = TitleBarTheme::dark_with_overrides(
            overrides.0,
            overrides.1,
            overrides.2,
            overrides.3,
            overrides.4,
            overrides.5,
            overrides.6,
            overrides.7,
            overrides.8,
            overrides.9,
            overrides.10,
            overrides.11,
            overrides.12,
            overrides.13,
            overrides.14,
            overrides.15,
            overrides.16,
        );
        self.apply_theme(theme);
    }

    /// Set a custom theme with all color properties
    ///
    /// This method applies a complete TitleBarTheme, overriding all color settings.
    ///
    /// # Arguments
    /// * `theme` - The complete theme to apply
    ///
    /// # Examples
    ///
    /// ```rust
    /// let custom_theme = TitleBarTheme {
    ///     background_color: Color32::from_rgb(45, 45, 65),
    ///     hover_color: Color32::from_rgb(65, 65, 85),
    ///     close_hover_color: Color32::from_rgb(220, 20, 40),
    ///     close_icon_color: Color32::from_rgb(180, 180, 180),
    ///     maximize_icon_color: Color32::from_rgb(180, 180, 180),
    ///     restore_icon_color: Color32::from_rgb(180, 180, 180),
    ///     minimize_icon_color: Color32::from_rgb(180, 180, 180),
    ///     title_color: Color32::from_rgb(200, 200, 255),
    ///     menu_text_color: Color32::from_rgb(200, 200, 200),
    ///     menu_text_size: 14.0,
    ///     menu_hover_color: Color32::from_rgb(60, 60, 60),
    /// };
    /// title_bar.with_theme(custom_theme)
    /// ```
    pub fn with_theme(mut self, theme: TitleBarTheme) -> Self {
        self.background_color = theme.background_color;
        self.hover_color = theme.hover_color;
        self.close_hover_color = theme.close_hover_color;
        self.close_icon_color = theme.close_icon_color;
        self.maximize_icon_color = theme.maximize_icon_color;
        self.restore_icon_color = theme.restore_icon_color;
        self.minimize_icon_color = theme.minimize_icon_color;
        self.title_color = theme.title_color;
        self.menu_text_color = theme.menu_text_color;
        self.menu_text_size = theme.menu_text_size;
        self.menu_hover_color = theme.menu_hover_color;
        self
    }

    /// Create a custom light theme with specific overrides
    ///
    /// This is a convenience method that creates a custom light theme with only
    /// the specified color overrides, keeping all other values at their defaults.
    ///
    /// # Arguments
    /// * `overrides` - A tuple of optional color overrides in the same order as TitleBarTheme fields
    ///
    /// # Examples
    ///
    /// ```rust
    /// title_bar.with_custom_light_theme((
    ///     Some(Color32::from_rgb(240, 240, 240)), // Custom background
    ///     None, // Default hover color
    ///     None, // Default close hover color
    ///     Some(Color32::from_rgb(120, 120, 120)), // Custom close icon color
    ///     None, None, None, None, None, None, None, // Default values
    /// ))
    /// ```
    pub fn with_custom_light_theme(
        self,
        overrides: (
            Option<Color32>, // background_color
            Option<Color32>, // hover_color
            Option<Color32>, // close_hover_color
            Option<Color32>, // close_icon_color
            Option<Color32>, // maximize_icon_color
            Option<Color32>, // restore_icon_color
            Option<Color32>, // minimize_icon_color
            Option<Color32>, // title_color
            Option<Color32>, // menu_text_color
            Option<f32>,     // menu_text_size
            Option<Color32>, // menu_hover_color
            Option<Color32>, // submenu_background_color
            Option<Color32>, // submenu_text_color
            Option<Color32>, // submenu_hover_color
            Option<Color32>, // submenu_shortcut_color
            Option<Color32>, // keyboard_selection_color
            Option<Color32>, // submenu_keyboard_selection_color
        ),
    ) -> Self {
        let theme = TitleBarTheme::light_with_overrides(
            overrides.0,
            overrides.1,
            overrides.2,
            overrides.3,
            overrides.4,
            overrides.5,
            overrides.6,
            overrides.7,
            overrides.8,
            overrides.9,
            overrides.10,
            overrides.11,
            overrides.12,
            overrides.13,
            overrides.14,
            overrides.15,
            overrides.16,
        );
        self.with_theme(theme)
    }

    /// Create a custom dark theme with specific overrides
    ///
    /// This is a convenience method that creates a custom dark theme with only
    /// the specified color overrides, keeping all other values at their defaults.
    ///
    /// # Arguments
    /// * `overrides` - A tuple of optional color overrides in the same order as TitleBarTheme fields
    ///
    /// # Examples
    ///
    /// ```rust
    /// title_bar.with_custom_dark_theme((
    ///     Some(Color32::from_rgb(20, 20, 20)), // Custom background
    ///     None, // Default hover color
    ///     None, // Default close hover color
    ///     Some(Color32::from_rgb(220, 220, 220)), // Custom close icon color
    ///     None, None, None, None, None, None, None, // Default values
    /// ))
    /// ```
    pub fn with_custom_dark_theme(
        self,
        overrides: (
            Option<Color32>, // background_color
            Option<Color32>, // hover_color
            Option<Color32>, // close_hover_color
            Option<Color32>, // close_icon_color
            Option<Color32>, // maximize_icon_color
            Option<Color32>, // restore_icon_color
            Option<Color32>, // minimize_icon_color
            Option<Color32>, // title_color
            Option<Color32>, // menu_text_color
            Option<f32>,     // menu_text_size
            Option<Color32>, // menu_hover_color
            Option<Color32>, // submenu_background_color
            Option<Color32>, // submenu_text_color
            Option<Color32>, // submenu_hover_color
            Option<Color32>, // submenu_shortcut_color
            Option<Color32>, // keyboard_selection_color
            Option<Color32>, // submenu_keyboard_selection_color
        ),
    ) -> Self {
        let theme = TitleBarTheme::dark_with_overrides(
            overrides.0,
            overrides.1,
            overrides.2,
            overrides.3,
            overrides.4,
            overrides.5,
            overrides.6,
            overrides.7,
            overrides.8,
            overrides.9,
            overrides.10,
            overrides.11,
            overrides.12,
            overrides.13,
            overrides.14,
            overrides.15,
            overrides.16,
        );
        self.with_theme(theme)
    }

    /// Apply theme mode based on current settings
    ///
    /// This internal method applies the appropriate theme colors based on the
    /// current theme mode. For System mode, it detects the system theme.
    fn apply_theme_mode(&mut self) {
        let theme = match self.theme_mode {
            ThemeMode::Light => TitleBarTheme::light(),
            ThemeMode::Dark => TitleBarTheme::dark(),
            ThemeMode::System => {
                // Detect system theme properly
                if detect_system_dark_mode() {
                    TitleBarTheme::dark()
                } else {
                    TitleBarTheme::light()
                }
            }
        };

        self.background_color = theme.background_color;
        self.hover_color = theme.hover_color;
        self.close_hover_color = theme.close_hover_color;
        self.close_icon_color = theme.close_icon_color;
        self.maximize_icon_color = theme.maximize_icon_color;
        self.restore_icon_color = theme.restore_icon_color;
        self.minimize_icon_color = theme.minimize_icon_color;
        self.title_color = theme.title_color;
        self.menu_text_color = theme.menu_text_color;
        self.menu_text_size = theme.menu_text_size;
        self.menu_hover_color = theme.menu_hover_color;
        self.keyboard_selection_color = theme.keyboard_selection_color;
        // Submenu colors
        self.submenu_background_color = theme.submenu_background_color;
        self.submenu_text_color = theme.submenu_text_color;
        self.submenu_text_size = theme.submenu_text_size;
        self.submenu_hover_color = theme.submenu_hover_color;
        self.submenu_disabled_color = theme.submenu_disabled_color;
        self.submenu_shortcut_color = theme.submenu_shortcut_color;
        self.submenu_border_color = theme.submenu_border_color;
        self.submenu_keyboard_selection_color = theme.submenu_keyboard_selection_color;
    }

    fn apply_theme(&mut self, theme: TitleBarTheme) {
        self.background_color = theme.background_color;
        self.hover_color = theme.hover_color;
        self.close_hover_color = theme.close_hover_color;
        self.close_icon_color = theme.close_icon_color;
        self.maximize_icon_color = theme.maximize_icon_color;
        self.restore_icon_color = theme.restore_icon_color;
        self.minimize_icon_color = theme.minimize_icon_color;
        self.title_color = theme.title_color;
        self.menu_text_color = theme.menu_text_color;
        self.menu_text_size = theme.menu_text_size;
        self.menu_hover_color = theme.menu_hover_color;
        self.keyboard_selection_color = theme.keyboard_selection_color;
        // Submenu colors
        self.submenu_background_color = theme.submenu_background_color;
        self.submenu_text_color = theme.submenu_text_color;
        self.submenu_text_size = theme.submenu_text_size;
        self.submenu_hover_color = theme.submenu_hover_color;
        self.submenu_disabled_color = theme.submenu_disabled_color;
        self.submenu_shortcut_color = theme.submenu_shortcut_color;
        self.submenu_border_color = theme.submenu_border_color;
        self.submenu_keyboard_selection_color = theme.submenu_keyboard_selection_color;
    }

    /// Sync with egui's theme (call this in your app's update loop)
    ///
    /// This method synchronizes the title bar colors with egui's current theme
    /// when the title bar is set to System mode. Call this in your app's update loop.
    ///
    /// # Arguments
    /// * `ctx` - The egui context
    ///
    /// # Examples
    ///
    /// ```rust
    /// fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
    ///     self.title_bar.sync_with_egui_theme(ctx);
    ///     self.title_bar.show(ctx);
    /// }
    /// ```
    pub fn sync_with_egui_theme(&mut self, ctx: &Context) {
        if self.theme_mode == ThemeMode::System {
            let is_dark = ctx.style().visuals.dark_mode;
            let theme = if is_dark {
                TitleBarTheme::dark()
            } else {
                TitleBarTheme::light()
            };

            self.background_color = theme.background_color;
            self.hover_color = theme.hover_color;
            self.close_hover_color = theme.close_hover_color;
            self.close_icon_color = theme.close_icon_color;
            self.maximize_icon_color = theme.maximize_icon_color;
            self.restore_icon_color = theme.restore_icon_color;
            self.minimize_icon_color = theme.minimize_icon_color;
            self.title_color = theme.title_color;
            self.menu_text_color = theme.menu_text_color;
            self.menu_hover_color = theme.menu_hover_color;
        }
    }

    /// Sync with system theme (call this in your app's update loop)
    ///
    /// This method synchronizes the title bar colors with the system theme
    /// when the title bar is set to System mode. It directly queries the OS
    /// for the current theme setting.
    ///
    /// # Examples
    ///
    /// ```rust
    /// fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
    ///     self.title_bar.sync_with_system_theme();
    ///     self.title_bar.show(ctx);
    /// }
    /// ```
    pub fn sync_with_system_theme(&mut self) {
        if self.theme_mode == ThemeMode::System {
            let is_dark = detect_system_dark_mode();
            let theme = if is_dark {
                TitleBarTheme::dark()
            } else {
                TitleBarTheme::light()
            };

            self.background_color = theme.background_color;
            self.hover_color = theme.hover_color;
            self.close_hover_color = theme.close_hover_color;
            self.close_icon_color = theme.close_icon_color;
            self.maximize_icon_color = theme.maximize_icon_color;
            self.restore_icon_color = theme.restore_icon_color;
            self.minimize_icon_color = theme.minimize_icon_color;
            self.title_color = theme.title_color;
            self.menu_text_color = theme.menu_text_color;
            self.menu_hover_color = theme.menu_hover_color;
        }
    }
}
