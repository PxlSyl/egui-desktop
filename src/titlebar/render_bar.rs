use egui::{
    Align, Align2, Color32, Context, FontId, Frame, Image, Layout, Margin, PointerButton, Pos2,
    Rect, Sense, TextStyle, TopBottomPanel, Vec2, ViewportCommand,
};

use crate::{TitleBar, titlebar::control_buttons::WindowControlIcon};

/// Get the title bar height based on the platform.
/// - macOS: 28.0 pixels (thinner title bar)
/// - Windows/Linux: 32.0 pixels (standard title bar)
pub fn title_bar_height() -> f32 {
    #[cfg(target_os = "macos")]
    {
        28.0
    }
    #[cfg(not(target_os = "macos"))]
    {
        32.0
    }
}

impl TitleBar {
    /// Display the title bar in the egui context
    ///
    /// This is the main method to render the title bar. It automatically
    /// chooses the appropriate rendering method based on the platform:
    /// - macOS: Uses native traffic light buttons
    /// - Windows/Linux: Uses generic window control buttons
    ///
    /// # Arguments
    /// * `ctx` - The egui context
    ///
    /// # Examples
    ///
    /// ```rust
    /// fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
    ///     self.title_bar.show(ctx);
    ///     
    ///     CentralPanel::default().show(ctx, |ui| {
    ///         ui.label("Main content");
    ///     });
    /// }
    /// ```
    pub fn show(&mut self, ctx: &Context) {
        #[cfg(target_os = "macos")]
        {
            self.render_macos_title_bar(ctx);
        }

        #[cfg(not(target_os = "macos"))]
        {
            self.render_generic_title_bar(ctx);
        }
    }

    /// Render a macOS-style title bar with traffic light controls.
    pub fn render_macos_title_bar(&mut self, ctx: &Context) {
        let content_rect = ctx.content_rect();
        if content_rect.width() < 100.0 || content_rect.height() < 100.0 {
            return;
        }

        TopBottomPanel::top(self.id)
            .exact_height(28.0)
            .frame(
                Frame::new()
                    .fill(self.background_color)
                    .inner_margin(Margin::same(0))
                    .outer_margin(Margin::same(0)),
            )
            .show(ctx, |ui| {
                let title_bar_rect = ui.available_rect_before_wrap();

                if title_bar_rect.width() <= 0.0 || title_bar_rect.height() <= 0.0 {
                    return;
                }

                let title_bar_response =
                    ui.interact(title_bar_rect, self.id, Sense::click_and_drag());

                if title_bar_response.drag_started_by(PointerButton::Primary) {
                    ctx.send_viewport_cmd(ViewportCommand::StartDrag);
                }

                if title_bar_response.double_clicked() {
                    let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
                    ctx.send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
                }

                ui.horizontal(|ui| {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        ui.add_space(8.0);

                        let close_response = self
                            .render_traffic_light(ui, Color32::from_rgb(255, 95, 87), 12.0)
                            .on_hover_text("Close");

                        if close_response.clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Close);
                        }

                        ui.add_space(6.0);

                        let minimize_response = self
                            .render_traffic_light(ui, Color32::from_rgb(255, 189, 46), 12.0)
                            .on_hover_text("Minimize");

                        if minimize_response.clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Minimized(true));
                        }

                        ui.add_space(6.0);

                        let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
                        let maximize_response = self
                            .render_traffic_light(ui, Color32::from_rgb(40, 201, 55), 12.0)
                            .on_hover_text(if is_maximized { "Restore" } else { "Maximize" });

                        if maximize_response.clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
                        }

                        ui.add_space(16.0);

                        self.render_menu_items(ui, ctx);
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        self.render_custom_icons(ui);
                        ui.add_space(8.0);
                    });
                });

                if let Some(ref title_text) = self.title {
                    if self.should_show_title() {
                        let font = TextStyle::Body.resolve(ui.style());
                        let galley = ui.fonts_mut(|f| {
                            f.layout_no_wrap(title_text.clone(), font, self.title_color)
                        });

                        let center_x = title_bar_rect.center().x;
                        let center_y = title_bar_rect.min.y + 14.0;

                        let title_pos = Pos2::new(
                            center_x - galley.size().x / 2.0,
                            center_y - galley.size().y / 2.0,
                        );

                        ui.painter().galley(title_pos, galley, self.title_color);
                    }
                }
            });

        self.render_open_submenu(ctx);
    }

    /// Render a platform-generic title bar (Windows/Linux-style).
    pub fn render_generic_title_bar(&mut self, ctx: &Context) {
        let content_rect = ctx.content_rect();
        if content_rect.width() < 100.0 || content_rect.height() < 100.0 {
            return;
        }

        TopBottomPanel::top(self.id)
            .exact_height(32.0)
            .frame(
                Frame::new()
                    .fill(self.background_color)
                    .inner_margin(Margin::same(0))
                    .outer_margin(Margin::same(0)),
            )
            .show(ctx, |ui| {
                let title_bar_rect = ui.available_rect_before_wrap();

                if title_bar_rect.width() <= 0.0 || title_bar_rect.height() <= 0.0 {
                    return;
                }

                let title_bar_response =
                    ui.interact(title_bar_rect, self.id, Sense::click_and_drag());

                if title_bar_response.drag_started_by(PointerButton::Primary) {
                    ctx.send_viewport_cmd(ViewportCommand::StartDrag);
                }

                if title_bar_response.double_clicked() {
                    let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
                    ctx.send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
                }

                ui.horizontal(|ui| {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        let icon_size = 20.0;
                        let title_bar_height = 32.0;
                        let icon_center_y = title_bar_height / 2.0;

                        let icon_response = ui.allocate_rect(
                            Rect::from_center_size(
                                Pos2::new(16.0, icon_center_y),
                                Vec2::new(icon_size, icon_size),
                            ),
                            Sense::click(),
                        );

                        ui.put(
                            icon_response.rect,
                            Image::new(self.get_app_icon())
                                .fit_to_exact_size(Vec2::new(icon_size, icon_size)),
                        );

                        if let Some(ref title) = self.title {
                            if self.should_show_title() {
                                let title_width = ui.fonts_mut(|f| {
                                    f.layout_no_wrap(
                                        title.clone(),
                                        FontId::proportional(self.title_font_size),
                                        self.title_color,
                                    )
                                    .size()
                                    .x
                                }) + 8.0;
                                let title_response = ui.allocate_response(
                                    Vec2::new(title_width, 32.0),
                                    Sense::hover(),
                                );

                                let painter = ui.painter();
                                painter.text(
                                    Pos2::new(title_response.rect.left() + 4.0, icon_center_y),
                                    Align2::LEFT_CENTER,
                                    title,
                                    FontId::proportional(self.title_font_size),
                                    self.title_color,
                                );
                            }
                        }

                        self.render_menu_items(ui, ctx);
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.spacing_mut().item_spacing = Vec2::ZERO;

                        let close_response = self
                            .render_window_control_button_with_drawn_icon(
                                ui,
                                WindowControlIcon::Close,
                                self.close_hover_color,
                                self.close_icon_color,
                                16.0,
                            )
                            .on_hover_text("Close");

                        if close_response.clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Close);
                        }

                        let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));

                        let maximize_response = self
                            .render_window_control_button_with_drawn_icon(
                                ui,
                                if is_maximized {
                                    WindowControlIcon::Restore
                                } else {
                                    WindowControlIcon::Maximize
                                },
                                self.hover_color,
                                if is_maximized {
                                    self.restore_icon_color
                                } else {
                                    self.maximize_icon_color
                                },
                                14.0,
                            )
                            .on_hover_text(if is_maximized { "Restore" } else { "Maximize" });

                        if maximize_response.clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
                        }

                        let minimize_response = self
                            .render_window_control_button_with_drawn_icon(
                                ui,
                                WindowControlIcon::Minimize,
                                self.hover_color,
                                self.minimize_icon_color,
                                14.0,
                            )
                            .on_hover_text("Minimize");

                        if minimize_response.clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Minimized(true));
                        }

                        self.render_custom_icons(ui);
                    });
                });
            });

        self.render_open_submenu(ctx);
    }
}
