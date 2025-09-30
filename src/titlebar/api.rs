use std::borrow::Cow;

use egui::load::Bytes;
use egui::{
    Color32, CornerRadius, CursorIcon, Id, Image, ImageSource, Pos2, Rect, Sense, Ui, Vec2,
};

use crate::titlebar::CustomIconButton;
use crate::{CustomIcon, TitleBar, TitleBarOptions};

impl TitleBar {
    /// Convenience constructor for a title bar with a title
    ///
    /// This is a shorthand for creating a title bar with just a title.
    /// Uses default theme and settings.
    ///
    /// # Arguments
    /// * `title` - The title text to display in the title bar
    ///
    /// # Examples
    ///
    /// ```rust
    /// let title_bar = TitleBar::with_title("My Application");
    /// ```
    pub fn with_title(title: impl Into<String>) -> Self {
        Self::new(TitleBarOptions::new().with_title(title))
    }

    /// Convenience constructor for an icon-only title bar
    ///
    /// Creates a title bar with no title text, typically used for
    /// applications that only want to display an app icon on Windows 11 (No icons by default on mac os).
    ///
    /// # Examples
    ///
    /// ```rust
    /// let title_bar = TitleBar::icon_only();
    /// ```
    pub fn icon_only() -> Self {
        Self::new(TitleBarOptions::new())
    }

    /// Determine if the title should be displayed based on the current platform
    ///
    /// Returns true if the title should be shown on the current platform,
    /// false otherwise. This respects the platform-specific visibility settings.
    ///
    /// # Returns
    /// * `bool` - True if title should be displayed, false otherwise
    pub fn should_show_title(&self) -> bool {
        #[cfg(target_os = "macos")]
        {
            self.show_title_on_macos
        }
        #[cfg(target_os = "windows")]
        {
            self.show_title_on_windows
        }
        #[cfg(target_os = "linux")]
        {
            self.show_title_on_linux
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        {
            false // Default to not showing title on unknown platforms
        }
    }

    /// Set the background color of the title bar
    ///
    /// # Arguments
    /// * `color` - The background color as a Color32
    ///
    /// # Examples
    ///
    /// ```rust
    /// title_bar.with_background_color(Color32::from_rgb(240, 240, 240))
    /// ```
    pub fn with_background_color(mut self, color: Color32) -> Self {
        self.background_color = color;
        self
    }

    /// Set the hover color for control buttons
    ///
    /// This color is used when hovering over maximize, restore, and minimize buttons.
    ///
    /// # Arguments
    /// * `color` - The hover color as a Color32
    ///
    /// # Examples
    ///
    /// ```rust
    /// title_bar.with_hover_color(Color32::from_rgb(220, 220, 220))
    /// ```
    pub fn with_hover_color(mut self, color: Color32) -> Self {
        self.hover_color = color;
        self
    }

    /// Set the hover color specifically for the close button
    ///
    /// This color is used when hovering over the close button.
    /// Typically red to indicate a destructive action.
    ///
    /// # Arguments
    /// * `color` - The close button hover color as a Color32
    ///
    /// # Examples
    ///
    /// ```rust
    /// title_bar.with_close_hover_color(Color32::from_rgb(232, 17, 35))
    /// ```
    pub fn with_close_hover_color(mut self, color: Color32) -> Self {
        self.close_hover_color = color;
        self
    }

    /// Set the color of the close button icon
    ///
    /// This color is used for the close button icon in its normal state.
    /// The icon becomes white when hovering over the close button.
    ///
    /// # Arguments
    /// * `color` - The close icon color as a Color32
    ///
    /// # Examples
    ///
    /// ```rust
    /// title_bar.with_close_icon_color(Color32::from_rgb(100, 100, 100))
    /// ```
    pub fn with_close_icon_color(mut self, color: Color32) -> Self {
        self.close_icon_color = color;
        self
    }

    /// Set the color of the title text
    ///
    /// # Arguments
    /// * `color` - The title text color as a Color32
    ///
    /// # Examples
    ///
    /// ```rust
    /// title_bar.with_title_color(Color32::from_rgb(50, 50, 50))
    /// ```
    pub fn with_title_color(mut self, color: Color32) -> Self {
        self.title_color = color;
        self
    }

    /// Set the font size of the title text
    ///
    /// # Arguments
    /// * `size` - The font size in points
    ///
    /// # Examples
    ///
    /// ```rust
    /// title_bar.with_title_font_size(14.0)
    /// ```
    pub fn with_title_font_size(mut self, size: f32) -> Self {
        self.title_font_size = size;
        self
    }

    /// Add a custom icon to the title bar
    ///
    /// The framework automatically positions the icon based on the platform:
    /// - Windows/Linux: To the left of window control buttons
    /// - macOS: To the right of traffic light buttons
    ///
    /// # Arguments
    /// * `icon` - The custom icon (image or drawing function)
    /// * `callback` - Optional callback function when the icon is clicked
    /// * `tooltip` - Optional tooltip text to show on hover
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Add a settings icon with tooltip - automatically positioned by platform
    /// title_bar.add_icon(
    ///     CustomIcon::Image(ImageSource::from_bytes("settings.svg", include_bytes!("settings.svg"))),
    ///     Some(Box::new(|| println!("Settings clicked!"))),
    ///     Some("Settings".to_string())
    /// );
    ///
    /// // Add a custom drawn notification bell with tooltip
    /// title_bar.add_icon(
    ///     CustomIcon::Drawn(Box::new(|painter, rect, color| {
    ///         // Custom drawing code here
    ///     })),
    ///     Some(Box::new(|| println!("Notification clicked!"))),
    ///     Some("Notifications".to_string())
    /// );
    /// ```
    pub fn add_icon(
        mut self,
        icon: CustomIcon,
        callback: Option<Box<dyn Fn() + Send + Sync>>,
        tooltip: Option<String>,
        shortcut: Option<crate::KeyboardShortcut>,
    ) -> Self {
        self.custom_icons.push(CustomIconButton {
            icon,
            tooltip,
            hover_color: None,
            icon_color: None,
            callback,
            shortcut,
        });
        // Keep animation states aligned
        if let CustomIcon::Animated(_) = self.custom_icons.last().unwrap().icon {
            self.icon_animation_states.push(Default::default());
        }
        self
    }

    /// Add an animated icon with tooltip and optional callback/shortcut
    pub fn add_animated_icon(
        mut self,
        draw: Box<
            dyn Fn(
                    &egui::Painter,
                    egui::Rect,
                    egui::Color32,
                    &mut crate::titlebar::IconAnimationState,
                    crate::titlebar::AnimationCtx,
                ) + Send
                + Sync,
        >,
        callback: Option<Box<dyn Fn() + Send + Sync>>,
        tooltip: Option<String>,
        shortcut: Option<crate::KeyboardShortcut>,
    ) -> Self {
        self = self.add_icon(CustomIcon::Animated(draw), callback, tooltip, shortcut);
        self
    }

    /// Add an animated Ui-based icon (render via Ui) with tooltip and optional callback/shortcut
    pub fn add_animated_ui_icon(
        mut self,
        draw: Box<
            dyn Fn(
                    &mut egui::Ui,
                    egui::Rect,
                    egui::Color32,
                    &mut crate::titlebar::IconAnimationState,
                    crate::titlebar::AnimationCtx,
                ) + Send
                + Sync,
        >,
        callback: Option<Box<dyn Fn() + Send + Sync>>,
        tooltip: Option<String>,
        shortcut: Option<crate::KeyboardShortcut>,
    ) -> Self {
        self = self.add_icon(CustomIcon::AnimatedUi(draw), callback, tooltip, shortcut);
        self
    }

    /// Check if any custom icon shortcut was pressed and execute the callback
    /// Call this in your app's update loop to handle icon shortcuts
    pub fn handle_icon_shortcuts(&self, ctx: &egui::Context) {
        for icon_button in &self.custom_icons {
            if let Some(shortcut) = &icon_button.shortcut {
                if shortcut.just_pressed(ctx) {
                    if let Some(callback) = &icon_button.callback {
                        callback();
                    }
                }
            }
        }
    }

    /// Update the color of a custom icon at a given index
    /// Pass None to revert to default icon color logic
    pub fn set_custom_icon_color(&mut self, index: usize, color: Option<Color32>) {
        if let Some(button) = self.custom_icons.get_mut(index) {
            button.icon_color = color;
        }
    }

    /// Create a custom app icon from image bytes (supports SVG, PNG, JPEG, etc.)
    ///
    /// This function automatically detects the format and creates the appropriate ImageSource.
    /// The icon will be displayed in the title bar next to the title (on Windows/Linux).
    ///
    /// # Arguments
    /// * `bytes` - The image data as bytes
    /// * `uri` - A unique identifier for the image (e.g., "my_app_icon.svg")
    ///
    /// # Examples
    ///
    /// ```rust
    /// title_bar.with_app_icon(include_bytes!("icon.svg"), "app_icon.svg")
    /// ```
    pub fn with_app_icon(mut self, bytes: &'static [u8], uri: &'static str) -> Self {
        let icon = ImageSource::Bytes {
            uri: Cow::Borrowed(uri),
            bytes: Bytes::Static(bytes),
        };
        self.app_icon = Some(icon);
        self
    }

    /// Render custom icons in the title bar
    ///
    /// This method renders all custom icon buttons automatically positioned
    /// based on the platform.
    pub fn render_custom_icons(&mut self, ui: &mut Ui) {
        if self.custom_icons.is_empty() {
            return;
        }

        let icon_size = 16.0;
        let spacing = self.icon_spacing;
        let icon_height = 28.0; // Standard icon height

        // Calculate total width needed for all icons + extra spacing
        let extra_spacing = 16.0; // Extra space between custom icons and window controls
        let total_width =
            self.custom_icons.len() as f32 * (icon_size + spacing) - spacing + extra_spacing;

        // Allocate space for the entire icon bar
        let (icon_bar_rect, _) =
            ui.allocate_exact_size(Vec2::new(total_width, icon_height), Sense::click());

        let mut current_x = icon_bar_rect.max.x - extra_spacing;

        // Ensure states vector matches icons length
        if self.icon_animation_states.len() < self.custom_icons.len() {
            self.icon_animation_states
                .resize(self.custom_icons.len(), Default::default());
        }

        let now = ui.input(|i| i.time);

        for index in 0..self.custom_icons.len() {
            let icon_button = &self.custom_icons[index];
            let icon_id = Id::new(format!("custom_icon_{}", index));

            // Create individual icon rect (positioned from right to left)
            let icon_rect = Rect::from_min_size(
                Pos2::new(current_x - icon_size, icon_bar_rect.min.y + 6.0),
                Vec2::new(icon_size, icon_size),
            );

            // Handle interaction
            let mut response = ui.interact(icon_rect, icon_id, Sense::click());

            // Show tooltip if available (include shortcut if present)
            if let Some(ref tooltip) = icon_button.tooltip {
                let tooltip_text = if let Some(ref shortcut) = icon_button.shortcut {
                    format!("{} ({})", tooltip, shortcut.display_string())
                } else {
                    tooltip.clone()
                };
                response = response.on_hover_text(tooltip_text);
            }

            // Handle hover effect (render background first)
            if response.hovered() {
                let hover_color = icon_button.hover_color.unwrap_or(self.hover_color);
                ui.painter()
                    .rect_filled(icon_rect.expand(2.0), CornerRadius::same(2), hover_color);
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }

            // Render the icon (always rendered on top)
            let icon_color = icon_button.icon_color.unwrap_or(self.menu_text_color);
            match &icon_button.icon {
                CustomIcon::Image(image_source) => {
                    let image = Image::new(image_source.clone())
                        .fit_to_exact_size(Vec2::new(icon_size, icon_size));
                    ui.put(icon_rect, image);
                }
                CustomIcon::Drawn(draw_fn) => {
                    draw_fn(ui.painter(), icon_rect, icon_color);
                }
                CustomIcon::Animated(draw_fn) => {
                    let hovered = response.hovered();
                    let pressed = response.is_pointer_button_down_on();

                    let state = &mut self.icon_animation_states[index];
                    let prev_time = state.last_time;
                    let dt = if prev_time == 0.0 {
                        0.0
                    } else {
                        (now - prev_time) as f32
                    };
                    state.last_time = now;

                    let speed = 8.0;
                    let target_hover = if hovered { 1.0 } else { 0.0 };
                    state.hover_t += (target_hover - state.hover_t) * (1.0 - (-speed * dt).exp());

                    let target_press = if pressed { 1.0 } else { 0.0 };
                    state.press_t += (target_press - state.press_t) * (1.0 - (-12.0 * dt).exp());

                    let ctx = crate::titlebar::AnimationCtx {
                        time: now,
                        delta_seconds: dt,
                        hovered,
                        pressed,
                    };

                    draw_fn(ui.painter(), icon_rect, icon_color, state, ctx);
                }
                CustomIcon::AnimatedUi(draw_fn) => {
                    let hovered = response.hovered();
                    let pressed = response.is_pointer_button_down_on();

                    let state = &mut self.icon_animation_states[index];
                    let prev_time = state.last_time;
                    let dt = if prev_time == 0.0 {
                        0.0
                    } else {
                        (now - prev_time) as f32
                    };
                    state.last_time = now;

                    let speed = 8.0;
                    let target_hover = if hovered { 1.0 } else { 0.0 };
                    state.hover_t += (target_hover - state.hover_t) * (1.0 - (-speed * dt).exp());

                    let target_press = if pressed { 1.0 } else { 0.0 };
                    state.press_t += (target_press - state.press_t) * (1.0 - (-12.0 * dt).exp());

                    let ctx = crate::titlebar::AnimationCtx {
                        time: now,
                        delta_seconds: dt,
                        hovered,
                        pressed,
                    };

                    draw_fn(ui, icon_rect, icon_color, state, ctx);
                }
            }

            // Handle click
            if response.clicked() {
                if let Some(ref callback) = icon_button.callback {
                    callback();
                }
                // Ensure the next frame runs so animations start immediately
                ui.ctx().request_repaint();
            }

            // Move to next icon position (from right to left)
            current_x -= icon_size + spacing;
        }
    }

    fn get_default_app_icon(&self) -> ImageSource<'static> {
        const DEFAULT_APP_ICON_SVG: &[u8] = include_bytes!("egui-desktop-ui.svg");
        ImageSource::Bytes {
            uri: Cow::Borrowed("egui-desktop-ui.svg"),
            bytes: Bytes::Static(DEFAULT_APP_ICON_SVG),
        }
    }

    /// Get the configured app icon, or a built-in default icon.
    pub fn get_app_icon(&self) -> ImageSource<'static> {
        self.app_icon
            .clone()
            .unwrap_or_else(|| self.get_default_app_icon())
    }
}
