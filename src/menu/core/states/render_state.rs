/// Recursive menu state for rendering - can handle N levels of nesting
#[derive(Debug, Clone)]
pub struct RenderState {
    /// open_menus[i] = Some(index) if menu at depth i is open with item at index selected
    /// open_menus[0] = Some(0) means main menu item 0 is open (File menu)
    /// open_menus[1] = Some(2) means submenu item 2 is open (Recent Files)
    pub open_menus: Vec<Option<usize>>,

    /// selections[i] = selected item index at depth i
    /// selections[0] = 0 means File menu is selected
    /// selections[1] = 2 means Recent Files is selected
    pub selections: Vec<usize>,
}

impl RenderState {
    pub fn new() -> Self {
        Self {
            open_menus: Vec::new(),
            selections: Vec::new(),
        }
    }

    /// Reset all state
    pub fn reset(&mut self) {
        self.open_menus.clear();
        self.selections.clear();
    }

    /// Get current depth (how many menus are open)
    pub fn get_current_depth(&self) -> usize {
        self.open_menus.len()
    }

    /// Check if any menu is open
    pub fn is_any_menu_open(&self) -> bool {
        !self.open_menus.is_empty()
    }

    /// Get the index of the open menu at specific depth
    pub fn get_open_menu_at_depth(&self, depth: usize) -> Option<usize> {
        self.open_menus.get(depth).copied().flatten()
    }

    /// Get the selection at specific depth
    pub fn get_selection_at_depth(&self, depth: usize) -> Option<usize> {
        self.selections.get(depth).copied()
    }

    /// Open a menu at specific depth with specific selection
    pub fn open_menu_at_depth(&mut self, depth: usize, menu_index: usize, selection: usize) {
        // Ensure vectors are long enough
        while self.open_menus.len() <= depth {
            self.open_menus.push(None);
            self.selections.push(0);
        }

        self.open_menus[depth] = Some(menu_index);
        self.selections[depth] = selection;

        // Close all deeper menus
        self.open_menus.truncate(depth + 1);
        self.selections.truncate(depth + 1);
    }

    /// Close menu at specific depth (and all deeper menus)
    pub fn close_menus_from_depth(&mut self, depth: usize) {
        self.open_menus.truncate(depth);
        self.selections.truncate(depth);
    }

    /// Update selection at specific depth
    pub fn update_selection_at_depth(&mut self, depth: usize, selection: usize) {
        if depth < self.selections.len() {
            self.selections[depth] = selection;
        }
    }

    /// Go deeper (open submenu of current item)
    /// Updates the current level's selection to the clicked item so the parent menu highlights the correct row (e.g. "Recent Files" not "New").
    pub fn go_deeper(&mut self, item_index: usize) {
        if !self.selections.is_empty() {
            let last = self.selections.len() - 1;
            self.selections[last] = item_index;
        }
        self.open_menus.push(Some(item_index));
        self.selections.push(0);
    }

    /// Go up (close current level)
    pub fn go_up(&mut self) -> bool {
        if self.open_menus.is_empty() {
            return false;
        }

        self.open_menus.pop();
        self.selections.pop();
        true
    }
}

impl Default for RenderState {
    fn default() -> Self {
        Self::new()
    }
}
