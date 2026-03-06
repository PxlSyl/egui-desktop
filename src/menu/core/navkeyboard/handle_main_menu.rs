use egui::{Context, Key};

use crate::TitleBar;

impl TitleBar {
    /// Get the maximum fitted index (repeated calculation pattern)
    fn get_max_fitted_index(&self) -> usize {
        self.items_fitted.len().saturating_sub(1)
    }

    /// Handle main menu bar navigation (top-level menu items)
    pub fn handle_main_menu_navigation(&mut self, ctx: &Context) -> bool {
        let current_time = ctx.input(|i| i.time);
        let has_overflow = self.items_fitted.len() < self.menu_order.len();

        if self.items_fitted.is_empty() {
            // No visible menus (minimal mode) - open overlay with Down/Right/Enter/Space
            if ctx.input(|i| i.key_pressed(Key::ArrowDown))
                || ctx.input(|i| i.key_pressed(Key::ArrowRight))
                || ctx.input(|i| i.key_pressed(Key::Enter))
                || ctx.input(|i| i.key_pressed(Key::Space))
            {
                if !self.hamburger_menu_open {
                    self.hamburger_menu_open = true;
                    self.hamburger_open_time = Some(current_time);
                    self.reset_all_menu_states();

                    // Select first item in overlay
                    self.overlay_selected_index = Some(0);
                }
                return true;
            }
        } else if has_overflow && self.selected_menu_index.is_none() {
            // Mixed mode (items + overflow) - dots selected (or no selection yet).
            // When dots are selected we must NOT wrap around to index 0 with ArrowRight.
            if ctx.input(|i| i.key_pressed(Key::ArrowRight)) && !self.dots_selected {
                // No selection yet (coming from activation): select first fitted item
                self.selected_menu_index = Some(0);
                return true;
            } else if ctx.input(|i| i.key_pressed(Key::ArrowLeft)) && self.dots_selected {
                // We're on dots, go to the last fitted item and close overlay
                let max_fitted_index = self.get_max_fitted_index();
                self.selected_menu_index = Some(max_fitted_index);
                self.dots_selected = false;
                self.hamburger_menu_open = false; // Close overlay
                return true;
            } else if (ctx.input(|i| i.key_pressed(Key::Enter))
                || ctx.input(|i| i.key_pressed(Key::Space)))
                && self.dots_selected
            {
                // Enter/Space on dots: toggle overflow overlay
                if self.hamburger_menu_open {
                    self.hamburger_menu_open = false;
                    self.overlay_selected_index = None;
                } else {
                    self.hamburger_menu_open = true;
                    self.hamburger_open_time = Some(current_time);
                    let first_overlay_index = self
                        .items_fitted
                        .last()
                        .map(|i| i.saturating_add(1))
                        .unwrap_or(0);
                    self.overlay_selected_index = Some(first_overlay_index);
                }
                return true;
            }
        } else {
            // Check if any submenu is open before allowing main menu navigation
            let has_open_submenu = self.render_state.is_any_menu_open();

            if !has_open_submenu {
                // Normal mode - handle main menu navigation only when no submenu is open
                if ctx.input(|i| i.key_pressed(Key::ArrowRight)) {
                    // Allow main menu navigation only when no submenu is open
                    if !self.dots_selected {
                        if let Some(current_index) = self.selected_menu_index {
                            // Navigation limit: don't go beyond fitted items
                            let max_fitted_index = self.get_max_fitted_index();
                            if current_index < max_fitted_index {
                                self.selected_menu_index = Some(current_index + 1);
                            } else if current_index == max_fitted_index && has_overflow {
                                // We're at the last fitted item, select dots only
                                self.dots_selected = true;
                                self.selected_menu_index = None; // Clear menu selection when on dots
                            }
                        }
                        // Close hamburger overlay when navigating to normal menu
                        self.hamburger_menu_open = false;
                        self.overlay_selected_index = None;
                    }
                    return true;
                }

                if ctx.input(|i| i.key_pressed(Key::ArrowLeft)) {
                    // Allow main menu left navigation only when no submenu is open
                    if !self.dots_selected {
                        if let Some(current_index) = self.selected_menu_index {
                            if current_index > 0 {
                                self.selected_menu_index = Some(current_index - 1);
                            }
                            // On first item (index 0): do nothing, do not wrap to dots
                        }
                        // Close hamburger overlay when navigating to normal menu
                        self.hamburger_menu_open = false;
                        self.overlay_selected_index = None;
                    }
                    return true;
                }
            }

            // Handle Enter/Space when no submenu is open
            let open_submenu_index = self.render_state.get_open_menu_at_depth(0);
            if open_submenu_index.is_none() {
                if ctx.input(|i| i.key_pressed(Key::Enter))
                    || ctx.input(|i| i.key_pressed(Key::Space))
                {
                    if let Some(menu_index) = self.selected_menu_index {
                        // Get the actual menu item from menu_order
                        if let Some(&(is_submenu, actual_index)) = self.menu_order.get(menu_index) {
                            if is_submenu {
                                // Menu with submenu
                                if let Some(menu_item) =
                                    self.menu_items_with_submenus.get(actual_index)
                                {
                                    if !menu_item.subitems.is_empty() {
                                        self.submenu_from_hamburger = false; // Open from bar, not overlay
                                        self.open_menu_recursive(0, actual_index, 0);
                                        self.submenu_just_opened_frame = true;
                                    }
                                }
                            } else {
                                // Simple menu item - trigger callback
                                if let Some((_, callback)) = self.menu_items.get(actual_index) {
                                    if let Some(callback) = callback {
                                        callback();
                                    }
                                }
                            }
                        }
                    }
                    return true;
                }
            }
        }
        false
    }
}
