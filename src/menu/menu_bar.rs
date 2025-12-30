use egui::{Align2, Color32, CursorIcon, FontId, Sense, TextStyle, Ui, Vec2};

/// A minimal horizontal menu bar composed of clickable items.
///
/// This is a lightweight helper primarily used by the title bar menu system.
/// Each `MenuItem` can optionally have an action callback that is invoked on click.
pub struct MenuBar {
    items: Vec<MenuItem>,
}

/// A single item displayed in the `MenuBar`.
pub struct MenuItem {
    /// Visible text in the bar.
    pub label: String,
    /// Optional callback executed when the item is clicked.
    pub action: Option<Box<dyn Fn() + Send + Sync>>,
}

impl MenuBar {
    /// Create an empty `MenuBar`.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Append a new clickable item to the menu bar.
    ///
    /// Returns `self` for fluent chaining.
    ///
    /// # Arguments
    /// - `label`: Text to display
    /// - `action`: Optional callback invoked on click
    pub fn add_item(mut self, label: &str, action: Option<Box<dyn Fn() + Send + Sync>>) -> Self {
        self.items.push(MenuItem {
            label: label.to_string(),
            action,
        });
        self
    }

    /// Render the menu bar into the given `egui::Ui`.
    ///
    /// Each item is laid out horizontally and becomes highlighted on hover.
    /// When clicked, the item's `action` callback (if any) is executed.
    pub fn render(&self, ui: &mut Ui) {
        let item_height = 28.0; // Match title bar height

        for item in &self.items {
            let item_width = ui.fonts_mut(|f| {
                f.layout_no_wrap(
                    item.label.clone(),
                    FontId::proportional(14.0), // Standard menu font size
                    Color32::WHITE,             // Will be overridden by theme
                )
                .size()
                .x
            }) + 16.0;
            let (rect, response) =
                ui.allocate_exact_size(Vec2::new(item_width, item_height), Sense::click());

            if response.hovered() {
                ui.painter()
                    .rect_filled(rect, 2.0, Color32::from_rgb(50, 50, 50));
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }

            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                &item.label,
                TextStyle::Body.resolve(ui.style()),
                Color32::from_rgb(200, 200, 200),
            );

            if response.clicked() {
                if let Some(action) = &item.action {
                    action();
                }
            }
        }
    }
}

impl Default for MenuBar {
    fn default() -> Self {
        Self::new()
    }
}
