//! Overflow indicator (dots / hamburger) and overflow menu overlay.

use egui::{
    Align2, Area, CornerRadius, CursorIcon, FontId, Id, Order, Pos2, Rect, Sense, Stroke, StrokeKind,
    Ui, Vec2,
};

use crate::{
    titlebar::{HamburgerStyle, render_bar::title_bar_height},
    TitleBar,
};

impl TitleBar {
    /// Render overflow indicator (hamburger or three dots)
    pub(super) fn render_overflow_indicator(
        &mut self,
        ui: &mut Ui,
        overflow_rect: Rect,
        is_minimal_mode: bool,
    ) -> egui::Response {
        let response = ui.interact(overflow_rect, Id::new("overflow"), Sense::click());

        let is_keyboard_selected = self.keyboard_navigation_active
            && ((self.items_fitted.is_empty() && self.selected_menu_index.is_none())
                || self.dots_selected);

        if response.hovered() || is_keyboard_selected {
            let highlight_color = if is_keyboard_selected {
                self.submenu_keyboard_selection_color
            } else {
                self.menu_hover_color
            };
            ui.painter().rect_filled(
                overflow_rect.expand(2.0),
                CornerRadius::same(2),
                highlight_color,
            );
            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }

        if is_minimal_mode {
            let line_color = if is_keyboard_selected {
                self.get_contrasting_text_color()
            } else {
                self.menu_text_color
            };
            let icon_size = 16.0;

            match self.hamburger_style {
                HamburgerStyle::Static => {
                    self.draw_static_hamburger(
                        ui.painter(),
                        overflow_rect,
                        line_color,
                        icon_size,
                        overflow_rect.center().y,
                    );
                }
                HamburgerStyle::Animated => {
                    let hovered = response.hovered();
                    let pressed = response.is_pointer_button_down_on();
                    let now = ui.input(|i| i.time);

                    let state = &mut self.hamburger_animation_state;
                    let prev_time = state.last_time;
                    let dt = if prev_time == 0.0 {
                        0.0
                    } else {
                        (now - prev_time) as f32
                    };
                    state.last_time = now;

                    let speed = 8.0;
                    let target_hover = if hovered { 1.0 } else { 0.0 };
                    state.hover_t += (target_hover - state.hover_t) * (1.0 - (-speed * dt).exp());

                    let target_press = if pressed { 1.0 } else { 0.0 };
                    state.press_t += (target_press - state.press_t) * (1.0 - (-12.0 * dt).exp());

                    let menu_open_progress = if self.hamburger_menu_open { 1.0 } else { 0.0 };
                    state.progress +=
                        (menu_open_progress - state.progress) * (1.0 - (-12.0 * dt).exp());

                    ui.ctx().request_repaint();

                    Self::draw_animated_hamburger(
                        ui.painter(),
                        overflow_rect,
                        line_color,
                        icon_size,
                        state,
                        overflow_rect.center().y,
                    );
                }
            }
        } else {
            let dot_color = if is_keyboard_selected {
                self.get_contrasting_text_color()
            } else {
                self.menu_text_color
            };
            self.draw_three_dots(ui.painter(), overflow_rect, dot_color);
        }

        response
    }

    /// Handle click outside hamburger menu to close it
    pub(super) fn handle_hamburger_click_outside(
        &mut self,
        ui: &mut Ui,
        overflow_rect: Rect,
        items_fitted: &[usize],
    ) {
        let current_time = ui.input(|i| i.time);
        let should_check_click = if let Some(open_time) = self.hamburger_open_time {
            current_time - open_time > 0.1
        } else {
            false
        };

        let submenu_from_hamburger_open =
            self.submenu_from_hamburger && self.render_state.is_any_menu_open();

        if should_check_click && !submenu_from_hamburger_open && ui.input(|i| i.pointer.any_click())
        {
            let mouse_pos = ui.input(|i| i.pointer.hover_pos());
            if let Some(mouse_pos) = mouse_pos {
                let item_height = 24.0;
                let padding = 8.0;

                let mut overlay_items = Vec::new();
                for (chronological_index, &(is_submenu, index)) in
                    self.menu_order.iter().enumerate()
                {
                    if !items_fitted.contains(&chronological_index) {
                        overlay_items.push((chronological_index, is_submenu, index));
                    }
                }

                let mut max_width: f32 = 120.0;
                for &(_chronological_index, is_submenu, index) in &overlay_items {
                    if is_submenu {
                        if let Some(menu_item) = self.menu_items_with_submenus.get(index) {
                            let label_width = ui.fonts_mut(|f| {
                                f.layout_no_wrap(
                                    menu_item.label.clone(),
                                    FontId::proportional(self.menu_text_size),
                                    self.submenu_text_color,
                                )
                                .size()
                                .x
                            });
                            let total_width = label_width + padding * 3.0 + 20.0;
                            max_width = max_width.max(total_width);
                        }
                    } else {
                        if let Some((label, _)) = self.menu_items.get(index) {
                            let label_width = ui.fonts_mut(|f| {
                                f.layout_no_wrap(
                                    label.clone(),
                                    FontId::proportional(self.menu_text_size),
                                    self.submenu_text_color,
                                )
                                .size()
                                .x
                            });
                            let total_width = label_width + padding * 2.0;
                            max_width = max_width.max(total_width);
                        }
                    }
                }

                let total_height = item_height * overlay_items.len() as f32;

                let overlay_x = overflow_rect.min.x;
                let overlay_rect = Rect::from_min_size(
                    Pos2::new(overlay_x, title_bar_height()),
                    Vec2::new(max_width, total_height),
                );

                let content_rect = ui.ctx().content_rect();
                let adjusted_rect = if overlay_rect.max.x > content_rect.max.x {
                    Rect::from_min_size(
                        Pos2::new(content_rect.max.x - max_width, overlay_rect.min.y),
                        overlay_rect.size(),
                    )
                } else {
                    overlay_rect
                };

                if !adjusted_rect.contains(mouse_pos) {
                    self.hamburger_menu_open = false;
                    self.overlay_selected_index = None;
                    self.dots_selected = false;
                    if self.submenu_from_hamburger {
                        self.reset_all_menu_states();
                        self.submenu_just_opened_frame = false;
                    }
                }
            }
        }
    }

    /// Render overlay menu for overflow items (both "..." and hamburger)
    pub(super) fn render_overflow_menu_overlay(&mut self, ui: &mut Ui, overflow_rect: Rect) {
        let item_height = 24.0;
        let padding = 8.0;

        let overlay_items = self.build_overlay_items();

        let mut max_width: f32 = 120.0;
        for &(_chronological_index, is_submenu, index) in &overlay_items {
            if is_submenu {
                if let Some(menu_item) = self.menu_items_with_submenus.get(index) {
                    let label_width = ui.fonts_mut(|f| {
                        f.layout_no_wrap(
                            menu_item.label.clone(),
                            FontId::proportional(self.menu_text_size),
                            self.submenu_text_color,
                        )
                        .size()
                        .x
                    });
                    let total_width = label_width + padding * 3.0 + 20.0;
                    max_width = max_width.max(total_width);
                }
            } else {
                if let Some((label, _)) = self.menu_items.get(index) {
                    let label_width = ui.fonts_mut(|f| {
                        f.layout_no_wrap(
                            label.clone(),
                            FontId::proportional(self.menu_text_size),
                            self.submenu_text_color,
                        )
                        .size()
                        .x
                    });
                    let total_width = label_width + padding * 2.0;
                    max_width = max_width.max(total_width);
                }
            }
        }

        let total_height = item_height * overlay_items.len() as f32;

        let overlay_rect = Rect::from_min_size(
            Pos2::new(overflow_rect.min.x, title_bar_height()),
            Vec2::new(max_width, total_height),
        );

        let content_rect = ui.ctx().content_rect();
        let adjusted_rect = if overlay_rect.max.x > content_rect.max.x {
            Rect::from_min_size(
                Pos2::new(content_rect.max.x - max_width, overlay_rect.min.y),
                overlay_rect.size(),
            )
        } else {
            overlay_rect
        };

        self.hamburger_overlay_x = Some(adjusted_rect.min.x);

        Area::new(Id::new("hamburger_menu_overlay"))
            .fixed_pos(adjusted_rect.min)
            .order(Order::Background)
            .show(ui.ctx(), |ui| {
                ui.painter().rect_filled(
                    adjusted_rect,
                    CornerRadius::same(4),
                    self.submenu_background_color,
                );
                ui.painter().rect_stroke(
                    adjusted_rect,
                    CornerRadius::same(4),
                    Stroke::new(1.0, self.submenu_border_color),
                    StrokeKind::Outside,
                );

                let mut current_y = adjusted_rect.min.y;
                for &(chronological_index, is_submenu, index) in &overlay_items {
                    let item_rect = Rect::from_min_size(
                        Pos2::new(adjusted_rect.min.x, current_y),
                        Vec2::new(adjusted_rect.width(), item_height),
                    );

                    let response = ui.interact(
                        item_rect,
                        Id::new(format!("hamburger_item_{}", chronological_index)),
                        Sense::click(),
                    );

                    let is_keyboard_selected = self.keyboard_navigation_active
                        && self.hamburger_menu_open
                        && self.overlay_selected_index == Some(chronological_index);

                    if response.clicked() {
                        self.overlay_selected_index = Some(chronological_index);

                        if is_submenu {
                            self.open_menu_recursive(0, index, 0);
                            self.submenu_just_opened_frame = true;
                            self.submenu_from_hamburger = true;
                        } else if let Some((_, callback)) = self.menu_items.get(index) {
                            if let Some(callback) = callback {
                                callback();
                            }
                            self.hamburger_menu_open = false;
                            self.overlay_selected_index = None;
                            self.dots_selected = false;
                        }
                    }

                    if response.hovered() || is_keyboard_selected {
                        let highlight_color = if is_keyboard_selected {
                            self.submenu_keyboard_selection_color
                        } else {
                            self.submenu_hover_color
                        };
                        ui.painter()
                            .rect_filled(item_rect, CornerRadius::same(2), highlight_color);
                        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                    }

                    if is_submenu {
                        if let Some(menu_item) = self.menu_items_with_submenus.get(index) {
                            ui.painter().text(
                                Pos2::new(item_rect.min.x + padding, item_rect.center().y),
                                Align2::LEFT_CENTER,
                                &menu_item.label,
                                FontId::proportional(self.menu_text_size),
                                if is_keyboard_selected {
                                    self.get_contrasting_text_color()
                                } else {
                                    self.submenu_text_color
                                },
                            );

                            let size = self.menu_text_size * 0.6;
                            let half = size * 0.5;
                            let center = Pos2::new(item_rect.max.x - padding, item_rect.center().y);
                            let p1 = Pos2::new(center.x - half, center.y - half);
                            let p2 = center;
                            let p3 = Pos2::new(center.x - half, center.y + half);
                            let stroke = Stroke::new(
                                1.5,
                                if is_keyboard_selected {
                                    self.get_contrasting_text_color()
                                } else {
                                    self.submenu_text_color
                                },
                            );
                            ui.painter().line_segment([p1, p2], stroke);
                            ui.painter().line_segment([p2, p3], stroke);
                        }
                    } else {
                        if let Some((label, _)) = self.menu_items.get(index) {
                            ui.painter().text(
                                Pos2::new(item_rect.min.x + padding, item_rect.center().y),
                                Align2::LEFT_CENTER,
                                label,
                                FontId::proportional(self.menu_text_size),
                                if is_keyboard_selected {
                                    self.get_contrasting_text_color()
                                } else {
                                    self.submenu_text_color
                                },
                            );
                        }
                    }

                    current_y += item_height;
                }
            });
    }
}
