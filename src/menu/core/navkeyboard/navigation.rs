use egui::{Context, Key};

use crate::TitleBar;

impl TitleBar {
    /// Check if menu navigation should be activated
    fn should_activate_navigation(&self, ctx: &Context) -> bool {
        ctx.input(|i| i.modifiers.alt)
            || (ctx.input(|i| i.modifiers.ctrl) && ctx.input(|i| i.key_pressed(Key::F2)))
    }

    /// Activate menu navigation based on current state
    fn activate_navigation(&mut self, ctx: &Context) {
        let current_time = ctx.input(|i| i.time);

        self.keyboard_navigation_active = true;
        // Check if we have any fitted items (visible menus)
        if !self.items_fitted.is_empty() {
            // We have visible menus, select the first one
            self.selected_menu_index = Some(0);
            self.hamburger_menu_open = false; // Ensure hamburger is closed
        } else {
            // No visible menus (minimal mode), activate hamburger navigation
            self.selected_menu_index = None; // No regular menu selected
            self.hamburger_menu_open = true; // Open hamburger menu
            self.hamburger_open_time = Some(current_time);
        }
        self.reset_all_menu_states();
        self.dots_selected = false; // Reset dots selection
        self.overlay_selected_index = None; // Reset overlay selection
        // In pure hamburger mode, select first overlay item so Up/Down and Enter work immediately
        if self.items_fitted.is_empty() && self.hamburger_menu_open {
            self.overlay_selected_index = Some(0);
        }
        self.last_keyboard_nav_time = current_time;
    }

    /// Handle keyboard navigation for menus
    pub fn handle_keyboard_navigation(&mut self, ctx: &Context) {
        // Check if Alt key or Ctrl+F2 is pressed to activate menu navigation
        if self.should_activate_navigation(ctx) {
            if !self.keyboard_navigation_active {
                self.activate_navigation(ctx);
            }
        }

        // Handle navigation when active
        if self.keyboard_navigation_active {
            // Priority 1: Handle submenu navigation first (when submenu is open)
            let open_submenu_index = self.render_state.get_open_menu_at_depth(0);
            if let Some(open_submenu_index) = open_submenu_index {
                let menu_item = self
                    .menu_items_with_submenus
                    .get(open_submenu_index)
                    .cloned();
                if let Some(ref menu_item) = menu_item {
                    if self.handle_submenu_navigation_at_depth(ctx, open_submenu_index, menu_item) {
                        self.invoke_pending_menu_leaf_action();
                        return;
                    }
                }
            }

            // Priority 2: Overflow overlay navigation when overlay is open and no submenu
            if self.hamburger_menu_open
                && self.overlay_selected_index.is_some()
                && self.render_state.get_open_menu_at_depth(0).is_none()
            {
                if self.handle_overflow_overlay(ctx) {
                    return;
                }
            }

            // Handle Escape to close menus
            if ctx.input(|i| i.key_pressed(Key::Escape)) {
                self.keyboard_navigation_active = false;
                self.selected_menu_index = None;
                self.reset_all_menu_states();
                self.hamburger_menu_open = false;
                self.dots_selected = false;
                self.overlay_selected_index = None;
                self.recursive_state.reset();
                return;
            }

            // Handle main menu navigation
            if self.handle_main_menu_navigation(ctx) {
                return;
            }
        }
    }
}
