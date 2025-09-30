use crate::menu::shortcuts::KeyboardShortcut;
use std::fmt::{Debug, Formatter, Result};

/// A single submenu item with customization options
pub struct SubMenuItem {
    pub label: String,
    pub shortcut: Option<KeyboardShortcut>,
    pub enabled: bool,
    pub separator_after: bool,
    pub callback: Option<Box<dyn Fn() + Send + Sync>>,
    pub children: Vec<SubMenuItem>,
}

impl Debug for SubMenuItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("SubMenuItem")
            .field("label", &self.label)
            .field("shortcut", &self.shortcut)
            .field("enabled", &self.enabled)
            .field("separator_after", &self.separator_after)
            .field("callback", &"<function>")
            .finish()
    }
}

impl Clone for SubMenuItem {
    fn clone(&self) -> Self {
        Self {
            label: self.label.clone(),
            shortcut: self.shortcut.clone(),
            enabled: self.enabled,
            separator_after: self.separator_after,
            callback: None, // Can't clone callbacks, set to None
            children: self.children.clone(),
        }
    }
}

impl SubMenuItem {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            shortcut: None,
            enabled: true,
            separator_after: false,
            callback: None,
            children: Vec::new(),
        }
    }

    pub fn with_shortcut(mut self, shortcut: KeyboardShortcut) -> Self {
        self.shortcut = Some(shortcut);
        self
    }

    pub fn with_callback(mut self, callback: Box<dyn Fn() + Send + Sync>) -> Self {
        self.callback = Some(callback);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    pub fn with_separator(mut self) -> Self {
        self.separator_after = true;
        self
    }

    pub fn add_child(mut self, child: SubMenuItem) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_children(mut self, children: Vec<SubMenuItem>) -> Self {
        self.children = children;
        self
    }
}

/// A menu item with submenu support
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub subitems: Vec<SubMenuItem>,
    pub enabled: bool,
}

impl MenuItem {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            subitems: Vec::new(),
            enabled: true,
        }
    }

    pub fn add_subitem(mut self, subitem: SubMenuItem) -> Self {
        self.subitems.push(subitem);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}
