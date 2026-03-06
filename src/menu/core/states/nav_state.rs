/// Navigation state: selection index at each menu depth (synced from RenderState).
#[derive(Debug, Clone)]
pub struct KeyboardState {
    /// Selection at each depth
    selections: Vec<usize>,
}

impl KeyboardState {
    pub fn new() -> Self {
        Self {
            selections: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.selections.clear();
    }

    pub fn get_selection_at_depth(&self, depth: usize) -> Option<usize> {
        self.selections.get(depth).copied()
    }

    pub fn set_selection_at_depth(&mut self, depth: usize, selection: usize) {
        while self.selections.len() <= depth {
            self.selections.push(0);
        }
        self.selections[depth] = selection;
    }
}
