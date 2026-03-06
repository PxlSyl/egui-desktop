use crate::app::{AppTheme, CustomThemeDemoApp};
use egui::{ScrollArea, Ui};
use egui_desktop::detect_system_dark_mode;

pub fn render_sidebar(app: &mut CustomThemeDemoApp, ui: &mut Ui) {
    ScrollArea::vertical()
        .max_width(ui.available_width())
        .show(ui, |ui| {
            ui.add_space(6.0);
            ui.colored_label(app.get_text_color(ui), "🎨 Theme Customization");
            ui.separator();

            ui.collapsing("🌓 Theme Selection", |ui| {
                ui.vertical(|ui| {
                    ui.radio_value(&mut app.app_theme, AppTheme::Light, "☀️ Default Light");
                    ui.add_space(4.0);
                    ui.radio_value(&mut app.app_theme, AppTheme::Dark, "🌙 Default Dark");
                    ui.add_space(4.0);
                    ui.radio_value(&mut app.app_theme, AppTheme::Auto, "🖥️ Auto (System)");

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(6.0);
                    ui.colored_label(app.get_text_color(ui), "Custom themes:");
                    ui.add_space(6.0);

                    let ocean_light_selected = matches!(app.app_theme, AppTheme::CustomLight)
                        && app.selected_custom_id == "ocean";
                    let ocean_dark_selected = matches!(app.app_theme, AppTheme::CustomDark)
                        && app.selected_custom_id == "ocean";
                    if ui.radio(ocean_light_selected, "🌊 Ocean (Light)").clicked() {
                        app.selected_custom_id = "ocean".to_string();
                        app.app_theme = AppTheme::CustomLight;
                    }
                    ui.add_space(4.0);
                    if ui.radio(ocean_dark_selected, "🌊 Ocean (Dark)").clicked() {
                        app.selected_custom_id = "ocean".to_string();
                        app.app_theme = AppTheme::CustomDark;
                    }

                    ui.add_space(8.0);

                    let forest_light_selected = matches!(app.app_theme, AppTheme::CustomLight)
                        && app.selected_custom_id == "forest";
                    let forest_dark_selected = matches!(app.app_theme, AppTheme::CustomDark)
                        && app.selected_custom_id == "forest";
                    if ui
                        .radio(forest_light_selected, "🌲 Forest (Light)")
                        .clicked()
                    {
                        app.selected_custom_id = "forest".to_string();
                        app.app_theme = AppTheme::CustomLight;
                    }
                    ui.add_space(4.0);
                    if ui.radio(forest_dark_selected, "🌲 Forest (Dark)").clicked() {
                        app.selected_custom_id = "forest".to_string();
                        app.app_theme = AppTheme::CustomDark;
                    }
                });
            });

            ui.add_space(12.0);

            ui.collapsing("📋 Current Theme", |ui| {
                ui.vertical(|ui| {
                    ui.label(format!("Selected: {:?}", app.app_theme));
                    ui.add_space(8.0);

                    match app.app_theme {
                        AppTheme::Light => {
                            ui.label("• Default light theme");
                            ui.label("• White background");
                            ui.label("• Dark text and icons");
                        }
                        AppTheme::Dark => {
                            ui.label("• Default dark theme");
                            ui.label("• Dark background");
                            ui.label("• Light text and icons");
                        }
                        AppTheme::CustomLight => {
                            ui.label("• Custom light theme");
                            ui.label("• Light blue background");
                            ui.label("• Larger menu text (14px)");
                            ui.label("• Larger title text (16px)");
                            ui.label("• Blue control icons");
                            ui.label("• Cohesive color palette");
                            ui.label("• Custom hover effects");
                        }
                        AppTheme::CustomDark => {
                            ui.label("• Custom dark theme");
                            ui.label("• Dark navy background");
                            ui.label("• Larger menu text (14px)");
                            ui.label("• Larger title text (16px)");
                            ui.label("• Light blue control icons");
                            ui.label("• Cohesive color palette");
                            ui.label("• Custom hover effects");
                        }
                        AppTheme::Auto => {
                            ui.label("• Auto theme");
                            ui.label("• Follows system");
                            let system_dark = detect_system_dark_mode();
                            ui.label(format!(
                                "• System is: {}",
                                if system_dark { "Dark" } else { "Light" }
                            ));
                        }
                    }
                });
            });

            ui.add_space(12.0);

            ui.collapsing("⚙️ Layout Options", |ui| {
                ui.checkbox(&mut app.show_sidebar, "Show Sidebar");
            });
        });
}
