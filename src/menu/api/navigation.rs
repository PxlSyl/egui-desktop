use egui::{Context, Pos2, Rect, Vec2};

use crate::{TitleBar, titlebar::render_bar::title_bar_height};

impl TitleBar {
    /// Handle keyboard navigation for menus
    ///
    /// This method handles arrow keys, Enter, and Escape for menu navigation.
    ///
    /// # Arguments
    /// * `ctx` - The egui context
    pub fn handle_keyboard_navigation(&mut self, ctx: &Context) {
        let current_time = ctx.input(|i| i.time);

        // Check if Alt key or Ctrl+F2 is pressed to activate menu navigation
        let should_activate = ctx.input(|i| i.modifiers.alt)
            || (ctx.input(|i| i.modifiers.ctrl) && ctx.input(|i| i.key_pressed(egui::Key::F2)));

        if should_activate {
            if !self.keyboard_navigation_active {
                self.keyboard_navigation_active = true;
                self.selected_menu_index = Some(0);
                self.selected_submenu_index = None;
                self.last_keyboard_nav_time = current_time;
            }
        }

        // Handle navigation when active
        if self.keyboard_navigation_active {
            // Handle Escape to close menus
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.keyboard_navigation_active = false;
                self.selected_menu_index = None;
                self.selected_submenu_index = None;
                self.open_submenu = None;
                return;
            }

            // Handle clicks outside menu areas to close menus (but keep keyboard nav active)
            if ctx.input(|i| i.pointer.primary_clicked()) {
                let click_pos = ctx.input(|i| i.pointer.interact_pos()).unwrap_or_default();
                let menu_bar_rect = Rect::from_min_size(
                    Pos2::new(0.0, 0.0),
                    Vec2::new(ctx.content_rect().width(), title_bar_height()),
                );

                // If click is outside menu bar and any submenu is open, close all menus
                if !menu_bar_rect.contains(click_pos) && self.open_submenu.is_some() {
                    self.open_submenu = None;
                    self.force_open_child_subitem = None;
                    self.child_submenu_selections.clear();
                    // Keep keyboard_navigation_active = true (don't disable it)
                }
            }

            // Handle left/right arrow keys for top-level menu navigation
            // Disable only when we're on a highlighted submenu item that has a sidemenu
            let current_highlighted_has_sidemenu = if let Some(open_submenu_index) =
                self.open_submenu
            {
                if let Some(menu_item) = self.menu_items_with_submenus.get(open_submenu_index) {
                    if let Some(selected_index) = self.submenu_selections.get(&open_submenu_index) {
                        if let Some(selected_item) = menu_item.subitems.get(*selected_index) {
                            !selected_item.children.is_empty()
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };

            if !current_highlighted_has_sidemenu {
                if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
                    if let Some(current_index) = self.selected_menu_index {
                        if current_index > 0 {
                            self.selected_menu_index = Some(current_index - 1);
                            self.open_submenu = None;
                            self.selected_submenu_index = None;
                        }
                    }
                }

                if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
                    let total_menus = self.menu_items.len() + self.menu_items_with_submenus.len();
                    if let Some(current_index) = self.selected_menu_index {
                        if current_index < total_menus - 1 {
                            self.selected_menu_index = Some(current_index + 1);
                            self.open_submenu = None;
                            self.selected_submenu_index = None;
                        }
                    }
                }
            }

            // Handle Enter and Space keys - unified logic for all contexts
            if ctx.input(|i| i.key_pressed(egui::Key::Enter))
                || ctx.input(|i| i.key_pressed(egui::Key::Space))
            {
                // Priority 1: If we're in a child sidemenu, handle that first
                if let Some(open_submenu_index) = self.open_submenu {
                    if let Some(child_submenu_index) =
                        self.child_submenu_selections.get(&open_submenu_index)
                    {
                        if let Some(menu_item) =
                            self.menu_items_with_submenus.get(open_submenu_index)
                        {
                            if let Some(child_index) = self.force_open_child_subitem {
                                if let Some(child_item) = menu_item.subitems.get(child_index) {
                                    if let Some(child_subitem) =
                                        child_item.children.get(*child_submenu_index)
                                    {
                                        if child_subitem.enabled {
                                            if let Some(ref callback) = child_subitem.callback {
                                                callback();
                                            }
                                            // Close all submenus after action
                                            self.open_submenu = None;
                                            self.selected_submenu_index = None;
                                            self.force_open_child_subitem = None;
                                            self.child_submenu_selections.clear();
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Priority 2: If we're in a submenu, handle submenu item
                if let Some(open_submenu_index) = self.open_submenu {
                    if self.submenu_just_opened_frame {
                        // Skip if submenu was just opened this frame
                        self.submenu_just_opened_frame = false;
                        return;
                    }
                    if let Some(submenu_index) = self.submenu_selections.get(&open_submenu_index) {
                        if let Some(menu_item) =
                            self.menu_items_with_submenus.get(open_submenu_index)
                        {
                            if let Some(subitem) = menu_item.subitems.get(*submenu_index) {
                                if subitem.enabled && subitem.children.is_empty() {
                                    // Only trigger if it has no children (no sidemenu)
                                    if let Some(ref callback) = subitem.callback {
                                        callback();
                                    }
                                    // Close submenu after action
                                    self.open_submenu = None;
                                    self.submenu_selections.remove(&open_submenu_index);
                                    return;
                                }
                            }
                        }
                    }
                }

                // Priority 3: Handle main menu items
                if let Some(menu_index) = self.selected_menu_index {
                    let total_simple_menus = self.menu_items.len();

                    if menu_index < total_simple_menus {
                        // Simple menu item - trigger callback
                        if let Some((_, callback)) = self.menu_items.get(menu_index) {
                            if let Some(callback) = callback {
                                callback();
                            }
                        }
                    } else {
                        // Menu with submenu
                        let submenu_index = menu_index - total_simple_menus;
                        if let Some(menu_item) = self.menu_items_with_submenus.get(submenu_index) {
                            if !menu_item.subitems.is_empty() {
                                self.open_submenu = Some(submenu_index);
                                self.submenu_selections.insert(submenu_index, 0);
                                // Mark as just opened to avoid immediately activating first item on Enter this frame
                                self.submenu_just_opened_frame = true;
                                // Only auto-open child submenu on keyboard navigation activation
                                if self.keyboard_navigation_active {
                                    if let Some(first_item) = menu_item.subitems.get(0) {
                                        if !first_item.children.is_empty() {
                                            self.force_open_child_subitem = Some(0);
                                        } else {
                                            self.force_open_child_subitem = None;
                                        }
                                    } else {
                                        self.force_open_child_subitem = None;
                                    }
                                } else {
                                    // Mouse-driven open: do not auto-open a child side menu
                                    self.force_open_child_subitem = None;
                                }
                            }
                        }
                    }
                }
            }

            // Handle up/down/left/right keys for submenu navigation and side menus
            if let Some(open_submenu_index) = self.open_submenu {
                if let Some(menu_item) = self.menu_items_with_submenus.get(open_submenu_index) {
                    // Handle up/down navigation in main submenu ONLY if no child sidemenu is open
                    // Child submenu navigation is handled separately below
                    if self.force_open_child_subitem.is_none()
                        && !self
                            .child_submenu_selections
                            .contains_key(&open_submenu_index)
                    {
                        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                            if let Some(current_submenu_index) =
                                self.submenu_selections.get(&open_submenu_index).copied()
                            {
                                if current_submenu_index > 0 {
                                    self.submenu_selections
                                        .insert(open_submenu_index, current_submenu_index - 1);
                                }
                            }
                        }

                        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                            if let Some(current_submenu_index) =
                                self.submenu_selections.get(&open_submenu_index).copied()
                            {
                                if current_submenu_index < menu_item.subitems.len() - 1 {
                                    self.submenu_selections
                                        .insert(open_submenu_index, current_submenu_index + 1);
                                }
                            }
                        }
                    }

                    // Right arrow on a submenu item that has children -> force-open child sidemenu
                    if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
                        if let Some(current_submenu_index) =
                            self.submenu_selections.get(&open_submenu_index)
                        {
                            if let Some(current_item) =
                                menu_item.subitems.get(*current_submenu_index)
                            {
                                if !current_item.children.is_empty() {
                                    // Flag to force-open the child submenu in the renderer
                                    self.force_open_child_subitem = Some(*current_submenu_index);
                                    // Prevent immediate activation this frame
                                    self.submenu_just_opened_frame = true;
                                }
                            }
                        }
                    }

                    // Left arrow: back out of child sidemenu first; if none, close this submenu
                    if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
                        if self.force_open_child_subitem.is_some() {
                            self.force_open_child_subitem = None;
                            self.child_submenu_selections.remove(&open_submenu_index);
                        } else if self.keyboard_navigation_active
                            && self
                                .child_submenu_selections
                                .contains_key(&open_submenu_index)
                        {
                            // Close child submenu navigation but keep parent submenu open
                            self.child_submenu_selections.remove(&open_submenu_index);
                        } else {
                            // Close current submenu, keep focus on the parent menu
                            self.open_submenu = None;
                            self.submenu_selections.remove(&open_submenu_index);
                        }
                    }

                    // Handle navigation within child submenu when it's open
                    // Check if any child submenu is currently open (either forced or hovered)
                    let mut active_child_index = self.force_open_child_subitem;

                    // If no forced child, check if we should enable child navigation
                    // This works for mouse hover, but NOT for keyboard navigation (keyboard needs explicit right arrow)
                    if active_child_index.is_none() && !self.keyboard_navigation_active {
                        // Find the first submenu item that has children and is currently selected
                        if let Some(selected_index) =
                            self.submenu_selections.get(&open_submenu_index)
                        {
                            if let Some(selected_item) = menu_item.subitems.get(*selected_index) {
                                if !selected_item.children.is_empty() {
                                    active_child_index = Some(*selected_index);
                                }
                            }
                        }
                    }

                    if let Some(child_index) = active_child_index {
                        if let Some(child_item) = menu_item.subitems.get(child_index) {
                            if !child_item.children.is_empty() {
                                // Initialize child submenu selection if not set
                                if !self
                                    .child_submenu_selections
                                    .contains_key(&open_submenu_index)
                                {
                                    self.child_submenu_selections.insert(open_submenu_index, 0);
                                }

                                // Handle up/down navigation in child submenu
                                if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                                    if let Some(current_child_index) =
                                        self.child_submenu_selections.get(&open_submenu_index)
                                    {
                                        if *current_child_index > 0 {
                                            self.child_submenu_selections.insert(
                                                open_submenu_index,
                                                current_child_index - 1,
                                            );
                                        }
                                    }
                                }

                                if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                                    if let Some(current_child_index) =
                                        self.child_submenu_selections.get(&open_submenu_index)
                                    {
                                        if *current_child_index < child_item.children.len() - 1 {
                                            self.child_submenu_selections.insert(
                                                open_submenu_index,
                                                current_child_index + 1,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Reset one-frame guards at the end of the keyboard nav cycle
            if self.submenu_just_opened_frame {
                self.submenu_just_opened_frame = false;
            }
            // force_open_child_subitem persists until user navigates away or presses Left; do not reset here
        }
    }
}
