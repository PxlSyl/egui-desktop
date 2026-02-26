use egui::Color32;

use crate::TitleBar;

impl TitleBar {
    /// Set the color of menu item text
    ///
    /// # Arguments
    /// * `color` - The menu text color as a Color32
    ///
    /// # Examples
    ///
    /// ```rust
    /// title_bar.with_menu_text_color(Color32::from_rgb(50, 50, 50))
    /// ```
    pub fn with_menu_text_color(mut self, color: Color32) -> Self {
        self.menu_text_color = color;
        self
    }

    /// Set the hover color for menu items
    ///
    /// This color is used when hovering over menu items in the title bar.
    ///
    /// # Arguments
    /// * `color` - The menu hover color as a Color32
    ///
    /// # Examples
    ///
    /// ```rust
    /// title_bar.with_menu_hover_color(Color32::from_rgb(220, 220, 220))
    /// ```
    pub fn with_menu_hover_color(mut self, color: Color32) -> Self {
        self.menu_hover_color = color;
        self
    }

    /// Set the font size of menu item text
    ///
    /// # Arguments
    /// * `size` - The font size in points
    ///
    /// # Examples
    ///
    /// ```rust
    /// title_bar.with_menu_text_size(14.0)
    /// ```
    pub fn with_menu_text_size(mut self, size: f32) -> Self {
        self.menu_text_size = size;
        self
    }
}
