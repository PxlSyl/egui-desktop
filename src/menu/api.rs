use egui::{
    Align2, Area, Color32, Context, CornerRadius, CursorIcon, FontId, Id, Order, Pos2, Rect, Sense,
    Stroke, StrokeKind, Ui, Vec2,
};
use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::menu::items::MenuItem;
use crate::TitleBar;

// Global state for submenu management
static SUBMENU_CLICK_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl TitleBar {
    /// Set the color of menu item text
    ///
    /// # Arguments
    /// * `color` - The menu text color as a Color32
    ///
    /// # Examples
    ///
    /// ```rust
    /// title_bar.with_menu_text_color(Color32::from_rgb(50, 50, 50))
    /// ```
    pub fn with_menu_text_color(mut self, color: Color32) -> Self {
        self.menu_text_color = color;
        self
    }

    /// Set the hover color for menu items
    ///
    /// This color is used when hovering over menu items in the title bar.
    ///
    /// # Arguments
    /// * `color` - The menu hover color as a Color32
    ///
    /// # Examples
    ///
    /// ```rust
    /// title_bar.with_menu_hover_color(Color32::from_rgb(220, 220, 220))
    /// ```
    pub fn with_menu_hover_color(mut self, color: Color32) -> Self {
        self.menu_hover_color = color;
        self
    }

    /// Set the font size of menu item text
    ///
    /// # Arguments
    /// * `size` - The font size in points
    ///
    /// # Examples
    ///
    /// ```rust
    /// title_bar.with_menu_text_size(14.0)
    /// ```
    pub fn with_menu_text_size(mut self, size: f32) -> Self {
        self.menu_text_size = size;
        self
    }

    /// Check for keyboard shortcuts and trigger callbacks
    ///
    /// This method should be called before rendering menus to handle keyboard shortcuts.
    ///
    /// # Arguments
    /// * `ctx` - The egui context
    pub fn check_keyboard_shortcuts(&mut self, ctx: &Context) {
        // Check menu items with submenus
        for menu_item in &self.menu_items_with_submenus {
            for subitem in &menu_item.subitems {
                if let Some(ref shortcut) = subitem.shortcut {
                    if shortcut.just_pressed(ctx) && subitem.enabled {
                        if let Some(ref callback) = subitem.callback {
                            callback();
                        }
                    }
                }
            }
        }
    }

    /// Handle keyboard navigation for menus
    ///
    /// This method handles arrow keys, Enter, and Escape for menu navigation.
    ///
    /// # Arguments
    /// * `ctx` - The egui context
    pub fn handle_keyboard_navigation(&mut self, ctx: &Context) {
        let current_time = ctx.input(|i| i.time);

        // Check if Alt key or Ctrl+F2 is pressed to activate menu navigation
        let should_activate = ctx.input(|i| i.modifiers.alt)
            || (ctx.input(|i| i.modifiers.ctrl) && ctx.input(|i| i.key_pressed(egui::Key::F2)));

        if should_activate {
            if !self.keyboard_navigation_active {
                self.keyboard_navigation_active = true;
                self.selected_menu_index = Some(0);
                self.selected_submenu_index = None;
                self.last_keyboard_nav_time = current_time;
            }
        }

        // Handle navigation when active
        if self.keyboard_navigation_active {
            // Handle Escape to close menus
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.keyboard_navigation_active = false;
                self.selected_menu_index = None;
                self.selected_submenu_index = None;
                self.open_submenu = None;
                return;
            }

            // Handle clicks outside menu areas to close menus (but keep keyboard nav active)
            if ctx.input(|i| i.pointer.primary_clicked()) {
                let click_pos = ctx.input(|i| i.pointer.interact_pos()).unwrap_or_default();
                let menu_bar_rect = Rect::from_min_size(
                    Pos2::new(0.0, 0.0),
                    Vec2::new(ctx.screen_rect().width(), 32.0),
                );

                // If click is outside menu bar and any submenu is open, close all menus
                if !menu_bar_rect.contains(click_pos) && self.open_submenu.is_some() {
                    self.open_submenu = None;
                    self.force_open_child_subitem = None;
                    self.child_submenu_selections.clear();
                    // Keep keyboard_navigation_active = true (don't disable it)
                }
            }

            // Handle left/right arrow keys for top-level menu navigation
            // Disable only when we're on a highlighted submenu item that has a sidemenu
            let current_highlighted_has_sidemenu = if let Some(open_submenu_index) =
                self.open_submenu
            {
                if let Some(menu_item) = self.menu_items_with_submenus.get(open_submenu_index) {
                    if let Some(selected_index) = self.submenu_selections.get(&open_submenu_index) {
                        if let Some(selected_item) = menu_item.subitems.get(*selected_index) {
                            !selected_item.children.is_empty()
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };

            if !current_highlighted_has_sidemenu {
                if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
                    if let Some(current_index) = self.selected_menu_index {
                        if current_index > 0 {
                            self.selected_menu_index = Some(current_index - 1);
                            self.open_submenu = None;
                            self.selected_submenu_index = None;
                        }
                    }
                }

                if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
                    let total_menus = self.menu_items.len() + self.menu_items_with_submenus.len();
                    if let Some(current_index) = self.selected_menu_index {
                        if current_index < total_menus - 1 {
                            self.selected_menu_index = Some(current_index + 1);
                            self.open_submenu = None;
                            self.selected_submenu_index = None;
                        }
                    }
                }
            }

            // Handle Enter and Space keys - unified logic for all contexts
            if ctx.input(|i| i.key_pressed(egui::Key::Enter))
                || ctx.input(|i| i.key_pressed(egui::Key::Space))
            {
                // Priority 1: If we're in a child sidemenu, handle that first
                if let Some(open_submenu_index) = self.open_submenu {
                    if let Some(child_submenu_index) =
                        self.child_submenu_selections.get(&open_submenu_index)
                    {
                        if let Some(menu_item) =
                            self.menu_items_with_submenus.get(open_submenu_index)
                        {
                            if let Some(child_index) = self.force_open_child_subitem {
                                if let Some(child_item) = menu_item.subitems.get(child_index) {
                                    if let Some(child_subitem) =
                                        child_item.children.get(*child_submenu_index)
                                    {
                                        if child_subitem.enabled {
                                            if let Some(ref callback) = child_subitem.callback {
                                                callback();
                                            }
                                            // Close all submenus after action
                                            self.open_submenu = None;
                                            self.selected_submenu_index = None;
                                            self.force_open_child_subitem = None;
                                            self.child_submenu_selections.clear();
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Priority 2: If we're in a submenu, handle submenu item
                if let Some(open_submenu_index) = self.open_submenu {
                    if self.submenu_just_opened_frame {
                        // Skip if submenu was just opened this frame
                        self.submenu_just_opened_frame = false;
                        return;
                    }
                    if let Some(submenu_index) = self.submenu_selections.get(&open_submenu_index) {
                        if let Some(menu_item) =
                            self.menu_items_with_submenus.get(open_submenu_index)
                        {
                            if let Some(subitem) = menu_item.subitems.get(*submenu_index) {
                                if subitem.enabled && subitem.children.is_empty() {
                                    // Only trigger if it has no children (no sidemenu)
                                    if let Some(ref callback) = subitem.callback {
                                        callback();
                                    }
                                    // Close submenu after action
                                    self.open_submenu = None;
                                    self.submenu_selections.remove(&open_submenu_index);
                                    return;
                                }
                            }
                        }
                    }
                }

                // Priority 3: Handle main menu items
                if let Some(menu_index) = self.selected_menu_index {
                    let total_simple_menus = self.menu_items.len();

                    if menu_index < total_simple_menus {
                        // Simple menu item - trigger callback
                        if let Some((_, callback)) = self.menu_items.get(menu_index) {
                            if let Some(ref callback) = callback {
                                callback();
                            }
                        }
                    } else {
                        // Menu with submenu
                        let submenu_index = menu_index - total_simple_menus;
                        if let Some(menu_item) = self.menu_items_with_submenus.get(submenu_index) {
                            if !menu_item.subitems.is_empty() {
                                self.open_submenu = Some(submenu_index);
                                self.submenu_selections.insert(submenu_index, 0);
                                // Mark as just opened to avoid immediately activating first item on Enter this frame
                                self.submenu_just_opened_frame = true;
                                // Only auto-open child submenu on keyboard navigation activation
                                if self.keyboard_navigation_active {
                                    if let Some(first_item) = menu_item.subitems.get(0) {
                                        if !first_item.children.is_empty() {
                                            self.force_open_child_subitem = Some(0);
                                        } else {
                                            self.force_open_child_subitem = None;
                                        }
                                    } else {
                                        self.force_open_child_subitem = None;
                                    }
                                } else {
                                    // Mouse-driven open: do not auto-open a child side menu
                                    self.force_open_child_subitem = None;
                                }
                            }
                        }
                    }
                }
            }

            // Handle up/down/left/right keys for submenu navigation and side menus
            if let Some(open_submenu_index) = self.open_submenu {
                if let Some(menu_item) = self.menu_items_with_submenus.get(open_submenu_index) {
                    // Handle up/down navigation in main submenu ONLY if no child sidemenu is open
                    // Child submenu navigation is handled separately below
                    if self.force_open_child_subitem.is_none()
                        && !self
                            .child_submenu_selections
                            .contains_key(&open_submenu_index)
                    {
                        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                            if let Some(current_submenu_index) =
                                self.submenu_selections.get(&open_submenu_index).copied()
                            {
                                if current_submenu_index > 0 {
                                    self.submenu_selections
                                        .insert(open_submenu_index, current_submenu_index - 1);
                                }
                            }
                        }

                        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                            if let Some(current_submenu_index) =
                                self.submenu_selections.get(&open_submenu_index).copied()
                            {
                                if current_submenu_index < menu_item.subitems.len() - 1 {
                                    self.submenu_selections
                                        .insert(open_submenu_index, current_submenu_index + 1);
                                }
                            }
                        }
                    }

                    // Right arrow on a submenu item that has children -> force-open child sidemenu
                    if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
                        if let Some(current_submenu_index) =
                            self.submenu_selections.get(&open_submenu_index)
                        {
                            if let Some(current_item) =
                                menu_item.subitems.get(*current_submenu_index)
                            {
                                if !current_item.children.is_empty() {
                                    // Flag to force-open the child submenu in the renderer
                                    self.force_open_child_subitem = Some(*current_submenu_index);
                                    // Prevent immediate activation this frame
                                    self.submenu_just_opened_frame = true;
                                }
                            }
                        }
                    }

                    // Left arrow: back out of child sidemenu first; if none, close this submenu
                    if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
                        if self.force_open_child_subitem.is_some() {
                            self.force_open_child_subitem = None;
                            self.child_submenu_selections.remove(&open_submenu_index);
                        } else if self.keyboard_navigation_active
                            && self
                                .child_submenu_selections
                                .contains_key(&open_submenu_index)
                        {
                            // Close child submenu navigation but keep parent submenu open
                            self.child_submenu_selections.remove(&open_submenu_index);
                        } else {
                            // Close current submenu, keep focus on the parent menu
                            self.open_submenu = None;
                            self.submenu_selections.remove(&open_submenu_index);
                        }
                    }

                    // Handle navigation within child submenu when it's open
                    // Check if any child submenu is currently open (either forced or hovered)
                    let mut active_child_index = self.force_open_child_subitem;

                    // If no forced child, check if we should enable child navigation
                    // This works for mouse hover, but NOT for keyboard navigation (keyboard needs explicit right arrow)
                    if active_child_index.is_none() && !self.keyboard_navigation_active {
                        // Find the first submenu item that has children and is currently selected
                        if let Some(selected_index) =
                            self.submenu_selections.get(&open_submenu_index)
                        {
                            if let Some(selected_item) = menu_item.subitems.get(*selected_index) {
                                if !selected_item.children.is_empty() {
                                    active_child_index = Some(*selected_index);
                                }
                            }
                        }
                    }

                    if let Some(child_index) = active_child_index {
                        if let Some(child_item) = menu_item.subitems.get(child_index) {
                            if !child_item.children.is_empty() {
                                // Initialize child submenu selection if not set
                                if !self
                                    .child_submenu_selections
                                    .contains_key(&open_submenu_index)
                                {
                                    self.child_submenu_selections.insert(open_submenu_index, 0);
                                }

                                // Handle up/down navigation in child submenu
                                if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                                    if let Some(current_child_index) =
                                        self.child_submenu_selections.get(&open_submenu_index)
                                    {
                                        if *current_child_index > 0 {
                                            self.child_submenu_selections.insert(
                                                open_submenu_index,
                                                current_child_index - 1,
                                            );
                                        }
                                    }
                                }

                                if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                                    if let Some(current_child_index) =
                                        self.child_submenu_selections.get(&open_submenu_index)
                                    {
                                        if *current_child_index < child_item.children.len() - 1 {
                                            self.child_submenu_selections.insert(
                                                open_submenu_index,
                                                current_child_index + 1,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Reset one-frame guards at the end of the keyboard nav cycle
            if self.submenu_just_opened_frame {
                self.submenu_just_opened_frame = false;
            }
            // force_open_child_subitem persists until user navigates away or presses Left; do not reset here
        }
    }

    /// Render menu items using native-style rendering (similar to Glitchine)
    ///
    /// This method renders menu items as clickable text areas with native-style behavior,
    /// similar to how native applications handle menu bars. Supports both simple menu items
    /// and menu items with submenus.
    ///
    /// # Arguments
    /// * `ui` - The egui UI context
    pub fn render_menu_items(&mut self, ui: &mut Ui, ctx: &Context) {
        // Check for keyboard shortcuts and navigation first
        self.check_keyboard_shortcuts(ctx);
        self.handle_keyboard_navigation(ctx);

        if self.menu_items.is_empty() && self.menu_items_with_submenus.is_empty() {
            return;
        }

        let menu_height = 28.0; // Standard menu height

        // Calculate total width needed for all menus
        let mut total_width = 0.0;
        for (label, _) in &self.menu_items {
            let label_width = ui.fonts(|f| {
                f.layout_no_wrap(
                    label.clone(),
                    FontId::proportional(self.menu_text_size),
                    self.menu_text_color,
                )
                .size()
                .x
            }) + 16.0;
            total_width += label_width;
        }
        for menu_item in &self.menu_items_with_submenus {
            let label_width = ui.fonts(|f| {
                f.layout_no_wrap(
                    menu_item.label.clone(),
                    FontId::proportional(self.menu_text_size),
                    self.menu_text_color,
                )
                .size()
                .x
            }) + 16.0;
            total_width += label_width;
        }

        // Allocate space for the entire menu bar
        let (menu_bar_rect, _) =
            ui.allocate_exact_size(egui::Vec2::new(total_width, menu_height), Sense::click());

        let mut current_x = menu_bar_rect.min.x;

        // Clear and rebuild menu positions
        self.menu_positions.clear();

        // Render simple menu items
        for (index, (label, callback)) in self.menu_items.iter().enumerate() {
            let label_width = ui.fonts(|f| {
                f.layout_no_wrap(
                    label.clone(),
                    FontId::proportional(self.menu_text_size),
                    self.menu_text_color,
                )
                .size()
                .x
            }) + 16.0;

            // Store the position of this menu item
            self.menu_positions.push(current_x);

            // Create individual menu rect
            let menu_rect = Rect::from_min_size(
                Pos2::new(current_x, menu_bar_rect.min.y),
                Vec2::new(label_width, menu_height),
            );

            // Interact with the menu area
            let response = ui.interact(
                menu_rect,
                Id::new(format!("menu_{}", label)),
                Sense::click(),
            );

            // Check if this menu item is selected by keyboard navigation
            let is_keyboard_selected =
                self.keyboard_navigation_active && self.selected_menu_index == Some(index);

            // Handle hover effect or keyboard selection (render background first)
            if response.hovered() || is_keyboard_selected {
                let highlight_color = if is_keyboard_selected {
                    // Use configurable keyboard selection color
                    self.keyboard_selection_color
                } else {
                    self.menu_hover_color
                };
                ui.painter()
                    .rect_filled(menu_rect, CornerRadius::same(2), highlight_color);
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }

            // Render menu text centered (always rendered on top)
            let text_color = if is_keyboard_selected {
                Color32::WHITE // White text on keyboard selection background
            } else {
                self.menu_text_color
            };

            ui.painter().text(
                menu_rect.center(),
                Align2::CENTER_CENTER,
                label,
                FontId::proportional(self.menu_text_size),
                text_color,
            );

            // Handle click
            if response.clicked() {
                if let Some(ref callback) = callback {
                    callback();
                }
            }

            // Move to next menu position
            current_x += label_width;
        }

        // Render menu items with submenus
        for (index, menu_item) in self.menu_items_with_submenus.iter().enumerate() {
            let label_width = ui.fonts(|f| {
                f.layout_no_wrap(
                    menu_item.label.clone(),
                    FontId::proportional(self.menu_text_size),
                    self.menu_text_color,
                )
                .size()
                .x
            }) + 16.0;

            // Store the position of this menu item (offset by simple menu count)
            self.menu_positions.push(current_x);

            // Create individual menu rect
            let menu_rect = Rect::from_min_size(
                Pos2::new(current_x, menu_bar_rect.min.y),
                Vec2::new(label_width, menu_height),
            );

            // Interact with the menu area
            let response = ui.interact(
                menu_rect,
                Id::new(format!("submenu_{}", menu_item.label)),
                Sense::click(),
            );

            // Check if this menu item is selected by keyboard navigation
            let menu_index = self.menu_items.len() + index;
            let is_keyboard_selected =
                self.keyboard_navigation_active && self.selected_menu_index == Some(menu_index);

            // Handle hover effect or keyboard selection
            if response.hovered() || is_keyboard_selected {
                let highlight_color = if is_keyboard_selected {
                    // Use configurable keyboard selection color
                    self.keyboard_selection_color
                } else {
                    self.menu_hover_color
                };
                ui.painter()
                    .rect_filled(menu_rect, CornerRadius::same(2), highlight_color);
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }

            // Handle click to toggle submenu
            if response.clicked() {
                // Toggle submenu: close if same, open if different
                if self.open_submenu == Some(index) {
                    self.open_submenu = None;
                    self.submenu_just_opened_frame = false;
                } else {
                    self.open_submenu = Some(index);
                    self.submenu_just_opened_frame = true;
                    // Generate unique click ID
                    self.last_click_id = SUBMENU_CLICK_COUNTER.fetch_add(1, Ordering::Relaxed);
                }
            }

            // Render menu text centered (always rendered on top)
            let text_color = if is_keyboard_selected {
                Color32::WHITE // White text on keyboard selection background
            } else if menu_item.enabled {
                self.menu_text_color
            } else {
                Color32::from_rgb(150, 150, 150) // Disabled color
            };

            ui.painter().text(
                menu_rect.center(),
                Align2::CENTER_CENTER,
                &menu_item.label,
                FontId::proportional(self.menu_text_size),
                text_color,
            );

            // Move to next menu position
            current_x += label_width;
        }
    }
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

    /// Render the currently open submenu as an overlay
    pub fn render_open_submenu(&mut self, ctx: &Context) {
        if let Some(open_index) = self.open_submenu {
            if let Some(menu_item) = self.menu_items_with_submenus.get(open_index) {
                if !menu_item.subitems.is_empty() {
                    // Use reference instead of clone to preserve callbacks
                    let menu_text_size = self.menu_text_size;
                    let submenu_background_color = self.submenu_background_color;
                    let submenu_text_color = self.submenu_text_color;
                    let submenu_hover_color = self.submenu_hover_color;
                    let submenu_shortcut_color = self.submenu_shortcut_color;
                    let submenu_border_color = self.submenu_border_color;
                    let submenu_keyboard_selection_color = self.submenu_keyboard_selection_color;
                    let keyboard_navigation_active = self.keyboard_navigation_active;
                    let submenu_selections = self.submenu_selections.clone();
                    let force_open_child_subitem = self.force_open_child_subitem;
                    let child_submenu_selections = self.child_submenu_selections.clone();

                    // Calculate submenu position using stored menu positions
                    let submenu_x = if let Some(menu_x) =
                        self.menu_positions.get(self.menu_items.len() + open_index)
                    {
                        *menu_x
                    } else {
                        // Fallback to old calculation if positions not available
                        let mut menu_x = 16.0 + 20.0 + 8.0; // icon + title space + padding
                        for i in 0..open_index {
                            if let Some(item) = self.menu_items_with_submenus.get(i) {
                                // Fallback: use character count approximation for positioning
                                menu_x += item.label.len() as f32 * (menu_text_size * 0.6) + 16.0;
                            }
                        }
                        menu_x
                    };
                    let submenu_position = Pos2::new(submenu_x, 32.0); // Below title bar

                    // Use a RefCell to allow modification from within the closure
                    let item_clicked = RefCell::new(false);

                    // Create a full-screen area to capture clicks outside
                    Area::new(egui::Id::new(format!("submenu_overlay_{}", open_index)))
                        .fixed_pos(Pos2::ZERO)
                        .order(Order::Foreground)
                        .show(ctx, |ui| {
                            // Render the submenu at the calculated position
                            let clicked = Self::render_submenu_overlay_static(
                                ui,
                                menu_item, // Pass reference instead of clone
                                submenu_position,
                                menu_text_size,
                                submenu_background_color,
                                submenu_text_color,
                                submenu_hover_color,
                                submenu_shortcut_color,
                                submenu_border_color,
                                submenu_keyboard_selection_color,
                                keyboard_navigation_active,
                                submenu_selections.get(&open_index).copied(),
                                force_open_child_subitem,
                                child_submenu_selections.get(&open_index).copied(),
                                open_index, // Pass parent submenu index
                            );

                            // Store the click result
                            *item_clicked.borrow_mut() = clicked;
                        });

                    // Close submenu if an item was clicked
                    if *item_clicked.borrow() {
                        self.open_submenu = None;
                        self.submenu_just_opened_frame = false;
                    }

                    // Check for clicks outside the submenu area using input detection
                    if ctx.input(|i| i.pointer.primary_clicked()) {
                        let current_click_id = SUBMENU_CLICK_COUNTER.load(Ordering::Relaxed);
                        let click_pos = ctx.input(|i| i.pointer.interact_pos()).unwrap_or_default();
                        let submenu_rect =
                            Rect::from_min_size(submenu_position, Vec2::new(200.0, 100.0));

                        // Only close if this is a different click than the one that opened the submenu
                        if current_click_id > self.last_click_id {
                            // Close if click is outside submenu and not in menu bar
                            let menu_bar_rect = Rect::from_min_size(
                                Pos2::new(0.0, 0.0),
                                Vec2::new(ctx.screen_rect().width(), 32.0),
                            );

                            if !submenu_rect.contains(click_pos)
                                && !menu_bar_rect.contains(click_pos)
                            {
                                // Close all menus but keep keyboard navigation active
                                self.open_submenu = None;
                                self.force_open_child_subitem = None;
                                self.child_submenu_selections.clear();
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

    /// Render submenu as an overlay at a specific position (static version)
    /// Returns true if an item was clicked
    fn render_submenu_overlay_static(
        ui: &mut Ui,
        menu_item: &MenuItem,
        position: egui::Pos2,
        menu_text_size: f32,
        submenu_background_color: Color32,
        submenu_text_color: Color32,
        submenu_hover_color: Color32,
        submenu_shortcut_color: Color32,
        submenu_border_color: Color32,
        submenu_keyboard_selection_color: Color32,
        keyboard_navigation_active: bool,
        selected_submenu_index: Option<usize>,
        force_open_child_subitem: Option<usize>,
        selected_child_submenu_index: Option<usize>,
        parent_submenu_index: usize,
    ) -> bool {
        // Calculate submenu dimensions
        let item_height = 24.0;
        let padding = 8.0;
        let separator_height = 1.0;

        // Find the maximum width needed
        let mut max_width: f32 = 120.0; // Minimum width
        for subitem in &menu_item.subitems {
            let label_width = ui.fonts(|f| {
                f.layout_no_wrap(
                    subitem.label.clone(),
                    FontId::proportional(menu_text_size),
                    submenu_text_color,
                )
                .size()
                .x
            });
            let shortcut_width = if let Some(ref shortcut) = subitem.shortcut {
                ui.fonts(|f| {
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
            let total_width = label_width + shortcut_width + padding * 3.0 + 20.0; // Extra space for arrow
            max_width = max_width.max(total_width);
        }

        let total_height = (item_height * menu_item.subitems.len() as f32)
            + (separator_height
                * menu_item
                    .subitems
                    .iter()
                    .filter(|s| s.separator_after)
                    .count() as f32);

        // Position submenu
        let submenu_rect = egui::Rect::from_min_size(position, Vec2::new(max_width, total_height));

        // Ensure submenu stays within screen bounds
        let screen_rect = ui.ctx().screen_rect();
        let adjusted_rect = if submenu_rect.max.x > screen_rect.max.x {
            // Move left if it would go off screen
            Rect::from_min_size(
                Pos2::new(screen_rect.max.x - max_width, submenu_rect.min.y),
                submenu_rect.size(),
            )
        } else {
            submenu_rect
        };

        // Draw submenu background and border
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

        // Render submenu items
        let mut current_y = adjusted_rect.min.y;
        let mut item_clicked = false;
        for (i, subitem) in menu_item.subitems.iter().enumerate() {
            let item_rect = Rect::from_min_size(
                Pos2::new(adjusted_rect.min.x, current_y),
                Vec2::new(adjusted_rect.width(), item_height),
            );

            // Handle hover effect
            let response = ui.interact(
                item_rect,
                Id::new(format!("subitem_overlay_{}_{}", menu_item.label, i)),
                Sense::click(),
            );

            // Check if this submenu item is selected by keyboard navigation
            // Use main selection if available, otherwise use child selection
            let is_keyboard_selected = keyboard_navigation_active
                && (selected_submenu_index == Some(i)
                    || (selected_submenu_index.is_none()
                        && selected_child_submenu_index == Some(i)));

            if (response.hovered() || is_keyboard_selected) && subitem.enabled {
                let highlight_color = if is_keyboard_selected {
                    // Use configurable keyboard selection color for submenus
                    submenu_keyboard_selection_color
                } else {
                    submenu_hover_color
                };
                ui.painter()
                    .rect_filled(item_rect, CornerRadius::same(2), highlight_color);
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }

            // Render text and shortcut
            let text_color = if is_keyboard_selected {
                Color32::WHITE // White text on keyboard selection background
            } else if subitem.enabled {
                submenu_text_color
            } else {
                Color32::from_rgb(150, 150, 150)
            };

            // Main label (left aligned)
            ui.painter().text(
                Pos2::new(item_rect.min.x + padding, item_rect.center().y),
                Align2::LEFT_CENTER,
                &subitem.label,
                FontId::proportional(menu_text_size),
                text_color,
            );

            // Shortcut or child arrow (right aligned)
            if !subitem.children.is_empty() {
                // Draw a chevron using two line segments for reliable rendering across fonts
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

            // Handle click or hover-open for cascading child menus
            // Keep child menu open while the pointer travels from parent row to child (hover corridor)
            // and also while the pointer is inside the child submenu area itself.
            let mut open_child = false;
            if subitem.enabled && !subitem.children.is_empty() {
                if response.hovered() {
                    open_child = true;
                } else if let Some(ptr) = ui.ctx().input(|i| i.pointer.interact_pos()) {
                    // 1) Narrow corridor bridging parent item and child menu
                    let corridor_width = 10.0;
                    let corridor = Rect::from_min_max(
                        Pos2::new(item_rect.max.x, item_rect.min.y - 6.0),
                        Pos2::new(item_rect.max.x + corridor_width, item_rect.max.y + 6.0),
                    );

                    // 2) Approximate child submenu bounds (so moving into it keeps it open)
                    let mut child_max_width: f32 = 120.0; // minimum width
                    let padding = 8.0;
                    let item_height = 24.0;
                    let separator_height = 1.0;
                    for c in &subitem.children {
                        let label_width = ui.fonts(|f| {
                            f.layout_no_wrap(
                                c.label.clone(),
                                FontId::proportional(menu_text_size),
                                submenu_text_color,
                            )
                            .size()
                            .x
                        });
                        let shortcut_width = if let Some(ref s) = c.shortcut {
                            ui.fonts(|f| {
                                f.layout_no_wrap(
                                    s.display_string(),
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
                        child_max_width = child_max_width.max(total_width);
                    }
                    let child_total_height = (item_height * subitem.children.len() as f32)
                        + (separator_height
                            * subitem
                                .children
                                .iter()
                                .filter(|s| s.separator_after)
                                .count() as f32);

                    let mut child_rect = Rect::from_min_size(
                        Pos2::new(item_rect.max.x, item_rect.min.y),
                        Vec2::new(child_max_width, child_total_height),
                    );
                    // Keep child rect on screen if needed
                    let screen_rect = ui.ctx().screen_rect();
                    if child_rect.max.x > screen_rect.max.x {
                        let shift = child_rect.max.x - screen_rect.max.x;
                        child_rect = child_rect.translate(Vec2::new(-shift, 0.0));
                    }

                    if corridor.contains(ptr) || child_rect.contains(ptr) {
                        open_child = true;
                    }
                }
            }
            if response.clicked() && subitem.enabled && subitem.children.is_empty() {
                if let Some(ref callback) = subitem.callback {
                    callback();
                }
                item_clicked = true;
            }

            // Render cascading child menu if needed
            // Allow hover to open even in keyboard mode; keyboard can also force-open
            if open_child || (keyboard_navigation_active && force_open_child_subitem == Some(i)) {
                // Initialize child submenu selection if not set (for keyboard navigation)
                if keyboard_navigation_active && selected_child_submenu_index.is_none() {
                    // Initialize the first item as selected for this child submenu
                    // Note: We can't modify title_bar_ref here due to borrowing constraints
                    // This is a limitation of the current approach
                }

                let child_position = Pos2::new(item_rect.max.x, item_rect.min.y);
                let child_menu = MenuItem {
                    label: format!("{}_child", menu_item.label),
                    subitems: subitem.children.clone(),
                    enabled: true,
                };

                // Draw child menu
                let child_clicked = Self::render_submenu_overlay_static(
                    ui,
                    &child_menu,
                    child_position,
                    menu_text_size,
                    submenu_background_color,
                    submenu_text_color,
                    submenu_hover_color,
                    submenu_shortcut_color,
                    submenu_border_color,
                    submenu_keyboard_selection_color,
                    keyboard_navigation_active,
                    None,                         // Child menus don't use parent menu selection
                    None,                         // Child menus don't have forced open items
                    selected_child_submenu_index, // Pass child selection for highlighting
                    parent_submenu_index,         // Pass parent submenu index
                );

                // Propagate child menu click to parent
                if child_clicked {
                    item_clicked = true;
                }
            }

            current_y += item_height;

            // Add separator if needed
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

        item_clicked
    }
}
