use egui::Context;

use crate::TitleBar;

impl TitleBar {
    /// Check if the menu is in minimal mode (only hamburger visible)
    pub fn is_in_minimal_mode(&self, ctx: &Context) -> bool {
        let available_width = ctx.content_rect().width();
        let control_buttons_width = Self::calculate_control_buttons_width(self);
        let effective_available_width = available_width - control_buttons_width;

        // Calculate total width needed for all menu items
        let mut total_menu_width = 0.0;
        for &(is_submenu, index) in &self.menu_order {
            let item_width = self.calculate_item_width(is_submenu, index);
            total_menu_width += item_width;
        }

        // Add overflow indicator width
        total_menu_width += 30.0;

        total_menu_width > effective_available_width
    }
}
