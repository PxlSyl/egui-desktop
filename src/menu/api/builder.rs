use crate::{TitleBar, menu::items::MenuItem};

impl TitleBar {
    /// Add a menu item to the title bar
    ///
    /// Menu items are displayed in the title bar and can have optional callbacks.
    ///
    /// # Arguments
    /// * `label` - The text label for the menu item
    /// * `callback` - Optional callback function to execute when clicked
    ///
    /// # Examples
    ///
    /// ```rust
    /// title_bar.add_menu_item("File", None)
    ///     .add_menu_item("Save", Some(Box::new(|| println!("Save clicked!"))))
    /// ```
    pub fn add_menu_item(
        mut self,
        label: &str,
        callback: Option<Box<dyn Fn() + Send + Sync>>,
    ) -> Self {
        self.menu_items.push((label.to_string(), callback));
        self
    }
    /// Add a menu item with submenu support to the title bar
    ///
    /// This method allows you to create dropdown menus with subitems that support
    /// keyboard shortcuts, separators, and individual callbacks.
    ///
    /// # Arguments
    /// * `menu_item` - A MenuItem struct containing the menu label and subitems
    ///
    /// # Examples
    ///
    /// ```rust
    /// let file_menu = MenuItem::new("File")
    ///     .add_subitem(SubMenuItem::new("New")
    ///         .with_shortcut(KeyboardShortcut::new("N").with_ctrl())
    ///         .with_callback(Box::new(|| println!("New file!"))))
    ///     .add_subitem(SubMenuItem::new("Open")
    ///         .with_shortcut(KeyboardShortcut::new("O").with_ctrl())
    ///         .with_callback(Box::new(|| println!("Open file!"))))
    ///     .add_subitem(SubMenuItem::new("Save")
    ///         .with_shortcut(KeyboardShortcut::new("S").with_ctrl())
    ///         .with_callback(Box::new(|| println!("Save file!")))
    ///         .with_separator())
    ///     .add_subitem(SubMenuItem::new("Exit").disabled());
    ///
    /// title_bar.add_menu_with_submenu(file_menu);
    /// ```
    pub fn add_menu_with_submenu(mut self, menu_item: MenuItem) -> Self {
        self.menu_items_with_submenus.push(menu_item);
        self
    }
}
