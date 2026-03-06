use crate::TitleBar;

impl TitleBar {
    /// Invoke deferred menu leaf action (run after releasing menu borrow).
    /// Used by both keyboard and mouse paths so the real callback from menu_items_with_submenus is called.
    pub fn invoke_pending_menu_leaf_action(&mut self) {
        if let Some((menu_index, path)) = self.pending_menu_leaf_action.take() {
            let mut current = match self.menu_items_with_submenus.get(menu_index) {
                Some(m) => &m.subitems,
                None => return,
            };
            for (depth, &idx) in path.iter().enumerate() {
                if let Some(item) = current.get(idx) {
                    if depth == path.len() - 1 {
                        if item.enabled {
                            if let Some(callback) = &item.callback {
                                callback();
                            }
                        }
                        break;
                    }
                    current = &item.children;
                } else {
                    break;
                }
            }
        }
    }
}
