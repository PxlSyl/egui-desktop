use egui::{
    Align2, Area, Color32, Context, CornerRadius, CursorIcon, FontId, Id, Order, Pos2, Rect, Sense,
    Stroke, StrokeKind, Ui, Vec2,
};
use std::{cell::RefCell, sync::atomic::Ordering};

use crate::{TitleBar, menu::items::MenuItem, titlebar::render_bar::title_bar_height};

use super::globals::SUBMENU_CLICK_COUNTER;

impl TitleBar {
    /// Render the currently open submenu as an overlay
    pub fn render_open_submenu(&mut self, ctx: &Context) {
        if let Some(open_index) = self.open_submenu {
            if let Some(menu_item) = self.menu_items_with_submenus.get(open_index) {
                if !menu_item.subitems.is_empty() {
                    // Use reference instead of clone to preserve callbacks
                    let menu_text_size = self.menu_text_size;
                    let submenu_background_color = self.submenu_background_color;
                    let submenu_text_color = self.submenu_text_color;
                    let submenu_hover_color = self.submenu_hover_color;
                    let submenu_shortcut_color = self.submenu_shortcut_color;
                    let submenu_border_color = self.submenu_border_color;
                    let submenu_keyboard_selection_color = self.submenu_keyboard_selection_color;
                    let keyboard_navigation_active = self.keyboard_navigation_active;
                    let submenu_selections = self.submenu_selections.clone();
                    let force_open_child_subitem = self.force_open_child_subitem;
                    let child_submenu_selections = self.child_submenu_selections.clone();

                    // Calculate submenu position using stored menu positions
                    let submenu_x = if let Some(menu_x) =
                        self.menu_positions.get(self.menu_items.len() + open_index)
                    {
                        *menu_x
                    } else {
                        // Fallback to old calculation if positions not available
                        let mut menu_x = 16.0 + 20.0 + 8.0; // icon + title space + padding
                        for i in 0..open_index {
                            if let Some(item) = self.menu_items_with_submenus.get(i) {
                                // Fallback: use character count approximation for positioning
                                menu_x += item.label.len() as f32 * (menu_text_size * 0.6) + 16.0;
                            }
                        }
                        menu_x
                    };
                    let submenu_position = Pos2::new(submenu_x, title_bar_height()); // Below title bar

                    // Use a RefCell to allow modification from within the closure
                    let item_clicked = RefCell::new(false);

                    // Create a full-screen area to capture clicks outside
                    Area::new(egui::Id::new(format!("submenu_overlay_{}", open_index)))
                        .fixed_pos(Pos2::ZERO)
                        .order(Order::Foreground)
                        .show(ctx, |ui| {
                            // Render the submenu at the calculated position
                            let clicked = Self::render_submenu_overlay_static(
                                ui,
                                menu_item, // Pass reference instead of clone
                                submenu_position,
                                menu_text_size,
                                submenu_background_color,
                                submenu_text_color,
                                submenu_hover_color,
                                submenu_shortcut_color,
                                submenu_border_color,
                                submenu_keyboard_selection_color,
                                keyboard_navigation_active,
                                submenu_selections.get(&open_index).copied(),
                                force_open_child_subitem,
                                child_submenu_selections.get(&open_index).copied(),
                                open_index, // Pass parent submenu index
                            );

                            // Store the click result
                            *item_clicked.borrow_mut() = clicked;
                        });

                    // Close submenu if an item was clicked
                    if *item_clicked.borrow() {
                        self.open_submenu = None;
                        self.submenu_just_opened_frame = false;
                    }

                    // Check for clicks outside the submenu area using input detection
                    if ctx.input(|i| i.pointer.primary_clicked()) {
                        let current_click_id = SUBMENU_CLICK_COUNTER.load(Ordering::Relaxed);
                        let click_pos = ctx.input(|i| i.pointer.interact_pos()).unwrap_or_default();
                        let submenu_rect =
                            Rect::from_min_size(submenu_position, Vec2::new(200.0, 100.0));

                        // Only close if this is a different click than the one that opened the submenu
                        if current_click_id > self.last_click_id {
                            // Close if click is outside submenu and not in menu bar
                            let menu_bar_rect = Rect::from_min_size(
                                Pos2::new(0.0, 0.0),
                                Vec2::new(ctx.content_rect().width(), title_bar_height()),
                            );

                            if !submenu_rect.contains(click_pos)
                                && !menu_bar_rect.contains(click_pos)
                            {
                                // Close all menus but keep keyboard navigation active
                                self.open_submenu = None;
                                self.force_open_child_subitem = None;
                                self.child_submenu_selections.clear();
                                // Keep keyboard_navigation_active = true (don't disable it)
                            }
                        }
                    }

                    // Reset the flag after first frame
                    if self.submenu_just_opened_frame {
                        self.submenu_just_opened_frame = false;
                    }
                }
            }
        }
    }

    /// Render submenu as an overlay at a specific position (static version)
    /// Returns true if an item was clicked
    fn render_submenu_overlay_static(
        ui: &mut Ui,
        menu_item: &MenuItem,
        position: egui::Pos2,
        menu_text_size: f32,
        submenu_background_color: Color32,
        submenu_text_color: Color32,
        submenu_hover_color: Color32,
        submenu_shortcut_color: Color32,
        submenu_border_color: Color32,
        submenu_keyboard_selection_color: Color32,
        keyboard_navigation_active: bool,
        selected_submenu_index: Option<usize>,
        force_open_child_subitem: Option<usize>,
        selected_child_submenu_index: Option<usize>,
        parent_submenu_index: usize,
    ) -> bool {
        // Calculate submenu dimensions
        let item_height = 24.0;
        let padding = 8.0;
        let separator_height = 1.0;

        // Find the maximum width needed
        let mut max_width: f32 = 120.0; // Minimum width
        for subitem in &menu_item.subitems {
            let label_width = ui.fonts_mut(|f| {
                f.layout_no_wrap(
                    subitem.label.clone(),
                    FontId::proportional(menu_text_size),
                    submenu_text_color,
                )
                .size()
                .x
            });
            let shortcut_width = if let Some(ref shortcut) = subitem.shortcut {
                ui.fonts_mut(|f| {
                    f.layout_no_wrap(
                        shortcut.display_string(),
                        FontId::proportional(menu_text_size * 0.9),
                        submenu_shortcut_color,
                    )
                    .size()
                    .x
                })
            } else {
                0.0
            };
            let total_width = label_width + shortcut_width + padding * 3.0 + 20.0; // Extra space for arrow
            max_width = max_width.max(total_width);
        }

        let total_height = (item_height * menu_item.subitems.len() as f32)
            + (separator_height
                * menu_item
                    .subitems
                    .iter()
                    .filter(|s| s.separator_after)
                    .count() as f32);

        // Position submenu
        let submenu_rect = egui::Rect::from_min_size(position, Vec2::new(max_width, total_height));

        // Ensure submenu stays within screen bounds
        let content_rect = ui.ctx().content_rect();
        let adjusted_rect = if submenu_rect.max.x > content_rect.max.x {
            // Move left if it would go off screen
            Rect::from_min_size(
                Pos2::new(content_rect.max.x - max_width, submenu_rect.min.y),
                submenu_rect.size(),
            )
        } else {
            submenu_rect
        };

        // Draw submenu background and border
        ui.painter().rect_filled(
            adjusted_rect,
            CornerRadius::same(4),
            submenu_background_color,
        );
        ui.painter().rect_stroke(
            adjusted_rect,
            CornerRadius::same(4),
            Stroke::new(1.0, submenu_border_color),
            StrokeKind::Outside,
        );

        // Render submenu items
        let mut current_y = adjusted_rect.min.y;
        let mut item_clicked = false;
        for (i, subitem) in menu_item.subitems.iter().enumerate() {
            let item_rect = Rect::from_min_size(
                Pos2::new(adjusted_rect.min.x, current_y),
                Vec2::new(adjusted_rect.width(), item_height),
            );

            // Handle hover effect
            let response = ui.interact(
                item_rect,
                Id::new(format!("subitem_overlay_{}_{}", menu_item.label, i)),
                Sense::click(),
            );

            // Check if this submenu item is selected by keyboard navigation
            // Use main selection if available, otherwise use child selection
            let is_keyboard_selected = keyboard_navigation_active
                && (selected_submenu_index == Some(i)
                    || (selected_submenu_index.is_none()
                        && selected_child_submenu_index == Some(i)));

            if (response.hovered() || is_keyboard_selected) && subitem.enabled {
                let highlight_color = if is_keyboard_selected {
                    // Use configurable keyboard selection color for submenus
                    submenu_keyboard_selection_color
                } else {
                    submenu_hover_color
                };
                ui.painter()
                    .rect_filled(item_rect, CornerRadius::same(2), highlight_color);
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }

            // Render text and shortcut
            let text_color = if is_keyboard_selected {
                Color32::WHITE // White text on keyboard selection background
            } else if subitem.enabled {
                submenu_text_color
            } else {
                Color32::from_rgb(150, 150, 150)
            };

            // Main label (left aligned)
            ui.painter().text(
                Pos2::new(item_rect.min.x + padding, item_rect.center().y),
                Align2::LEFT_CENTER,
                &subitem.label,
                FontId::proportional(menu_text_size),
                text_color,
            );

            // Shortcut or child arrow (right aligned)
            if !subitem.children.is_empty() {
                // Draw a chevron using two line segments for reliable rendering across fonts
                let center = Pos2::new(item_rect.max.x - padding, item_rect.center().y);
                let size = menu_text_size * 0.6;
                let half = size * 0.5;
                let p1 = Pos2::new(center.x - half, center.y - half);
                let p2 = center;
                let p3 = Pos2::new(center.x - half, center.y + half);
                let stroke_color = if is_keyboard_selected {
                    Color32::WHITE
                } else {
                    submenu_text_color
                };
                let stroke = Stroke::new(1.5, stroke_color);
                ui.painter().line_segment([p1, p2], stroke);
                ui.painter().line_segment([p2, p3], stroke);
            } else if let Some(ref shortcut) = subitem.shortcut {
                let shortcut_color = if is_keyboard_selected {
                    Color32::WHITE
                } else {
                    submenu_shortcut_color
                };
                ui.painter().text(
                    Pos2::new(item_rect.max.x - padding, item_rect.center().y),
                    Align2::RIGHT_CENTER,
                    &shortcut.display_string(),
                    FontId::proportional(menu_text_size * 0.9),
                    shortcut_color,
                );
            }

            // Handle click or hover-open for cascading child menus
            // Keep child menu open while the pointer travels from parent row to child (hover corridor)
            // and also while the pointer is inside the child submenu area itself.
            let mut open_child = false;
            if subitem.enabled && !subitem.children.is_empty() {
                if response.hovered() {
                    open_child = true;
                } else if let Some(ptr) = ui.ctx().input(|i| i.pointer.interact_pos()) {
                    // 1) Narrow corridor bridging parent item and child menu
                    let corridor_width = 10.0;
                    let corridor = Rect::from_min_max(
                        Pos2::new(item_rect.max.x, item_rect.min.y - 6.0),
                        Pos2::new(item_rect.max.x + corridor_width, item_rect.max.y + 6.0),
                    );

                    // 2) Approximate child submenu bounds (so moving into it keeps it open)
                    let mut child_max_width: f32 = 120.0; // minimum width
                    let padding = 8.0;
                    let item_height = 24.0;
                    let separator_height = 1.0;
                    for c in &subitem.children {
                        let label_width = ui.fonts_mut(|f| {
                            f.layout_no_wrap(
                                c.label.clone(),
                                FontId::proportional(menu_text_size),
                                submenu_text_color,
                            )
                            .size()
                            .x
                        });
                        let shortcut_width = if let Some(ref s) = c.shortcut {
                            ui.fonts_mut(|f| {
                                f.layout_no_wrap(
                                    s.display_string(),
                                    FontId::proportional(menu_text_size * 0.9),
                                    submenu_shortcut_color,
                                )
                                .size()
                                .x
                            })
                        } else {
                            0.0
                        };
                        let total_width = label_width + shortcut_width + padding * 3.0 + 20.0;
                        child_max_width = child_max_width.max(total_width);
                    }
                    let child_total_height = (item_height * subitem.children.len() as f32)
                        + (separator_height
                            * subitem
                                .children
                                .iter()
                                .filter(|s| s.separator_after)
                                .count() as f32);

                    let mut child_rect = Rect::from_min_size(
                        Pos2::new(item_rect.max.x, item_rect.min.y),
                        Vec2::new(child_max_width, child_total_height),
                    );
                    // Keep child rect on screen if needed
                    let content_rect = ui.ctx().content_rect();
                    if child_rect.max.x > content_rect.max.x {
                        let shift = child_rect.max.x - content_rect.max.x;
                        child_rect = child_rect.translate(Vec2::new(-shift, 0.0));
                    }

                    if corridor.contains(ptr) || child_rect.contains(ptr) {
                        open_child = true;
                    }
                }
            }
            if response.clicked() && subitem.enabled && subitem.children.is_empty() {
                if let Some(ref callback) = subitem.callback {
                    callback();
                }
                item_clicked = true;
            }

            // Render cascading child menu if needed
            // Allow hover to open even in keyboard mode; keyboard can also force-open
            if open_child || (keyboard_navigation_active && force_open_child_subitem == Some(i)) {
                // Initialize child submenu selection if not set (for keyboard navigation)
                if keyboard_navigation_active && selected_child_submenu_index.is_none() {
                    // Initialize the first item as selected for this child submenu
                    // Note: We can't modify title_bar_ref here due to borrowing constraints
                    // This is a limitation of the current approach
                }

                let child_position = Pos2::new(item_rect.max.x, item_rect.min.y);
                let child_menu = MenuItem {
                    label: format!("{}_child", menu_item.label),
                    subitems: subitem.children.clone(),
                    enabled: true,
                };

                // Draw child menu
                let child_clicked = Self::render_submenu_overlay_static(
                    ui,
                    &child_menu,
                    child_position,
                    menu_text_size,
                    submenu_background_color,
                    submenu_text_color,
                    submenu_hover_color,
                    submenu_shortcut_color,
                    submenu_border_color,
                    submenu_keyboard_selection_color,
                    keyboard_navigation_active,
                    None,                         // Child menus don't use parent menu selection
                    None,                         // Child menus don't have forced open items
                    selected_child_submenu_index, // Pass child selection for highlighting
                    parent_submenu_index,         // Pass parent submenu index
                );

                // Propagate child menu click to parent
                if child_clicked {
                    item_clicked = true;
                }
            }

            current_y += item_height;

            // Add separator if needed
            if subitem.separator_after && i < menu_item.subitems.len() - 1 {
                let separator_rect = Rect::from_min_size(
                    Pos2::new(adjusted_rect.min.x + padding, current_y),
                    Vec2::new(adjusted_rect.width() - padding * 2.0, separator_height),
                );
                ui.painter().rect_filled(
                    separator_rect,
                    CornerRadius::same(0),
                    Color32::from_rgb(200, 200, 200),
                );
                current_y += separator_height;
            }
        }

        item_clicked
    }
}
