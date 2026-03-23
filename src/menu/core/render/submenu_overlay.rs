//! Drawing of a single submenu overlay (one level). Used by `submenu.rs` for root and cascades.

use super::submenu::MenuToRender;
use crate::menu::core::states::globals::MENU_STATE;
use crate::{TitleBar, menu::items::MenuItem};
use egui::{
    Align2, Color32, CornerRadius, CursorIcon, FontId, Id, Pos2, Rect, Sense, Stroke, StrokeKind,
    Ui, Vec2,
};

impl TitleBar {
    /// Calculate submenu dimensions without rendering
    pub fn measure_overlay(ui: &mut Ui, menu_item: &MenuItem, menu_text_size: f32) -> (f32, f32) {
        let item_height = 24.0;
        let separator_height = 1.0;
        let padding = 8.0;

        let mut max_width: f32 = 120.0;
        for subitem in &menu_item.subitems {
            let label_width = ui.fonts_mut(|f| {
                f.layout_no_wrap(
                    subitem.label.clone(),
                    FontId::proportional(menu_text_size),
                    Color32::GRAY,
                )
                .size()
                .x
            });
            let shortcut_width = if let Some(ref shortcut) = subitem.shortcut {
                ui.fonts_mut(|f| {
                    f.layout_no_wrap(
                        shortcut.display_string(),
                        FontId::proportional(menu_text_size * 0.9),
                        Color32::GRAY,
                    )
                    .size()
                    .x
                })
            } else {
                0.0
            };
            let total_width = label_width + shortcut_width + padding * 3.0 + 20.0;
            max_width = max_width.max(total_width);
        }

        let total_height = (item_height * menu_item.subitems.len() as f32)
            + (separator_height
                * menu_item
                    .subitems
                    .iter()
                    .filter(|s| s.separator_after)
                    .count() as f32);

        (max_width, total_height)
    }

    /// Render submenu as an overlay at a specific position.
    /// Returns (clicked, was_leaf_item, submenu_rect, pending_leaf_action, open_cascade_sync).
    pub fn render_overlay(
        ui: &mut Ui,
        menu_item: &MenuItem,
        position: Pos2,
        menu_text_size: f32,
        submenu_background_color: Color32,
        submenu_text_color: Color32,
        submenu_hover_color: Color32,
        submenu_shortcut_color: Color32,
        submenu_border_color: Color32,
        submenu_disabled_color: Color32,
        submenu_keyboard_selection_color: Color32,
        keyboard_navigation_active: bool,
        selected_submenu_index: Option<usize>,
        _child_selection: Option<usize>,
        keyboard_open_child_index: Option<usize>,
        parent_submenu_index: usize,
        submenu_from_hamburger: bool,
        hamburger_x: Option<f32>,
        menus_to_render: &mut Vec<MenuToRender>,
        menu_index: usize,
        path_prefix: Vec<usize>,
    ) -> (bool, bool, Rect, Option<(usize, Vec<usize>)>, Option<usize>) {
        let item_height = 24.0;
        let padding = 8.0;
        let separator_height = 1.0;

        let mut max_width: f32 = 120.0;
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
            let total_width = label_width + shortcut_width + padding * 3.0 + 20.0;
            max_width = max_width.max(total_width);
        }

        let total_height = (item_height * menu_item.subitems.len() as f32)
            + (separator_height
                * menu_item
                    .subitems
                    .iter()
                    .filter(|s| s.separator_after)
                    .count() as f32);

        let submenu_rect = Rect::from_min_size(position, Vec2::new(max_width, total_height));
        let content_rect = ui.ctx().content_rect();
        let adjusted_rect = if submenu_rect.max.x > content_rect.max.x {
            Rect::from_min_size(
                Pos2::new(content_rect.max.x - max_width, submenu_rect.min.y),
                submenu_rect.size(),
            )
        } else {
            submenu_rect
        };

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

        let mut current_y = adjusted_rect.min.y;
        let mut item_clicked = false;
        let mut was_leaf_item = false;
        let mut pending_leaf_action: Option<(usize, Vec<usize>)> = None;
        let mut open_cascade_sync: Option<usize> = None;

        let submenu_result = ui.push_id(
            Id::new(format!(
                "submenu_{}_{}",
                menu_item.get_stable_id(),
                parent_submenu_index
            )),
            |ui| {
                let local_item_clicked = false;

                for (i, subitem) in menu_item.subitems.iter().enumerate() {
                    let item_rect = Rect::from_min_size(
                        Pos2::new(adjusted_rect.min.x, current_y),
                        Vec2::new(adjusted_rect.width(), item_height),
                    );

                    let subitem_id = subitem
                        .id
                        .as_ref()
                        .map(|id| format!("subitem_{}_{}_{}", id, parent_submenu_index, i))
                        .unwrap_or_else(|| {
                            format!(
                                "subitem_{}_{}_{}",
                                menu_item.get_stable_id(),
                                parent_submenu_index,
                                i
                            )
                        });

                    let response = ui.interact(item_rect, Id::new(subitem_id), Sense::click());

                    let is_keyboard_selected =
                        keyboard_navigation_active && selected_submenu_index == Some(i);

                    if (response.hovered() || is_keyboard_selected) && subitem.enabled {
                        let highlight_color = if is_keyboard_selected {
                            submenu_keyboard_selection_color
                        } else {
                            submenu_hover_color
                        };
                        ui.painter()
                            .rect_filled(item_rect, CornerRadius::same(2), highlight_color);
                        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                    }

                    let text_color = if is_keyboard_selected {
                        Color32::WHITE
                    } else if subitem.enabled {
                        submenu_text_color
                    } else {
                        submenu_disabled_color
                    };

                    ui.painter().text(
                        Pos2::new(item_rect.min.x + padding, item_rect.center().y),
                        Align2::LEFT_CENTER,
                        &subitem.label,
                        FontId::proportional(menu_text_size),
                        text_color,
                    );

                    if !subitem.children.is_empty() {
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

                    let mut open_child = false;
                    if subitem.enabled && !subitem.children.is_empty() {
                        let menu_key =
                            format!("{}_{}", menu_item.get_stable_id(), parent_submenu_index);

                        if response.clicked() {
                            if let Ok(mut state) = MENU_STATE.lock() {
                                state.insert(menu_key, i);
                            }
                            open_child = true;
                            open_cascade_sync = Some(i);
                        } else {
                            if let Ok(state) = MENU_STATE.lock() {
                                if let Some(&opened_idx) = state.get(&menu_key) {
                                    if opened_idx == i {
                                        open_child = true;
                                    }
                                }
                            }
                        }
                    }

                    if response.clicked() && subitem.enabled && subitem.children.is_empty() {
                        let mut path = path_prefix.clone();
                        path.push(i);
                        pending_leaf_action = Some((menu_index, path));
                        item_clicked = true;
                        was_leaf_item = true;
                        let menu_key =
                            format!("{}_{}", menu_item.get_stable_id(), parent_submenu_index);
                        if let Ok(mut state) = MENU_STATE.lock() {
                            state.remove(&menu_key);
                        }
                    }

                    if open_child
                        || (keyboard_navigation_active && keyboard_open_child_index == Some(i))
                    {
                        let child_position = if submenu_from_hamburger {
                            let child_y = position.y + (i as f32 * 24.0);
                            let child_x = if let Some(hamb_x) = hamburger_x {
                                position.x.max(hamb_x)
                            } else {
                                position.x
                            };
                            Pos2::new(child_x, child_y)
                        } else {
                            Pos2::new(item_rect.max.x, item_rect.min.y)
                        };

                        let content_rect = ui.ctx().content_rect();

                        let child_menu = MenuItem {
                            label: format!("{}_child", menu_item.label),
                            subitems: subitem.children.clone(),
                            enabled: true,
                            id: Some(format!(
                                "{}_child_{}_{}",
                                menu_item.get_stable_id(),
                                parent_submenu_index,
                                i
                            )),
                        };

                        let (child_menu_width, _) =
                            Self::measure_overlay(ui, &child_menu, menu_text_size);

                        let adjusted_child_position = if submenu_from_hamburger {
                            if let Some(hamb_x) = hamburger_x {
                                let child_x = child_position.x.max(hamb_x);
                                Pos2::new(child_x, child_position.y)
                            } else {
                                child_position
                            }
                        } else {
                            let screen_right = content_rect.max.x;
                            let menu_right = child_position.x + child_menu_width;

                            if menu_right > screen_right {
                                let overflow = menu_right - screen_right;
                                let max_shift = child_menu_width * 0.8;
                                let shift = overflow.min(max_shift);
                                let min_x = item_rect.min.x + 20.0;
                                let new_x = (child_position.x - shift).max(min_x);
                                Pos2::new(new_x, child_position.y)
                            } else {
                                child_position
                            }
                        };

                        let mut child_path = path_prefix.clone();
                        child_path.push(i);
                        menus_to_render.push(MenuToRender {
                            menu_item: child_menu,
                            position: adjusted_child_position,
                            parent_index: i,
                            submenu_from_hamburger,
                            path_prefix: child_path,
                        });
                    }

                    current_y += item_height;

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

                local_item_clicked
            },
        );

        item_clicked |= submenu_result.inner;

        let final_clicked = item_clicked && was_leaf_item;
        (
            final_clicked,
            was_leaf_item,
            adjusted_rect,
            pending_leaf_action,
            open_cascade_sync,
        )
    }
}
