/// Synchronisation utilities between recursive navigation state and recursive render state
impl crate::titlebar::main::TitleBar {
    /// Synchronize navigation state from render state
    /// Call this after any direct render state manipulation
    pub fn sync_navigation_from_render(&mut self) {
        let depth = self.render_state.get_current_depth();

        // Clear navigation state
        self.recursive_state.reset();

        // Copy render path to navigation state
        for d in 0..depth {
            if let Some(_menu_index) = self.render_state.get_open_menu_at_depth(d) {
                if let Some(selection) = self.render_state.get_selection_at_depth(d) {
                    self.recursive_state.set_selection_at_depth(d, selection);
                }
            }
        }
    }

    /// Open menu at specific depth using recursive system
    pub fn open_menu_recursive(&mut self, depth: usize, menu_index: usize, selection: usize) {
        self.render_state
            .open_menu_at_depth(depth, menu_index, selection);
        self.sync_navigation_from_render();
    }

    /// Close menu from specific depth using recursive system
    pub fn close_menu_from_depth(&mut self, depth: usize) {
        self.render_state.close_menus_from_depth(depth);
        self.sync_navigation_from_render();
    }

    /// Go deeper in menu hierarchy using recursive system
    pub fn go_deeper_recursive(&mut self, item_index: usize) {
        // Get current depth to determine what we're opening
        let current_depth = self.render_state.get_current_depth();

        if current_depth == 0 {
            // We're at main menu level, open submenu
            if let Some(menu_item) = self.menu_items_with_submenus.get(item_index) {
                if !menu_item.subitems.is_empty() {
                    self.render_state.go_deeper(item_index);
                }
            }
        } else {
            // We're in a submenu, need to navigate through the hierarchy
            // Start from the root menu
            if let Some(root_menu_index) = self.render_state.get_open_menu_at_depth(0) {
                if let Some(root_menu) = self.menu_items_with_submenus.get(root_menu_index) {
                    let mut current_items = &root_menu.subitems;

                    // Navigate through intermediate levels
                    for depth in 1..current_depth {
                        if let Some(selection) = self.render_state.get_selection_at_depth(depth) {
                            if let Some(item) = current_items.get(selection) {
                                current_items = &item.children;
                            } else {
                                return;
                            }
                        }
                    }

                    // Now try to open the child submenu at current level
                    if let Some(selected_item) = current_items.get(item_index) {
                        if !selected_item.children.is_empty() {
                            // Open child submenu at this level
                            self.render_state.go_deeper(item_index);
                        }
                    }
                }
            }
        }

        self.sync_navigation_from_render();
    }

    /// Go up in menu hierarchy using recursive system
    pub fn go_up_recursive(&mut self) -> bool {
        let result = self.render_state.go_up();
        self.sync_navigation_from_render();
        result
    }

    /// Update selection at specific depth using recursive system
    pub fn update_selection_recursive(&mut self, depth: usize, selection: usize) {
        self.render_state
            .update_selection_at_depth(depth, selection);
        self.sync_navigation_from_render();
    }

    /// Reset all menu states using recursive system
    pub fn reset_all_menu_states(&mut self) {
        self.render_state.reset();
        self.recursive_state.reset();
    }
}
