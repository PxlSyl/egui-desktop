use egui::{Context, Key};

use crate::TitleBar;

impl TitleBar {
    /// Handle navigation within submenus only (not main menu).
    /// `open_submenu_index`: index into menu_items_with_submenus (used for deferred callback).
    pub fn handle_submenu_navigation_at_depth(
        &mut self,
        ctx: &Context,
        open_submenu_index: usize,
        submenu_item: &crate::menu::items::MenuItem,
    ) -> bool {
        // Get the actual current depth from render state
        let current_depth = self.render_state.get_current_depth();

        // For submenu navigation, we need to navigate at depth-1 (the selection level)
        let navigation_depth = if current_depth > 0 {
            current_depth - 1
        } else {
            // If we're at depth 0, we shouldn't be in this function
            return false;
        };

        // Navigate to the correct items based on navigation depth
        let items = if navigation_depth == 0 {
            // First submenu level - use submenu items
            &submenu_item.subitems
        } else {
            // We're in a nested submenu - need to navigate through hierarchy.
            // Start from the root submenu items, then walk down using the
            // selections from depth 0 up to (navigation_depth - 1). At depth k,
            // the visible items are the children of the item selected at depth k-1.
            let mut current_items = &submenu_item.subitems;

            for d in 0..navigation_depth {
                if let Some(selection) = self.render_state.get_selection_at_depth(d) {
                    if let Some(item) = current_items.get(selection) {
                        current_items = &item.children;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            // Now current_items points to the items at the navigation depth
            current_items
        };

        if items.is_empty() {
            return false;
        }

        // Get current selection for this navigation depth
        let current_selection = self
            .recursive_state
            .get_selection_at_depth(navigation_depth)
            .unwrap_or(0);

        // Arrow down
        if ctx.input(|i| i.key_pressed(Key::ArrowDown)) {
            if current_selection < items.len() - 1 {
                self.update_selection_recursive(navigation_depth, current_selection + 1);
                return true;
            }
        }

        // Arrow up
        if ctx.input(|i| i.key_pressed(Key::ArrowUp)) {
            if current_selection > 0 {
                self.update_selection_recursive(navigation_depth, current_selection - 1);
                return true;
            }
        }

        // Arrow right (go deeper if current item has children)
        if ctx.input(|i| i.key_pressed(Key::ArrowRight)) {
            if let Some(item) = items.get(current_selection) {
                if !item.children.is_empty() {
                    // Keep current selection at this level, then open the cascade
                    self.update_selection_recursive(navigation_depth, current_selection);
                    self.render_state.go_deeper(current_selection);
                    // go_deeper already pushed selection 0 for the new level (first item)
                    // Sync render state -> recursive_state so highlight is on first item of new cascade
                    let new_depth = self.render_state.get_current_depth();
                    for d in 0..new_depth {
                        if let Some(selection) = self.render_state.get_selection_at_depth(d) {
                            self.recursive_state.set_selection_at_depth(d, selection);
                        }
                    }
                    return true;
                }
            }
        }

        // Arrow left (go up)
        if ctx.input(|i| i.key_pressed(Key::ArrowLeft)) {
            if current_depth > 0 {
                self.go_up_recursive();
                return true;
            }
        }

        // Enter or Space: defer callback so we don't hold menu ref while calling &mut self
        if ctx.input(|i| i.key_pressed(Key::Enter)) || ctx.input(|i| i.key_pressed(Key::Space)) {
            if let Some(item) = items.get(current_selection) {
                if item.children.is_empty() && item.enabled {
                    let path: Vec<usize> = (0..=navigation_depth)
                        .map(|d| self.recursive_state.get_selection_at_depth(d).unwrap_or(0))
                        .collect();
                    self.pending_menu_leaf_action = Some((open_submenu_index, path));
                    self.close_all_menus();
                    self.recursive_state.reset();
                    return true;
                }
            }
        }

        false
    }
}
