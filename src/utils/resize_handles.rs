use egui::{
    Area, Context, CursorIcon, Id, PointerButton, Pos2, ResizeDirection, Sense, Vec2,
    ViewportCommand,
};

/// Render invisible viewport resize handles around the window.
pub fn render_resize_handles(ctx: &Context) {
    let screen_rect = ctx.screen_rect();
    let resize_handle_size = 8.0;

    if screen_rect.width() < 100.0 || screen_rect.height() < 100.0 {
        return;
    }

    create_resize_handle(
        ctx,
        "resize_top",
        "resize_top_handle",
        Pos2::new(screen_rect.min.x, screen_rect.min.y),
        Vec2::new(screen_rect.width(), resize_handle_size),
        CursorIcon::ResizeVertical,
        ResizeDirection::North,
    );

    create_resize_handle(
        ctx,
        "resize_bottom",
        "resize_bottom_handle",
        Pos2::new(screen_rect.min.x, screen_rect.max.y - resize_handle_size),
        Vec2::new(screen_rect.width(), resize_handle_size),
        CursorIcon::ResizeVertical,
        ResizeDirection::South,
    );

    create_resize_handle(
        ctx,
        "resize_left",
        "resize_left_handle",
        Pos2::new(screen_rect.min.x, screen_rect.min.y),
        Vec2::new(resize_handle_size, screen_rect.height()),
        CursorIcon::ResizeHorizontal,
        ResizeDirection::West,
    );

    create_resize_handle(
        ctx,
        "resize_right",
        "resize_right_handle",
        Pos2::new(screen_rect.max.x - resize_handle_size, screen_rect.min.y),
        Vec2::new(resize_handle_size, screen_rect.height()),
        CursorIcon::ResizeHorizontal,
        ResizeDirection::East,
    );

    let corner_size = resize_handle_size * 1.5;

    if screen_rect.width() > corner_size * 2.0 && screen_rect.height() > corner_size * 2.0 {
        create_resize_handle(
            ctx,
            "resize_top_left",
            "resize_top_left_handle",
            Pos2::new(screen_rect.min.x, screen_rect.min.y),
            Vec2::new(corner_size, corner_size),
            CursorIcon::ResizeNwSe,
            ResizeDirection::NorthWest,
        );

        create_resize_handle(
            ctx,
            "resize_top_right",
            "resize_top_right_handle",
            Pos2::new(screen_rect.max.x - corner_size, screen_rect.min.y),
            Vec2::new(corner_size, corner_size),
            CursorIcon::ResizeNeSw,
            ResizeDirection::NorthEast,
        );

        create_resize_handle(
            ctx,
            "resize_bottom_left",
            "resize_bottom_left_handle",
            Pos2::new(screen_rect.min.x, screen_rect.max.y - corner_size),
            Vec2::new(corner_size, corner_size),
            CursorIcon::ResizeNeSw,
            ResizeDirection::SouthWest,
        );

        create_resize_handle(
            ctx,
            "resize_bottom_right",
            "resize_bottom_right_handle",
            Pos2::new(
                screen_rect.max.x - corner_size,
                screen_rect.max.y - corner_size,
            ),
            Vec2::new(corner_size, corner_size),
            CursorIcon::ResizeNwSe,
            ResizeDirection::SouthEast,
        );
    }
}

fn create_resize_handle(
    ctx: &Context,
    area_id: &str,
    handle_id: &str,
    position: Pos2,
    size: Vec2,
    cursor_icon: CursorIcon,
    resize_direction: ResizeDirection,
) {
    Area::new(Id::new(area_id))
        .fixed_pos(position)
        .show(ctx, |ui| {
            ui.set_min_size(size);
            let (_id, response) = ui.allocate_space(size);
            let interaction_response =
                ui.interact(response, Id::new(handle_id), Sense::click_and_drag());

            if interaction_response.hovered() {
                ctx.set_cursor_icon(cursor_icon);
            }

            if interaction_response.drag_started_by(PointerButton::Primary) {
                ctx.send_viewport_cmd(ViewportCommand::BeginResize(resize_direction));
            }
        });
}
