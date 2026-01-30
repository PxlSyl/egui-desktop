use egui::{
    Color32, CursorIcon, Painter, Pos2, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2, vec2,
};

use crate::{TitleBar, titlebar::render_bar::title_bar_height};

/// Window control icon types used by the title bar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowControlIcon {
    /// Close the window.
    Close,
    /// Maximize the window.
    Maximize,
    /// Restore the window from maximized state.
    Restore,
    /// Minimize the window.
    Minimize,
}

impl TitleBar {
    /// Draw the close button icon (X shape)
    ///
    /// Draws two diagonal lines forming an X shape for the close button.
    ///
    /// # Arguments
    /// * `painter` - The egui painter to draw with
    /// * `rect` - The bounding rectangle for the icon
    /// * `color` - The color of the icon lines
    fn draw_close_icon(&self, painter: &egui::Painter, rect: Rect, color: Color32) {
        let center = rect.center();
        let size = rect.width().min(rect.height()) * 0.6;
        let half_size = size / 2.0;

        let stroke = Stroke::new(1.5, color);
        painter.line_segment(
            [
                center + Vec2::new(-half_size, -half_size),
                center + Vec2::new(half_size, half_size),
            ],
            stroke,
        );
        painter.line_segment(
            [
                center + Vec2::new(half_size, -half_size),
                center + Vec2::new(-half_size, half_size),
            ],
            stroke,
        );
    }

    /// Draw the maximize button icon (square shape)
    ///
    /// Draws a square outline representing the maximize button.
    ///
    /// # Arguments
    /// * `painter` - The egui painter to draw with
    /// * `rect` - The bounding rectangle for the icon
    /// * `color` - The color of the icon lines
    fn draw_maximize_icon(&self, painter: &Painter, rect: egui::Rect, color: Color32) {
        let center = rect.center();
        let size = rect.width().min(rect.height()) * 0.75;
        let stroke = Stroke::new(1.5, color);
        let square_rect = Rect::from_center_size(center, Vec2::new(size, size));
        painter.rect_stroke(square_rect, 0.0, stroke, StrokeKind::Inside);
    }

    /// Draw the restore button icon (overlapping squares)
    ///
    /// Draws a main square with two perpendicular lines representing an overlapping
    /// second square, indicating the restore down functionality.
    ///
    /// # Arguments
    /// * `painter` - The egui painter to draw with
    /// * `rect` - The bounding rectangle for the icon
    /// * `color` - The color of the icon lines
    fn draw_restore_icon(&self, painter: &Painter, rect: Rect, color: Color32) {
        let button_size = rect.width().min(rect.height());
        let square_size = button_size * 0.85;
        let icon_rect = Rect::from_center_size(rect.center(), Vec2::new(square_size, square_size));

        let center = icon_rect.center();
        let half_size = square_size / 2.0;

        let stroke = Stroke::new(1.5, color);

        let main_square_size = square_size * 0.7;
        let main_square_center = center + Vec2::new(-half_size * 0.2, 0.0);
        let main_square = Rect::from_center_size(
            main_square_center,
            Vec2::new(main_square_size, main_square_size),
        );
        painter.rect_stroke(main_square, 0.0, stroke, StrokeKind::Inside);

        let spacing = half_size * 0.12;

        let horizontal_start = center + Vec2::new(-half_size * 0.3, -half_size + spacing);
        let horizontal_end = center + Vec2::new(half_size - spacing, -half_size + spacing);

        let vertical_start = center + Vec2::new(half_size - spacing, -half_size + spacing);
        let vertical_end = center + Vec2::new(half_size - spacing, half_size * 0.2);

        painter.line_segment([horizontal_start, horizontal_end], stroke);
        painter.line_segment([vertical_start, vertical_end], stroke);
    }

    /// Draw the minimize button icon (horizontal line)
    ///
    /// Draws a horizontal line representing the minimize button.
    ///
    /// # Arguments
    /// * `painter` - The egui painter to draw with
    /// * `rect` - The bounding rectangle for the icon
    /// * `color` - The color of the icon line
    fn draw_minimize_icon(&self, painter: &Painter, rect: Rect, color: Color32) {
        let center = rect.center();
        let size = rect.width().min(rect.height()) * 0.8;
        let half_size = size / 2.0;

        let stroke = Stroke::new(2.0, color);
        painter.line_segment(
            [
                center + Vec2::new(-half_size, 0.0),
                center + Vec2::new(half_size, 0.0),
            ],
            stroke,
        );
    }

    /// Render a window control button with a drawn icon
    ///
    /// This method creates an interactive button for window controls (close, maximize,
    /// restore, minimize) with custom drawn icons instead of SVG images.
    ///
    /// # Arguments
    /// * `ui` - The egui UI context
    /// * `icon_type` - The type of icon to draw
    /// * `hover_color` - The background color when hovering
    /// * `icon_color` - The color of the icon
    /// * `icon_size` - The size of the icon
    ///
    /// # Returns
    /// * `egui::Response` - The interaction response for the button
    pub fn render_window_control_button_with_drawn_icon(
        &self,
        ui: &mut Ui,
        icon_type: WindowControlIcon,
        hover_color: Color32,
        icon_color: Color32,
        icon_size: f32,
    ) -> Response {
        let desired_size = Vec2::new(46.0, 32.0);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        if response.hovered() {
            ui.painter().rect_filled(rect, 2.0, hover_color);
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        let icon_rect = Rect::from_center_size(rect.center(), Vec2::new(icon_size, icon_size));

        let final_icon_color = if response.hovered() && hover_color == self.close_hover_color {
            Color32::WHITE
        } else {
            icon_color
        };

        match icon_type {
            WindowControlIcon::Close => {
                self.draw_close_icon(ui.painter(), icon_rect, final_icon_color)
            }
            WindowControlIcon::Maximize => {
                self.draw_maximize_icon(ui.painter(), icon_rect, final_icon_color)
            }
            WindowControlIcon::Restore => {
                self.draw_restore_icon(ui.painter(), icon_rect, final_icon_color)
            }
            WindowControlIcon::Minimize => {
                self.draw_minimize_icon(ui.painter(), icon_rect, final_icon_color)
            }
        }

        response
    }

    /// Draw the macOS close button icon (X shape).
    ///
    /// Renders two diagonal lines forming the standard macOS close glyph.
    ///
    /// # Arguments
    /// * `painter` - The egui painter used for rendering
    /// * `rect` - The bounding rectangle of the icon
    /// * `color` - The color of the icon strokes
    fn draw_mac_close_icon(&self, painter: &egui::Painter, rect: Rect, color: Color32) {
        let stroke = Stroke::new(1.5, color);
        painter.line_segment([rect.left_top(), rect.right_bottom()], stroke);
        painter.line_segment([rect.right_top(), rect.left_bottom()], stroke);
    }

    /// Draw the macOS miniaturize button icon (horizontal line).
    ///
    /// Renders a single horizontal line representing the macOS miniaturize glyph.
    ///
    /// # Arguments
    /// * `painter` - The egui painter used for rendering
    /// * `rect` - The bounding rectangle of the icon
    /// * `color` - The color of the icon stroke
    fn draw_mac_miniaturize_icon(&self, painter: &Painter, rect: egui::Rect, color: Color32) {
        let center = rect.center();
        let size = rect.width();
        let half_size = size / 2.0;
        let stroke = Stroke::new(1.5, color);
        painter.line_segment(
            [
                center + Vec2::new(-half_size, 0.0),
                center + Vec2::new(half_size, 0.0),
            ],
            stroke,
        );
    }

    /// Draw the macOS classic zoom button icon (plus sign).
    ///
    /// Renders two perpendicular lines forming a "+" glyph, used for classic zoom
    /// when the Option (⌥) key is pressed.
    ///
    /// # Arguments
    /// * `painter` - The egui painter used for rendering
    /// * `rect` - The bounding rectangle of the icon
    /// * `color` - The color of the icon strokes
    fn draw_mac_classic_zoom_icon(&self, painter: &Painter, rect: Rect, color: Color32) {
        let size = rect.width().min(rect.height());
        let center = rect.center();
        let half_size = size / 2.0;

        let stroke = Stroke::new(1.5, color);
        painter.line_segment(
            [
                center + Vec2::new(0.0, -half_size),
                center + Vec2::new(0.0, half_size),
            ],
            stroke,
        );
        painter.line_segment(
            [
                center + Vec2::new(-half_size, 0.0),
                center + Vec2::new(half_size, 0.0),
            ],
            stroke,
        );
    }

    /// Draw the macOS zoom-to-fullscreen button icon.
    ///
    /// Renders the standard macOS fullscreen zoom glyph composed of two opposing
    /// triangular arrows.
    ///
    /// # Arguments
    /// * `painter` - The egui painter used for rendering
    /// * `rect` - The bounding rectangle of the icon
    /// * `color` - The color of the icon strokes
    fn draw_mac_zoom_icon(&self, painter: &Painter, rect: Rect, color: Color32) {
        // Left top triangle
        painter.add(egui::Shape::convex_polygon(
            vec![
                rect.left_top(),
                rect.left_top() + vec2(rect.width() * 0.75, 0.0),
                rect.left_top() + vec2(0.0, rect.height() * 0.75),
            ],
            color,
            Stroke::NONE,
        ));
        // Right bottom triangle
        painter.add(egui::Shape::convex_polygon(
            vec![
                rect.right_bottom(),
                rect.right_bottom() - vec2(rect.width() * 0.75, 0.0),
                rect.right_bottom() - vec2(0.0, rect.height() * 0.75),
            ],
            color,
            Stroke::NONE,
        ));
    }

    /// Render a macOS-style traffic light window control button.
    ///
    /// Creates an interactive traffic light button (close, miniaturize, or zoom)
    /// with a custom-drawn icon, matching native macOS appearance and behavior.
    /// Icons are rendered procedurally instead of using image assets.
    ///
    /// # Arguments
    /// * `ui` - The egui UI context
    /// * `button_color` - The background color of the button
    /// * `icon_type` - Optional icon to render inside the button
    /// * `icon_color` - The color of the icon glyph
    /// * `size` - The diameter of the button
    ///
    /// # Returns
    /// * `egui::Response` - The interaction response for the button
    pub fn render_traffic_light(
        &self,
        ui: &mut Ui,
        button_color: Color32,
        icon_type: Option<WindowControlIcon>,
        icon_color: Color32,
        size: f32,
    ) -> egui::Response {
        let button_size = Vec2::new(size, size);
        let (button_id, button_rect) = ui.allocate_space(button_size);

        let y_center = title_bar_height() / 2.0;
        let centered_pos = Pos2::new(button_rect.center().x, y_center);
        let centered_rect = Rect::from_center_size(centered_pos, button_size);
        let response = ui.interact(centered_rect, button_id, Sense::click());
        let primary_mouse_down = ui
            .ctx()
            .input(|i| i.pointer.button_down(egui::PointerButton::Primary));

        let darken = response.hovered() && primary_mouse_down;
        let bkg = if darken {
            Color32::from_rgba_premultiplied(
                (button_color.r() as f32 * 0.75) as u8,
                (button_color.g() as f32 * 0.75) as u8,
                (button_color.b() as f32 * 0.75) as u8,
                button_color.a(),
            )
        } else {
            button_color
        };

        ui.painter().circle_filled(centered_pos, size / 2.0, bkg);
        ui.painter().circle_stroke(
            centered_pos,
            size / 2.0,
            Stroke::new(0.5, Color32::from_rgba_premultiplied(0, 0, 0, 30)),
        );

        if let Some(icon) = icon_type {
            let icon_color = if darken {
                Color32::from_rgba_premultiplied(
                    (icon_color.r() as f32 * 0.75) as u8,
                    (icon_color.g() as f32 * 0.75) as u8,
                    (icon_color.b() as f32 * 0.75) as u8,
                    icon_color.a(),
                )
            } else {
                icon_color
            };
            match icon {
                WindowControlIcon::Close => {
                    let rect = Rect::from_center_size(centered_pos, Vec2::new(6.0, 6.0));
                    self.draw_mac_close_icon(ui.painter(), rect, icon_color)
                }
                WindowControlIcon::Maximize => {
                    let rect = Rect::from_center_size(centered_pos, Vec2::new(6.0, 6.0));
                    self.draw_mac_zoom_icon(ui.painter(), rect, icon_color)
                }
                WindowControlIcon::Restore => {
                    let rect = Rect::from_center_size(centered_pos, Vec2::new(6.0, 6.0));
                    self.draw_mac_classic_zoom_icon(ui.painter(), rect, icon_color)
                }
                WindowControlIcon::Minimize => {
                    let rect = Rect::from_center_size(centered_pos, Vec2::new(8.0, 6.0));
                    self.draw_mac_miniaturize_icon(ui.painter(), rect, icon_color)
                }
            }
        }

        response
    }
}
