use crate::menu::shortcuts::KeyboardShortcut;
use std::fmt::{Debug, Formatter, Result};

/// A single submenu item with customization options.
///
/// Represents an entry inside a dropdown menu, with optional keyboard shortcut,
/// enabled/disabled state, separator, callback, and nested children.
pub struct SubMenuItem {
    /// The visible label for this submenu item.
    pub label: String,
    /// Optional keyboard shortcut that triggers this item.
    pub shortcut: Option<KeyboardShortcut>,
    /// Whether the item can be interacted with.
    pub enabled: bool,
    /// If true, draws a separator line after this item.
    pub separator_after: bool,
    /// Optional callback executed when the item is activated.
    pub callback: Option<Box<dyn Fn() + Send + Sync>>,
    /// Optional nested submenu items.
    pub children: Vec<SubMenuItem>,
    /// Stable unique identifier for this menu item (prevents egui ID collisions)
    pub id: Option<String>,
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
            id: self.id.clone(),
        }
    }
}

impl SubMenuItem {
    /// Create a new submenu item with a text label.
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            shortcut: None,
            enabled: true,
            separator_after: false,
            callback: None,
            children: Vec::new(),
            id: None,
        }
    }

    /// Assign a keyboard shortcut to this item.
    pub fn with_shortcut(mut self, shortcut: KeyboardShortcut) -> Self {
        self.shortcut = Some(shortcut);
        self
    }

    /// Set the callback executed when this item is activated.
    pub fn with_callback(mut self, callback: Box<dyn Fn() + Send + Sync>) -> Self {
        self.callback = Some(callback);
        self
    }

    /// Disable this item (non-interactive, rendered as disabled).
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Disable this item conditionally (non-interactive when condition is true).
    pub fn disabled_if(mut self, condition: bool) -> Self {
        self.enabled = !condition;
        self
    }

    /// Draw a separator line after this item.
    pub fn with_separator(mut self) -> Self {
        self.separator_after = true;
        self
    }

    /// Append a child item to this submenu.
    pub fn add_child(mut self, child: SubMenuItem) -> Self {
        self.children.push(child);
        self
    }

    /// Set a stable unique ID for this menu item (prevents egui ID collisions)
    pub fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Replace the children of this submenu.
    pub fn with_children(mut self, children: Vec<SubMenuItem>) -> Self {
        self.children = children;
        self
    }
}

/// A menu item with submenu support.
#[derive(Debug, Clone)]
pub struct MenuItem {
    /// Top-level menu label.
    pub label: String,
    /// Submenu entries displayed when this menu is opened.
    pub subitems: Vec<SubMenuItem>,
    /// Whether the top-level menu is enabled.
    pub enabled: bool,
    /// Stable unique identifier for this menu (prevents egui ID collisions)
    pub id: Option<String>,
}

impl MenuItem {
    /// Create a new top-level menu.
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            subitems: Vec::new(),
            enabled: true,
            id: None,
        }
    }

    /// Append a submenu item to this menu.
    pub fn add_subitem(mut self, subitem: SubMenuItem) -> Self {
        self.subitems.push(subitem);
        self
    }

    /// Disable this top-level menu.
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Set a stable unique ID for this menu (prevents egui ID collisions)
    pub fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Get a stable ID for egui widgets (fallback to label if no ID set)
    pub fn get_stable_id(&self) -> String {
        self.id.as_ref().unwrap_or(&self.label).clone()
    }
}
