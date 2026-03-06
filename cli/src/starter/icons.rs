use egui::{Color32, Painter, Pos2, Rect, Stroke};

/// Draw a gear icon using egui primitives
pub fn draw_gear_icon(painter: &Painter, rect: Rect, color: Color32) {
    let center = rect.center();
    let size = rect.width().min(rect.height());
    let radius = size * 0.4;
    let inner_radius = size * 0.2;

    for i in 0..8 {
        let angle = i as f32 * std::f32::consts::PI / 4.0;
        let outer_x = center.x + radius * angle.cos();
        let outer_y = center.y + radius * angle.sin();
        let inner_x = center.x + inner_radius * angle.cos();
        let inner_y = center.y + inner_radius * angle.sin();

        painter.line_segment(
            [Pos2::new(inner_x, inner_y), Pos2::new(outer_x, outer_y)],
            Stroke::new(2.0, color),
        );
    }

    painter.circle(
        center,
        inner_radius * 0.6,
        Color32::TRANSPARENT,
        Stroke::new(2.0, color),
    );

    painter.circle(
        center,
        inner_radius * 0.3,
        Color32::TRANSPARENT,
        Stroke::new(1.5, color),
    );
}
