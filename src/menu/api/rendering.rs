use egui::{
    Align2, Color32, Context, CornerRadius, CursorIcon, FontId, Id, Pos2, Rect, Sense, Ui, Vec2,
};
use std::sync::atomic::Ordering;

use crate::TitleBar;

use super::globals::SUBMENU_CLICK_COUNTER;

impl TitleBar {
    /// Render menu items using native-style rendering (similar to Glitchine)
    ///
    /// This method renders menu items as clickable text areas with native-style behavior,
    /// similar to how native applications handle menu bars. Supports both simple menu items
    /// and menu items with submenus.
    ///
    /// # Arguments
    /// * `ui` - The egui UI context
    pub fn render_menu_items(&mut self, ui: &mut Ui, ctx: &Context) {
        // Check for keyboard shortcuts and navigation first
        self.check_keyboard_shortcuts(ctx);
        self.handle_keyboard_navigation(ctx);

        if self.menu_items.is_empty() && self.menu_items_with_submenus.is_empty() {
            return;
        }

        let menu_height = 28.0; // Standard menu height

        // Calculate total width needed for all menus
        let mut total_width = 0.0;
        for (label, _) in &self.menu_items {
            let label_width = ui.fonts_mut(|f| {
                f.layout_no_wrap(
                    label.clone(),
                    FontId::proportional(self.menu_text_size),
                    self.menu_text_color,
                )
                .size()
                .x
            }) + 16.0;
            total_width += label_width;
        }
        for menu_item in &self.menu_items_with_submenus {
            let label_width = ui.fonts_mut(|f| {
                f.layout_no_wrap(
                    menu_item.label.clone(),
                    FontId::proportional(self.menu_text_size),
                    self.menu_text_color,
                )
                .size()
                .x
            }) + 16.0;
            total_width += label_width;
        }

        // Allocate space for the entire menu bar
        let (menu_bar_rect, _) =
            ui.allocate_exact_size(egui::Vec2::new(total_width, menu_height), Sense::click());

        let mut current_x = menu_bar_rect.min.x;

        // Clear and rebuild menu positions
        self.menu_positions.clear();

        // Render simple menu items
        for (index, (label, callback)) in self.menu_items.iter().enumerate() {
            let label_width = ui.fonts_mut(|f| {
                f.layout_no_wrap(
                    label.clone(),
                    FontId::proportional(self.menu_text_size),
                    self.menu_text_color,
                )
                .size()
                .x
            }) + 16.0;

            // Store the position of this menu item
            self.menu_positions.push(current_x);

            // Create individual menu rect
            let menu_rect = Rect::from_min_size(
                Pos2::new(current_x, menu_bar_rect.min.y),
                Vec2::new(label_width, menu_height),
            );

            // Interact with the menu area
            let response = ui.interact(
                menu_rect,
                Id::new(format!("menu_{}", label)),
                Sense::click(),
            );

            // Check if this menu item is selected by keyboard navigation
            let is_keyboard_selected =
                self.keyboard_navigation_active && self.selected_menu_index == Some(index);

            // Handle hover effect or keyboard selection (render background first)
            if response.hovered() || is_keyboard_selected {
                let highlight_color = if is_keyboard_selected {
                    // Use configurable keyboard selection color
                    self.keyboard_selection_color
                } else {
                    self.menu_hover_color
                };
                ui.painter()
                    .rect_filled(menu_rect, CornerRadius::same(2), highlight_color);
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }

            // Render menu text centered (always rendered on top)
            let text_color = if is_keyboard_selected {
                Color32::WHITE // White text on keyboard selection background
            } else {
                self.menu_text_color
            };

            ui.painter().text(
                menu_rect.center(),
                Align2::CENTER_CENTER,
                label,
                FontId::proportional(self.menu_text_size),
                text_color,
            );

            // Handle click
            if response.clicked() {
                if let Some(callback) = callback {
                    callback();
                }
            }

            // Move to next menu position
            current_x += label_width;
        }

        // Render menu items with submenus
        for (index, menu_item) in self.menu_items_with_submenus.iter().enumerate() {
            let label_width = ui.fonts_mut(|f| {
                f.layout_no_wrap(
                    menu_item.label.clone(),
                    FontId::proportional(self.menu_text_size),
                    self.menu_text_color,
                )
                .size()
                .x
            }) + 16.0;

            // Store the position of this menu item (offset by simple menu count)
            self.menu_positions.push(current_x);

            // Create individual menu rect
            let menu_rect = Rect::from_min_size(
                Pos2::new(current_x, menu_bar_rect.min.y),
                Vec2::new(label_width, menu_height),
            );

            // Interact with the menu area
            let response = ui.interact(
                menu_rect,
                Id::new(format!("submenu_{}", menu_item.label)),
                Sense::click(),
            );

            // Check if this menu item is selected by keyboard navigation
            let menu_index = self.menu_items.len() + index;
            let is_keyboard_selected =
                self.keyboard_navigation_active && self.selected_menu_index == Some(menu_index);

            // Handle hover effect or keyboard selection
            if response.hovered() || is_keyboard_selected {
                let highlight_color = if is_keyboard_selected {
                    // Use configurable keyboard selection color
                    self.keyboard_selection_color
                } else {
                    self.menu_hover_color
                };
                ui.painter()
                    .rect_filled(menu_rect, CornerRadius::same(2), highlight_color);
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }

            // Handle click to toggle submenu
            if response.clicked() {
                // Toggle submenu: close if same, open if different
                if self.open_submenu == Some(index) {
                    self.open_submenu = None;
                    self.submenu_just_opened_frame = false;
                } else {
                    self.open_submenu = Some(index);
                    self.submenu_just_opened_frame = true;
                    // Generate unique click ID
                    self.last_click_id = SUBMENU_CLICK_COUNTER.fetch_add(1, Ordering::Relaxed);
                }
            }

            // Render menu text centered (always rendered on top)
            let text_color = if is_keyboard_selected {
                Color32::WHITE // White text on keyboard selection background
            } else if menu_item.enabled {
                self.menu_text_color
            } else {
                Color32::from_rgb(150, 150, 150) // Disabled color
            };

            ui.painter().text(
                menu_rect.center(),
                Align2::CENTER_CENTER,
                &menu_item.label,
                FontId::proportional(self.menu_text_size),
                text_color,
            );

            // Move to next menu position
            current_x += label_width;
        }
    }
}
