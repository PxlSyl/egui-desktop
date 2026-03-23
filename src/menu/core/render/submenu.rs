use egui::{Area, Context, FontId, Id, Order, Pos2, Rect, Vec2};
use std::{cell::RefCell, sync::atomic::Ordering};

use crate::{TitleBar, menu::items::MenuItem, titlebar::render_bar::title_bar_height};

use crate::menu::core::states::globals::{MENU_STATE, SUBMENU_CLICK_COUNTER};

/// Structure to store menu rendering information for proper z-order (used by submenu_overlay).
/// `path_prefix` is the path from root to this overlay (e.g. [1] for first cascade, [1, 0] for nested).
#[derive(Clone)]
pub struct MenuToRender {
    pub menu_item: MenuItem,
    pub position: Pos2,
    pub parent_index: usize,
    pub submenu_from_hamburger: bool,
    /// Path from root: depth 1 = [i], depth 2 = [i, j], etc. Used for recursive rendering and keyboard filter.
    pub path_prefix: Vec<usize>,
}

impl TitleBar {
    /// Render the currently open submenu as an overlay
    pub fn render_submenu(&mut self, ctx: &Context) {
        if let Some(open_index) = self.render_state.get_open_menu_at_depth(0) {
            if let Some(menu_item) = self.menu_items_with_submenus.get(open_index) {
                if !menu_item.subitems.is_empty() {
                    // Use reference instead of clone to preserve callbacks
                    let menu_text_size = self.menu_text_size;
                    let submenu_background_color = self.submenu_background_color;
                    let submenu_text_color = self.submenu_text_color;
                    let submenu_hover_color = self.submenu_hover_color;
                    let submenu_shortcut_color = self.submenu_shortcut_color;
                    let submenu_border_color = self.submenu_border_color;
                    let submenu_disabled_color = self.submenu_disabled_color;
                    let submenu_keyboard_selection_color = self.submenu_keyboard_selection_color;
                    let keyboard_navigation_active = self.keyboard_navigation_active;
                    // Get selection from recursive system
                    let current_selection =
                        self.render_state.get_selection_at_depth(0).unwrap_or(0);
                    let child_selection = self.render_state.get_selection_at_depth(1);
                    // Which item has the cascade open (for keyboard: show that cascade and highlight child_selection inside it)
                    let keyboard_open_child_index = (self.render_state.get_current_depth() >= 2)
                        .then(|| self.render_state.get_open_menu_at_depth(1))
                        .flatten();

                    // Calculate submenu position and hamburger_x
                    let (submenu_position, hamburger_x) = if self.submenu_from_hamburger {
                        // For hamburger menus, use the stored actual overlay position
                        // This ensures we use the same position as the real hamburger overlay
                        if let Some(overlay_x) = self.hamburger_overlay_x {
                            let submenu_pos = Pos2::new(overlay_x, title_bar_height());
                            (submenu_pos, Some(overlay_x))
                        } else {
                            // Fallback: calculate position if not stored (shouldn't happen in normal operation)
                            // This is the same logic as before but only as backup

                            // First, find which items are currently visible in main bar vs hamburger
                            let available_width = ctx.content_rect().width();
                            let control_buttons_width =
                                TitleBar::calculate_control_buttons_width(self);
                            let effective_available_width = available_width - control_buttons_width;

                            // Calculate which items fit in available space (same logic as render_menu_items)
                            let mut items_fitted = Vec::new();
                            let mut visible_width = 0.0;

                            // Create unified list using the stored chronological order
                            let mut unified_items = Vec::new();

                            for &(is_submenu, index) in &self.menu_order {
                                if is_submenu {
                                    if let Some(menu_item) =
                                        self.menu_items_with_submenus.get(index)
                                    {
                                        let label_width = ctx.fonts_mut(|f| {
                                            f.layout_no_wrap(
                                                menu_item.label.clone(),
                                                FontId::proportional(self.menu_text_size),
                                                self.menu_text_color,
                                            )
                                            .size()
                                            .x
                                        }) + 16.0;
                                        unified_items.push((index, label_width, true));
                                    }
                                } else {
                                    if let Some((label, _)) = self.menu_items.get(index) {
                                        let label_width = ctx.fonts_mut(|f| {
                                            f.layout_no_wrap(
                                                label.clone(),
                                                FontId::proportional(self.menu_text_size),
                                                self.menu_text_color,
                                            )
                                            .size()
                                            .x
                                        }) + 16.0;
                                        unified_items.push((index, label_width, false));
                                    }
                                }
                            }

                            // Determine if we need overflow indicator and reserve space for it
                            let needs_overflow =
                                unified_items.iter().map(|(_, w, _)| *w).sum::<f32>()
                                    > effective_available_width;
                            let overflow_width = if needs_overflow { 30.0 } else { 0.0 };
                            let effective_available_width =
                                effective_available_width - overflow_width;

                            // Calculate which items fit
                            let items_to_fit_count = unified_items.len();
                            for i in 0..items_to_fit_count {
                                let (_index, width, _is_submenu) = unified_items[i];
                                if visible_width + width <= effective_available_width {
                                    visible_width += width;
                                    items_fitted.push(i);
                                } else {
                                    break;
                                }
                            }

                            // Calculate hamburger overlay position - SAME LOGIC as render_overflow_menu_overlay
                            let hamburger_x = visible_width;
                            let padding = 8.0;

                            // Count items that will be shown in overlay
                            let mut overlay_items = Vec::new();
                            for (chronological_index, &(is_submenu, index)) in
                                self.menu_order.iter().enumerate()
                            {
                                if !items_fitted.contains(&chronological_index) {
                                    overlay_items.push((chronological_index, is_submenu, index));
                                }
                            }

                            // Calculate maximum width needed for overlay
                            let mut max_width: f32 = 120.0;
                            for &(_chronological_index, is_submenu, index) in &overlay_items {
                                if is_submenu {
                                    if let Some(menu_item) =
                                        self.menu_items_with_submenus.get(index)
                                    {
                                        let label_width = ctx.fonts_mut(|f| {
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
                                        let label_width = ctx.fonts_mut(|f| {
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

                            // IMPORTANT: Apply the SAME adjustment logic as render_overflow_menu_overlay
                            let content_rect = ctx.content_rect();
                            let adjusted_hamburger_x =
                                if hamburger_x + max_width > content_rect.max.x {
                                    content_rect.max.x - max_width // Same adjustment as line 627 in rendering.rs
                                } else {
                                    hamburger_x
                                };

                            // Position submenu using the ADJUSTED position
                            let item_y = title_bar_height();
                            let submenu_pos = Pos2::new(adjusted_hamburger_x, item_y);
                            (submenu_pos, Some(adjusted_hamburger_x))
                        }
                    } else {
                        // Normal positioning relative to parent menu item
                        // Calculate submenu position using stored menu positions
                        // Find the chronological position of this submenu in menu_order
                        let mut chronological_index = 0;
                        for &(is_submenu, index) in &self.menu_order {
                            if is_submenu && index == open_index {
                                break;
                            }
                            chronological_index += 1;
                        }

                        let submenu_x =
                            if let Some(menu_x) = self.menu_positions.get(chronological_index) {
                                *menu_x // Use actual stored position
                            } else {
                                // Fallback: center submenu if positions not available
                                100.0 // Reasonable default position
                            };
                        let submenu_pos = Pos2::new(submenu_x, title_bar_height()); // Below title bar
                        (submenu_pos, None)
                    };

                    // Get hamburger_x for child menus (only available in hamburger mode)
                    let hamburger_x_for_children = hamburger_x;

                    // Use a simple Vec instead of RefCell for render queue
                    let item_clicked = RefCell::new(false);
                    let was_leaf = RefCell::new(false);
                    let actual_submenu_rect = RefCell::new(Rect::ZERO);
                    let pending_leaf_action = RefCell::new(None::<(usize, Vec<usize>)>);
                    let mut menus_to_render = Vec::new();

                    // Create area at calculated position with HIGHER order than main overlay
                    let submenu_id = if self.submenu_from_hamburger {
                        format!("submenu_overlay_{}_hamburger", open_index)
                    } else {
                        format!("submenu_overlay_{}", open_index)
                    };
                    let open_cascade_sync_root = RefCell::new(None);
                    Area::new(Id::new(submenu_id))
                        .fixed_pos(submenu_position)
                        .order(Order::Foreground) // Higher than Background
                        .show(ctx, |ui| {
                            let (clicked, was_leaf_result, submenu_rect, pending, open_cascade) =
                                Self::render_overlay(
                                    ui,
                                    menu_item,
                                    submenu_position,
                                    menu_text_size,
                                    submenu_background_color,
                                    submenu_text_color,
                                    submenu_hover_color,
                                    submenu_shortcut_color,
                                    submenu_border_color,
                                    submenu_disabled_color,
                                    submenu_keyboard_selection_color,
                                    keyboard_navigation_active,
                                    Some(current_selection),
                                    child_selection,
                                    keyboard_open_child_index,
                                    open_index,
                                    self.submenu_from_hamburger,
                                    hamburger_x_for_children,
                                    &mut menus_to_render,
                                    open_index,
                                    vec![],
                                );

                            *item_clicked.borrow_mut() = clicked;
                            *was_leaf.borrow_mut() = was_leaf_result;
                            *actual_submenu_rect.borrow_mut() = submenu_rect;
                            if let Some(p) = pending {
                                *pending_leaf_action.borrow_mut() = Some(p);
                            }
                            *open_cascade_sync_root.borrow_mut() = open_cascade;
                        });
                    if let Some(idx) = open_cascade_sync_root.into_inner() {
                        // Close any already-open cascade so we don't stack overlays (e.g. click "New" after "Recent Files")
                        while self.render_state.get_current_depth() > 1 {
                            self.go_up_recursive();
                        }
                        self.go_deeper_recursive(idx);
                    }

                    // Render all cascade levels recursively: process menus_to_render in order (it grows as we render).
                    let mut i = 0;
                    while i < menus_to_render.len() {
                        let menu_info = menus_to_render[i].clone();
                        let depth = menu_info.path_prefix.len(); // 1 = first cascade, 2 = second, etc.
                        if keyboard_navigation_active
                            && self.render_state.get_open_menu_at_depth(depth)
                                != Some(menu_info.parent_index)
                        {
                            i += 1;
                            continue;
                        }
                        let child_submenu_id = format!(
                            "submenu_child_{}_{}",
                            open_index,
                            menu_info
                                .path_prefix
                                .iter()
                                .map(|n| n.to_string())
                                .collect::<Vec<_>>()
                                .join("_")
                        );

                        let selection_at_depth = self.render_state.get_selection_at_depth(depth);
                        let child_selection_for_overlay = selection_at_depth;
                        let keyboard_open_child_for_overlay =
                            (self.render_state.get_current_depth() > depth + 1)
                                .then(|| self.render_state.get_open_menu_at_depth(depth + 1))
                                .flatten();

                        let open_cascade_sync_child = RefCell::new(None);
                        Area::new(Id::new(child_submenu_id))
                            .fixed_pos(menu_info.position)
                            .order(Order::Foreground)
                            .show(ctx, |ui| {
                                let (child_clicked, child_was_leaf, _, pending, open_cascade) =
                                    Self::render_overlay(
                                        ui,
                                        &menu_info.menu_item,
                                        menu_info.position,
                                        menu_text_size,
                                        submenu_background_color,
                                        submenu_text_color,
                                        submenu_hover_color,
                                        submenu_shortcut_color,
                                        submenu_border_color,
                                        submenu_disabled_color,
                                        submenu_keyboard_selection_color,
                                        keyboard_navigation_active,
                                        child_selection_for_overlay,
                                        None,
                                        keyboard_open_child_for_overlay,
                                        menu_info.parent_index,
                                        menu_info.submenu_from_hamburger,
                                        hamburger_x_for_children,
                                        &mut menus_to_render,
                                        open_index,
                                        menu_info.path_prefix,
                                    );

                                if child_clicked && child_was_leaf {
                                    *item_clicked.borrow_mut() = true;
                                    *was_leaf.borrow_mut() = true;
                                }
                                if let Some(p) = pending {
                                    *pending_leaf_action.borrow_mut() = Some(p);
                                }
                                *open_cascade_sync_child.borrow_mut() = open_cascade;
                            });
                        if let Some(idx) = open_cascade_sync_child.into_inner() {
                            while self.render_state.get_current_depth() > depth + 1 {
                                self.go_up_recursive();
                            }
                            self.go_deeper_recursive(idx);
                        }
                        i += 1;
                    }

                    // Invoke deferred leaf callback after releasing menu_item borrow (mouse path: resolve original item so callbacks work in nested cascades)
                    if let Some(p) = pending_leaf_action.borrow_mut().take() {
                        self.pending_menu_leaf_action = Some(p);
                        self.invoke_pending_menu_leaf_action();
                    }

                    // Only close submenu if an item without children was clicked OR if clicking outside
                    let should_close = *item_clicked.borrow() && *was_leaf.borrow();
                    if should_close {
                        self.close_menu_from_depth(0);
                        self.submenu_just_opened_frame = false;

                        // Clear all menu states to reset cascading submenu states
                        if let Ok(mut state) = MENU_STATE.lock() {
                            state.clear();
                        }

                        // If submenu was opened from hamburger, also close hamburger
                        if self.submenu_from_hamburger {
                            self.hamburger_menu_open = false;
                        }
                    }

                    // Check for clicks outside the submenu area using input detection
                    // Skip this check for the first frame after opening from hamburger to avoid immediate closure
                    if ctx.input(|i| i.pointer.primary_clicked())
                        && !(self.submenu_just_opened_frame && self.submenu_from_hamburger)
                    {
                        let current_click_id = SUBMENU_CLICK_COUNTER.load(Ordering::Relaxed);
                        let click_pos = ctx.input(|i| i.pointer.interact_pos()).unwrap_or_default();

                        // Use the actual submenu rect from render_overlay
                        let submenu_rect = *actual_submenu_rect.borrow();

                        // Only close if this is a different click than the one that opened the submenu
                        if current_click_id > self.last_click_id {
                            // Close if click is outside submenu and not in menu bar
                            let menu_bar_rect = Rect::from_min_size(
                                Pos2::new(0.0, 0.0),
                                Vec2::new(ctx.content_rect().width(), title_bar_height()),
                            );

                            // For submenus opened from hamburger, also close if clicking anywhere outside submenu
                            // (including in the hamburger overlay area)
                            let should_close = if self.submenu_from_hamburger {
                                !submenu_rect.contains(click_pos)
                            } else {
                                !submenu_rect.contains(click_pos)
                                    && !menu_bar_rect.contains(click_pos)
                            };

                            if should_close {
                                // Close all menus but keep keyboard navigation active
                                self.reset_all_menu_states();
                                self.selected_menu_index = None;
                                self.overlay_selected_index = None;
                                self.dots_selected = false;

                                // Clear all menu states to reset cascading submenu states
                                if let Ok(mut state) = MENU_STATE.lock() {
                                    state.clear();
                                }

                                // If submenu was opened from hamburger, also close hamburger
                                if self.submenu_from_hamburger {
                                    self.hamburger_menu_open = false;
                                }
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
}
