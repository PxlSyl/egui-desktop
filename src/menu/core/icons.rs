use crate::{TitleBar, titlebar::IconAnimationState};
use egui::{Color32, Painter, Pos2, Rect, Vec2};
use std::f32::consts::PI;

impl TitleBar {
    /// Draw three dots for overflow indicator
    pub fn draw_three_dots(&self, painter: &Painter, overflow_rect: Rect, dot_color: Color32) {
        let dot_radius = 2.0;
        let dot_spacing = 6.0;
        let start_x = overflow_rect.center().x - dot_spacing;
        let start_y = overflow_rect.center().y;

        for i in 0..3 {
            let x = start_x + i as f32 * dot_spacing;
            let dot_pos = Pos2::new(x, start_y);
            painter.circle_filled(dot_pos, dot_radius, dot_color);
        }
    }

    /// Draw static hamburger icon
    pub fn draw_static_hamburger(
        &self,
        painter: &Painter,
        overflow_rect: Rect,
        line_color: Color32,
        icon_size: f32,
        menu_bar_center_y: f32,
    ) {
        let line_width = icon_size * 0.6;
        let line_height = 1.5;
        let line_spacing = 3.0;
        let center = Pos2::new(overflow_rect.center().x, menu_bar_center_y);

        let start_x = center.x - line_width / 2.0;
        let hamburger_y1 = center.y - line_spacing - line_height;
        let hamburger_y2 = center.y - line_height / 2.0;
        let hamburger_y3 = center.y + line_spacing;

        // Top line
        let line_rect1 = Rect::from_min_size(
            Pos2::new(start_x, hamburger_y1),
            Vec2::new(line_width, line_height),
        );
        painter.rect_filled(line_rect1, 0.0, line_color);

        // Middle line
        let line_rect2 = Rect::from_min_size(
            Pos2::new(start_x, hamburger_y2),
            Vec2::new(line_width, line_height),
        );
        painter.rect_filled(line_rect2, 0.0, line_color);

        // Bottom line
        let line_rect3 = Rect::from_min_size(
            Pos2::new(start_x, hamburger_y3),
            Vec2::new(line_width, line_height),
        );
        painter.rect_filled(line_rect3, 0.0, line_color);
    }

    /// Draw animated hamburger icon that transforms to 3 dots with rotation
    pub fn draw_animated_hamburger(
        painter: &Painter,
        rect: Rect,
        color: Color32,
        icon_size: f32,
        state: &IconAnimationState,
        center_y: f32,
    ) {
        let center = Pos2::new(rect.center().x, center_y);
        let line_width = icon_size * 0.6;
        let line_height = 1.5;
        let line_spacing = 3.0;
        let dot_radius = 2.0;
        let dot_spacing = 6.0;

        let start_x = center.x - line_width / 2.0;
        let hamburger_y1 = center.y - line_spacing - line_height;
        let hamburger_y2 = center.y - line_height / 2.0;
        let hamburger_y3 = center.y + line_spacing;

        // Final positions for the 3 dots (same as draw_three_dots)
        let dot_start_x = center.x - dot_spacing;
        let dot1_final = Pos2::new(dot_start_x, center.y);
        let dot2_final = Pos2::new(dot_start_x + dot_spacing, center.y);
        let dot3_final = Pos2::new(dot_start_x + dot_spacing * 2.0, center.y);

        // Use the animation progress from state (managed by framework)
        let eased_progress = state.progress * state.progress * (3.0 - 2.0 * state.progress); // Smoothstep
        let rotation_angle = eased_progress * PI * 0.25; // 45 degrees rotation

        // Draw three elements with proper animation
        for i in 0..3 {
            let y = match i {
                0 => hamburger_y1,
                1 => hamburger_y2,
                2 => hamburger_y3,
                _ => hamburger_y2, // fallback
            };

            let final_pos = match i {
                0 => dot1_final,
                1 => dot2_final,
                2 => dot3_final,
                _ => dot2_final, // fallback
            };

            // ONLY animate when progress > 0, otherwise show EXACTLY like cross mode
            if eased_progress > 0.01 {
                // Interpolate between horizontal line and dot position
                let line_center_x = start_x + line_width / 2.0;
                let current_x = line_center_x + (final_pos.x - line_center_x) * eased_progress;
                let current_y = y + (final_pos.y - y) * eased_progress;

                // Calculate size transformation (from line to dot)
                let current_size = line_height + (dot_radius * 2.0 - line_height) * eased_progress;

                // Apply rotation around the final position
                let rotated_pos = if eased_progress > 0.01 {
                    let dx = current_x - final_pos.x;
                    let dy = current_y - final_pos.y;

                    let rotated_dx = dx * rotation_angle.cos() - dy * rotation_angle.sin();
                    let rotated_dy = dx * rotation_angle.sin() + dy * rotation_angle.cos();

                    Pos2::new(final_pos.x + rotated_dx, final_pos.y + rotated_dy)
                } else {
                    Pos2::new(current_x, current_y)
                };

                // Draw the element with smooth transition
                if eased_progress < 0.7 {
                    // Draw as rectangle (line) for first 70% of animation
                    let alpha = 1.0 - (eased_progress / 0.7);
                    if alpha > 0.01 {
                        let line_rect = Rect::from_min_size(
                            Pos2::new(
                                rotated_pos.x - current_size / 2.0,
                                rotated_pos.y - line_height / 2.0,
                            ),
                            Vec2::new(current_size, line_height),
                        );
                        painter.rect_filled(
                            line_rect,
                            0.0,
                            Color32::from_rgba_premultiplied(
                                color.r(),
                                color.g(),
                                color.b(),
                                (alpha * 255.0) as u8,
                            ),
                        );
                    }
                }

                if eased_progress > 0.3 {
                    // Draw as circle (dot) for last 70% of animation (overlap for smooth transition)
                    let alpha = if eased_progress < 0.7 {
                        (eased_progress - 0.3) / 0.7
                    } else {
                        1.0
                    };
                    if alpha > 0.01 {
                        painter.circle_filled(
                            rotated_pos,
                            current_size / 2.0,
                            Color32::from_rgba_premultiplied(
                                color.r(),
                                color.g(),
                                color.b(),
                                (alpha * 255.0) as u8,
                            ),
                        );
                    }
                }
            } else {
                // When progress = 0, draw EXACTLY like cross mode (horizontal line)
                let line_rect =
                    Rect::from_min_size(Pos2::new(start_x, y), Vec2::new(line_width, line_height));
                painter.rect_filled(line_rect, 0.0, color);
            }
        }
    }
}
