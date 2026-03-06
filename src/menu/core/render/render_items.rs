//! Menu bar rendering: layout, bar items, overflow (dots/hamburger), and main orchestration.

use egui::{Context, Pos2, Rect, Sense, Ui, Vec2};
use std::sync::atomic::Ordering;

use crate::titlebar::render_bar::title_bar_height;
use crate::{
    TitleBar,
    menu::core::states::globals::{MENU_STATE, SUBMENU_CLICK_COUNTER},
};

impl TitleBar {
    /// Render menu items using native-style rendering with responsive behavior
    ///
    /// This method renders menu items as clickable text areas with native-style behavior,
    /// similar to how native applications handle menu bars. Supports both simple menu items
    /// and menu items with submenus. When responsive mode is enabled, automatically adapts
    /// the layout based on available width.
    ///
    /// # Arguments
    /// * `ui` - The egui UI context
    /// * `ctx` - The egui context for interactions
    pub fn render_menu_items(&mut self, ui: &mut Ui, ctx: &Context) {
        self.check_keyboard_shortcuts(ctx);
        self.handle_keyboard_navigation(ctx);

        if self.menu_items.is_empty() && self.menu_items_with_submenus.is_empty() {
            return;
        }

        let menu_height = title_bar_height();
        let available_width = ui.available_width();
        let control_buttons_width = Self::calculate_control_buttons_width(self);
        let effective_available_width = available_width - control_buttons_width;

        let unified_items = self.create_unified_items(ui);
        let (items_fitted, needs_overflow, final_overflow_width, is_minimal_mode) =
            self.calculate_overflow_details(&unified_items, effective_available_width);

        self.items_fitted = items_fitted.clone();

        let total_width = unified_items
            .iter()
            .enumerate()
            .filter(|(i, _)| items_fitted.contains(i))
            .map(|(_, (_, w, _))| *w)
            .sum::<f32>()
            + final_overflow_width;

        let available_rect = ui.available_rect_before_wrap();
        let centered_y = available_rect.center().y - menu_height / 2.0;
        let menu_bar_rect = Rect::from_min_size(
            Pos2::new(available_rect.min.x, centered_y),
            Vec2::new(total_width, menu_height),
        );
        let (menu_bar_rect, _) = ui.allocate_exact_size(menu_bar_rect.size(), Sense::click());

        let mut current_x = menu_bar_rect.min.x;
        self.menu_positions.clear();

        for &global_index in &items_fitted {
            let (index, width, is_submenu) = unified_items[global_index];

            let menu_rect = Rect::from_min_size(
                Pos2::new(current_x, menu_bar_rect.center().y - menu_height / 2.0),
                Vec2::new(width, menu_height),
            );

            if is_submenu {
                let menu_item = self.menu_items_with_submenus.get(index).cloned();
                if let Some(menu_item) = menu_item {
                    let item_width =
                        self.render_submenu_item(ui, &menu_item, index, menu_rect, current_x);
                    current_x += item_width;
                }
            } else {
                let label = self.menu_items.get(index).map(|(l, _)| l.clone());
                if let Some(label) = label {
                    let item_width = self.render_simple_menu_item(ui, &label, menu_rect, current_x);
                    current_x += item_width;
                }
            }
        }

        if needs_overflow {
            let overflow_rect = Rect::from_min_size(
                Pos2::new(current_x, menu_bar_rect.center().y - menu_height / 2.0),
                Vec2::new(final_overflow_width, menu_height),
            );

            let response = self.render_overflow_indicator(ui, overflow_rect, is_minimal_mode);

            if response.clicked() {
                if !self.hamburger_menu_open {
                    self.reset_all_menu_states();
                    self.submenu_just_opened_frame = false;

                    if let Ok(mut state) = MENU_STATE.lock() {
                        state.clear();
                    }
                }
                self.hamburger_menu_open = !self.hamburger_menu_open;
                if self.hamburger_menu_open {
                    self.dots_selected = true;
                    self.selected_menu_index = None;
                    self.overlay_selected_index = self
                        .items_fitted
                        .last()
                        .map(|i| i.saturating_add(1))
                        .or(Some(0));
                    self.last_click_id = SUBMENU_CLICK_COUNTER.fetch_add(1, Ordering::Relaxed);
                    self.hamburger_open_time = Some(ui.input(|i| i.time));
                } else {
                    self.overlay_selected_index = None;
                }
            }

            if self.hamburger_menu_open && !self.force_close_overlay {
                self.render_overflow_menu_overlay(ui, overflow_rect);
                self.handle_hamburger_click_outside(ui, overflow_rect, &items_fitted);
            }

            if self.force_close_overlay {
                self.force_close_overlay = false;
            }
        }
    }
}
