use egui::Context;

use crate::TitleBar;

impl TitleBar {
    /// Check for keyboard shortcuts and trigger callbacks
    ///
    /// This method should be called before rendering menus to handle keyboard shortcuts.
    ///
    /// # Arguments
    /// * `ctx` - The egui context
    pub fn check_keyboard_shortcuts(&mut self, ctx: &Context) {
        // Check menu items with submenus
        for menu_item in &self.menu_items_with_submenus {
            for subitem in &menu_item.subitems {
                if let Some(ref shortcut) = subitem.shortcut {
                    if shortcut.just_pressed(ctx) && subitem.enabled {
                        if let Some(ref callback) = subitem.callback {
                            callback();
                        }
                    }
                }
            }
        }
    }
}
