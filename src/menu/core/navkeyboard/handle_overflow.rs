use egui::{Context, Key};

use crate::TitleBar;

impl TitleBar {
    /// Handle keyboard navigation inside the overflow overlay (dots or hamburger: Up/Down, Enter/Space/Right to open submenus).
    pub fn handle_overflow_overlay(&mut self, ctx: &Context) -> bool {
        let overlay_items = self.build_overlay_items();
        if overlay_items.is_empty() {
            return false;
        }
        let current_chrono = self.overlay_selected_index.unwrap_or(0);
        let pos = overlay_items
            .iter()
            .position(|(c, _, _)| *c == current_chrono)
            .unwrap_or(0);

        // Arrow down
        if ctx.input(|i| i.key_pressed(Key::ArrowDown)) {
            if pos < overlay_items.len().saturating_sub(1) {
                self.overlay_selected_index = Some(overlay_items[pos + 1].0);
                return true;
            }
        }

        // Arrow up
        if ctx.input(|i| i.key_pressed(Key::ArrowUp)) {
            if pos > 0 {
                self.overlay_selected_index = Some(overlay_items[pos - 1].0);
                return true;
            }
        }

        // Arrow left: close overlay, stay on dots
        if ctx.input(|i| i.key_pressed(Key::ArrowLeft)) {
            self.hamburger_menu_open = false;
            self.overlay_selected_index = None;
            return true;
        }

        // Enter / Space / Right: activate item or open submenu
        if ctx.input(|i| i.key_pressed(Key::Enter))
            || ctx.input(|i| i.key_pressed(Key::Space))
            || ctx.input(|i| i.key_pressed(Key::ArrowRight))
        {
            let &(_chronological_index, is_submenu, index) =
                overlay_items.get(pos).unwrap_or(&overlay_items[0]);
            if is_submenu {
                if let Some(menu_item) = self.menu_items_with_submenus.get(index) {
                    if !menu_item.subitems.is_empty() {
                        self.open_menu_recursive(0, index, 0);
                        self.submenu_just_opened_frame = true;
                        self.submenu_from_hamburger = true;
                    }
                }
            } else {
                if let Some((_, callback)) = self.menu_items.get(index) {
                    if let Some(callback) = callback {
                        callback();
                    }
                }
                self.hamburger_menu_open = false;
                self.overlay_selected_index = None;
            }
            return true;
        }

        false
    }
}
