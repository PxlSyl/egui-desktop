//! Rendering of individual menu bar items (simple and submenu).

use egui::{Align2, CornerRadius, CursorIcon, FontId, Id, Rect, Sense, Ui};
use std::sync::atomic::Ordering;

use crate::{
    menu::core::states::globals::{MENU_STATE, SUBMENU_CLICK_COUNTER},
    MenuItem, TitleBar,
};

impl TitleBar {
    /// Render a simple menu item
    pub(super) fn render_simple_menu_item(
        &mut self,
        ui: &mut Ui,
        label: &str,
        menu_rect: Rect,
        current_x: f32,
    ) -> f32 {
        self.menu_positions.push(current_x);

        let response = ui.interact(
            menu_rect,
            Id::new(format!("menu_{}", label)),
            Sense::click(),
        );

        // Check if this menu item is selected by keyboard navigation
        let is_keyboard_selected = self.keyboard_navigation_active
            && if let Some(selected_index) = self.selected_menu_index {
                // Find the chronological index of this menu item in menu_order
                let mut chronological_index = 0;
                for &(is_submenu, index) in &self.menu_order {
                    if !is_submenu
                        && index
                            == self
                                .menu_items
                                .iter()
                                .position(|(l, _)| l == label)
                                .unwrap_or(0)
                    {
                        break;
                    }
                    chronological_index += 1;
                }
                selected_index == chronological_index
            } else {
                false
            };

        if response.hovered() || is_keyboard_selected {
            let highlight_color = if is_keyboard_selected {
                self.submenu_keyboard_selection_color
            } else {
                self.menu_hover_color
            };
            ui.painter()
                .rect_filled(menu_rect, CornerRadius::same(2), highlight_color);
            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }

        ui.painter().text(
            menu_rect.center(),
            Align2::CENTER_CENTER,
            label,
            FontId::proportional(self.menu_text_size),
            if is_keyboard_selected {
                self.get_contrasting_text_color()
            } else {
                self.menu_text_color
            },
        );

        if response.clicked() {
            if self.render_state.is_any_menu_open() {
                self.close_menu_from_depth(0);
                self.submenu_just_opened_frame = false;
            }
            let mut chronological_index = 0;
            for &(is_submenu, index) in &self.menu_order {
                if !is_submenu && self.menu_items.get(index).map(|(l, _)| l.as_str()) == Some(label)
                {
                    self.selected_menu_index = Some(chronological_index);
                    break;
                }
                chronological_index += 1;
            }
            self.dots_selected = false;
            self.overlay_selected_index = None;

            if let Some((_, callback)) = self.menu_items.iter().find(|(l, _)| l == label) {
                if let Some(callback) = callback {
                    callback();
                }
            }
        }

        menu_rect.width()
    }

    /// Render a submenu item
    pub(super) fn render_submenu_item(
        &mut self,
        ui: &mut Ui,
        menu_item: &MenuItem,
        submenu_index: usize,
        menu_rect: Rect,
        current_x: f32,
    ) -> f32 {
        self.menu_positions.push(current_x);

        let response = ui.interact(
            menu_rect,
            Id::new(format!("submenu_{}", menu_item.get_stable_id())),
            Sense::click(),
        );

        let is_keyboard_selected = self.keyboard_navigation_active
            && if let Some(selected_index) = self.selected_menu_index {
                let mut chronological_index = 0;
                for &(is_submenu, index) in &self.menu_order {
                    if is_submenu && index == submenu_index {
                        break;
                    }
                    chronological_index += 1;
                }
                selected_index == chronological_index
            } else {
                false
            };

        if response.hovered() || is_keyboard_selected {
            let highlight_color = if is_keyboard_selected {
                self.submenu_keyboard_selection_color
            } else {
                self.menu_hover_color
            };
            ui.painter()
                .rect_filled(menu_rect, CornerRadius::same(2), highlight_color);
            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }

        ui.painter().text(
            menu_rect.center(),
            Align2::CENTER_CENTER,
            &menu_item.label,
            FontId::proportional(self.menu_text_size),
            if is_keyboard_selected {
                self.get_contrasting_text_color()
            } else {
                self.menu_text_color
            },
        );

        if response.clicked() {
            for (chronological_index, &(is_submenu, index)) in self.menu_order.iter().enumerate() {
                if is_submenu && index == submenu_index {
                    self.selected_menu_index = Some(chronological_index);
                    break;
                }
            }
            self.dots_selected = false;
            self.overlay_selected_index = None;

            if self.render_state.is_any_menu_open()
                && self.render_state.get_open_menu_at_depth(0) == Some(submenu_index)
            {
                self.close_menu_from_depth(0);
                self.submenu_just_opened_frame = false;
            } else {
                self.hamburger_menu_open = false;
                self.open_menu_recursive(0, submenu_index, 0);
                self.submenu_just_opened_frame = true;
                self.submenu_from_hamburger = false;
                self.last_click_id = SUBMENU_CLICK_COUNTER.fetch_add(1, Ordering::Relaxed);

                if let Ok(mut state) = MENU_STATE.lock() {
                    state.clear();
                }
            }
        }

        menu_rect.width()
    }
}
