//! Layout and measurement helpers for the responsive menu bar.

use egui::{Color32, FontId, Ui};

use crate::TitleBar;

impl TitleBar {
    /// Build overlay items list based on current mode (matches navigation logic)
    pub fn build_overlay_items(&self) -> Vec<(usize, bool, usize)> {
        let mut overlay_items = Vec::new();
        for (chronological_index, &(is_submenu, index)) in self.menu_order.iter().enumerate() {
            // Logic must match navigation logic
            let should_include = if self.items_fitted.is_empty() {
                // Mode minimal: tous les items vont dans les dots
                true
            } else {
                // Mode normal: seulement les items après le dernier item fitted
                if let Some(&last_fitted_index) = self.items_fitted.last() {
                    chronological_index > last_fitted_index
                } else {
                    false
                }
            };

            if should_include {
                overlay_items.push((chronological_index, is_submenu, index));
            }
        }
        overlay_items
    }

    /// Get a contrasting text color for keyboard selection
    pub(super) fn get_contrasting_text_color(&self) -> Color32 {
        // For now, use white text which works well with blue selection backgrounds
        // This could be made more sophisticated in the future to adapt to the exact background color
        Color32::WHITE
    }

    /// Calculate the width of a menu item label
    pub(super) fn calculate_menu_item_width(&self, ui: &mut Ui, label: &str) -> f32 {
        ui.fonts_mut(|f| {
            f.layout_no_wrap(
                label.to_string(),
                FontId::proportional(self.menu_text_size),
                self.menu_text_color,
            )
            .size()
            .x
        }) + 16.0
    }

    /// Create unified list of menu items with their widths
    pub(super) fn create_unified_items(&self, ui: &mut Ui) -> Vec<(usize, f32, bool)> {
        let mut unified_items = Vec::new();

        // Use menu_order to preserve the exact order in which items were added
        for &(is_submenu, index) in &self.menu_order {
            if is_submenu {
                // Submenu item
                if let Some(menu_item) = self.menu_items_with_submenus.get(index) {
                    let label_width = self.calculate_menu_item_width(ui, &menu_item.label);
                    unified_items.push((index, label_width, true)); // true = submenu
                }
            } else {
                // Simple menu item
                if let Some((label, _)) = self.menu_items.get(index) {
                    let label_width = self.calculate_menu_item_width(ui, label);
                    unified_items.push((index, label_width, false)); // false = simple menu
                }
            }
        }

        unified_items
    }

    /// Calculate which items fit and overflow details
    pub(super) fn calculate_overflow_details(
        &self,
        unified_items: &[(usize, f32, bool)],
        effective_available_width: f32,
    ) -> (Vec<usize>, bool, f32, bool) {
        let mut items_fitted = Vec::new();
        let mut visible_width = 0.0;

        // Determine if we need overflow indicator and reserve space for it
        let needs_overflow =
            unified_items.iter().map(|(_, w, _)| *w).sum::<f32>() > effective_available_width;
        let overflow_width = if needs_overflow {
            30.0 // "..." width for compact mode
        } else {
            0.0
        };

        // Now calculate which items fit with overflow space reserved
        let effective_available_width = effective_available_width - overflow_width;
        let items_to_fit_count = unified_items.len();

        for i in 0..items_to_fit_count {
            let (_index, width, _is_submenu) = unified_items[i];
            if visible_width + width <= effective_available_width {
                visible_width += width;
                items_fitted.push(i);
            } else {
                break; // Stop when items no longer fit
            }
        }

        // Check if we're in minimal mode (very small window - no items fit)
        let is_minimal_mode = items_fitted.is_empty();

        // Adjust overflow width for minimal mode (hamburger instead of "...")
        let final_overflow_width = if needs_overflow {
            if is_minimal_mode {
                28.0 // Hamburger width for minimal mode (same as icon height)
            } else {
                30.0 // "..." width for compact mode
            }
        } else {
            0.0
        };

        (
            items_fitted,
            needs_overflow,
            final_overflow_width,
            is_minimal_mode,
        )
    }
}
