use crate::TitleBar;

impl TitleBar {
    /// Calculate the width of a menu item based on its type and index
    pub fn calculate_item_width(&self, is_submenu: bool, index: usize) -> f32 {
        if is_submenu {
            if let Some(menu_item) = self.menu_items_with_submenus.get(index) {
                menu_item.label.len() as f32 * 8.0 + 16.0
            } else {
                0.0
            }
        } else {
            if let Some((label, _)) = self.menu_items.get(index) {
                label.len() as f32 * 8.0 + 16.0
            } else {
                0.0
            }
        }
    }
}
