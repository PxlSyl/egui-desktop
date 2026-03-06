use crate::app::CustomThemeDemoApp;
use egui::{ScrollArea, Ui, Slider, ProgressBar, Color32};

pub fn render_main_content(app: &mut CustomThemeDemoApp, ui: &mut Ui) {
    ScrollArea::vertical()
        .max_width(ui.available_width())
        .show(ui, |ui| {
            ui.colored_label(app.get_text_color(ui), "🎨 Egui Desktop Starter");
            ui.separator();

            ui.label("This demo demonstrates the complete theme customization system with override capabilities.");
            ui.add_space(16.0);

            ui.collapsing("💻 Code Examples", |ui| {
                ui.vertical(|ui| {
                    ui.label("1. Default themes:");
                    ui.code("TitleBar::new(\"My App\").with_theme_mode(ThemeMode::Light)");
                    ui.code("TitleBar::new(\"My App\").with_theme_mode(ThemeMode::Dark)");
                    ui.code("TitleBar::new(\"My App\").with_theme_mode(ThemeMode::Auto)");
                    ui.add_space(8.0);

                    ui.label("1.5. Simple custom colors:");
                    ui.code("TitleBar::new(\"My App\")
    .with_background_color(Color32::from_rgb(45, 45, 65))
    .with_title_color(Color32::from_rgb(200, 200, 255))
    .with_menu_text_color(Color32::from_rgb(200, 200, 255))");
                    ui.add_space(8.0);

                    ui.label("2. Custom light theme with overrides:");
                    ui.code("TitleBar::new(\"My App\").with_custom_light_theme((
    Some(Color32::from_rgb(240, 248, 255)), // background_color
    Some(Color32::from_rgb(220, 235, 255)), // hover_color
    Some(Color32::from_rgb(220, 38, 38)), // close_hover_color
    Some(Color32::from_rgb(59, 130, 246)), // close_icon_color
    None, // maximize_icon_color
    None, // restore_icon_color
    None, // minimize_icon_color
    None, // title_color
    None, // menu_text_color
    Some(14.0), // menu_text_size
    Some(Color32::from_rgb(219, 234, 254)), // menu_hover_color
    None, // submenu_background_color
    None, // submenu_text_color
    None, // submenu_hover_color
    None, // submenu_shortcut_color
    None, // keyboard_selection_color
    None, // submenu_keyboard_selection_color
))");
                    ui.add_space(8.0);

                    ui.label("3. Custom dark theme with overrides:");
                    ui.code("TitleBar::new(\"My App\").with_custom_dark_theme((
    Some(Color32::from_rgb(30, 30, 46)), // background_color
    Some(Color32::from_rgb(50, 50, 70)), // hover_color
    Some(Color32::from_rgb(239, 68, 68)), // close_hover_color
    Some(Color32::from_rgb(147, 197, 253)), // close_icon_color
    None, // maximize_icon_color
    None, // restore_icon_color
    None, // minimize_icon_color
    None, // title_color
    None, // menu_text_color
    Some(14.0), // menu_text_size
    Some(Color32::from_rgb(60, 60, 80)), // menu_hover_color
    None, // submenu_background_color
    None, // submenu_text_color
    None, // submenu_hover_color
    None, // submenu_shortcut_color
    None, // keyboard_selection_color
    None, // submenu_keyboard_selection_color
))");
                });
            });

            ui.add_space(16.0);

            ui.collapsing("✨ Features Overview", |ui| {
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.label("🎯 Menu Customization:");
                        ui.label("• Menu text color: Fully customizable");
                        ui.label("• Menu text size: Adjustable (default: 12.0)");
                        ui.label("• Menu hover color: Customizable per theme");
                        ui.label("• Native-style menu rendering");
                        ui.label("• All values sync with light/dark themes");
                    });

                    ui.add_space(12.0);

                    ui.group(|ui| {
                        ui.label("🎨 Icon Customization:");
                        ui.label("• Close icon color: Customizable");
                        ui.label("• Maximize icon color: Customizable");
                        ui.label("• Restore icon color: Customizable");
                        ui.label("• Minimize icon color: Customizable");
                        ui.label("• All icons adapt to light/dark themes");
                    });

                    ui.add_space(12.0);

                    ui.group(|ui| {
                        ui.label("🌓 Theme System:");
                        ui.label("• Default light/dark themes");
                        ui.label("• Custom theme overrides");
                        ui.label("• System theme detection");
                        ui.label("• Individual color customization");
                    });

                    ui.add_space(12.0);

                    ui.group(|ui| {
                        ui.label("⌨️ Keyboard Navigation:");
                        ui.label("• Alt + Arrow Keys for menu navigation");
                        ui.label("• Enter to activate menu items");
                        ui.label("• Escape to close menus");
                        ui.label("• Full keyboard accessibility support");
                    });

                    ui.add_space(12.0);

                    ui.group(|ui| {
                        ui.label("🪟 Window Features:");
                        ui.label("• Native rounded corners");
                        ui.label("• Manual resize handles");
                        ui.label("• Cross-platform compatibility");
                        ui.label("• Custom window decorations");
                    });
                });
            });

            ui.add_space(16.0);

            ui.collapsing("⚡ Live Demo", |ui| {
                ui.vertical(|ui| {
                    ui.label("Instructions:");
                    ui.label("1. Use the sidebar to switch between different themes");
                    ui.label("2. Notice how the title bar changes immediately");
                    ui.label("3. Compare menu text sizes between default and custom themes");
                    ui.label("4. Observe icon color changes in custom themes");
                    ui.label("5. Try the Auto theme to follow your system settings");
                    ui.label("6. Try keyboard navigation: Alt + Arrow Keys");
                    ui.add_space(8.0);
                    ui.label("💡 Tip: The custom themes use larger menu text (14px vs 12px default), larger title text (16px vs 12px default), and custom icon colors!");
                });
            });

            ui.add_space(16.0);

            ui.collapsing("🎮 Keyboard Navigation Demo", |ui| {
                ui.vertical(|ui| {
                    ui.label("Try these keyboard shortcuts:");
                    ui.add_space(8.0);
                    
                    ui.group(|ui| {
                        ui.label("🚀 Getting Started:");
                        ui.label("1. Press Alt to activate keyboard navigation");
                        ui.label("2. Use Left/Right arrows to navigate between menus");
                        ui.label("3. Use Up/Down arrows to navigate within menus");
                        ui.label("4. Press Enter to activate a menu item");
                        ui.label("5. Press Escape to close menus");
                    });
                    
                    ui.add_space(8.0);
                    
                    ui.group(|ui| {
                        ui.label("🎯 Menu Navigation:");
                        ui.label("• Alt + Right Arrow: Next menu");
                        ui.label("• Alt + Left Arrow: Previous menu");
                        ui.label("• Alt + Down Arrow: Open current menu");
                        ui.label("• Down Arrow: Next menu item");
                        ui.label("• Up Arrow: Previous menu item");
                        ui.label("• Right Arrow: Open submenu");
                        ui.label("• Left Arrow: Close submenu");
                    });
                    
                    ui.add_space(8.0);
                    
                    ui.group(|ui| {
                        ui.label("⌨️ Quick Actions:");
                        ui.label("• Ctrl+B: Toggle sidebar");
                        ui.label("• Ctrl+N: New file");
                        ui.label("• Ctrl+O: Open file");
                        ui.label("• Ctrl+S: Save file");
                        ui.label("• F3: Find next");
                    });
                });
            });

            ui.add_space(16.0);

            ui.collapsing("⌨️ Keyboard Shortcuts", |ui| {
                ui.vertical(|ui| {
                    ui.label("Available keyboard shortcuts:");
                    ui.add_space(8.0);
                    
                    ui.group(|ui| {
                        ui.label("🎛️ Control Shortcuts:");
                        ui.label("• Ctrl+B - Toggle Sidebar");
                        ui.label("• Alt + Arrow Keys - Navigate menus");
                        ui.label("• Enter - Activate menu item");
                        ui.label("• Escape - Close menu");
                    });
                    
                    ui.add_space(8.0);
                    
                    ui.group(|ui| {
                        ui.label("📁 File Menu:");
                        ui.label("• Ctrl+N - New file");
                        ui.label("• Ctrl+O - Open file");
                        ui.label("• Ctrl+S - Save file");
                        ui.label("• Ctrl+Q - Exit app");
                    });
                    
                    ui.add_space(8.0);
                    
                    ui.group(|ui| {
                        ui.label("✏️ Edit Menu:");
                        ui.label("• Ctrl+Z - Undo");
                        ui.label("• Ctrl+Y - Redo");
                        ui.label("• Ctrl+X - Cut");
                        ui.label("• Ctrl+C - Copy");
                        ui.label("• Ctrl+V - Paste");
                        ui.label("• F3 - Find Next");
                        ui.label("• Shift+F3 - Find Previous");
                    });
                    
                    ui.add_space(8.0);
                    
                    ui.group(|ui| {
                        ui.label("👁️ View Menu:");
                        ui.label("• Ctrl+= - Zoom In");
                        ui.label("• - - Zoom Out");
                        ui.label("• 0 - Reset Zoom");
                    });
                });
            });

            ui.add_space(16.0);

            ui.collapsing("🎨 Theme Preview", |ui| {
                ui.vertical(|ui| {
                    ui.label("Interactive elements to see theme effects:");
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        if ui.button("Demo Button 1").clicked() {
                            println!("Button 1 clicked!");
                        }
                        if ui.button("Demo Button 2").clicked() {
                            println!("Button 2 clicked!");
                        }
                        if ui.button("Demo Button 3").clicked() {
                            println!("Button 3 clicked!");
                        }
                    });

                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.label("Input field:");
                        let mut text = String::new();
                        ui.text_edit_singleline(&mut text);
                    });

                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        let mut checked = false;
                        ui.checkbox(&mut checked, "Demo checkbox");
                        
                        ui.add_space(16.0);
                        
                        ui.label("Slider:");
                        let mut value = 0.5;
                        ui.add(Slider::new(&mut value, 0.0..=1.0));
                    });

                    ui.add_space(8.0);

                    ui.label("Progress bar:");
                    let progress = 0.7;
                    ui.add(ProgressBar::new(progress).text("70% complete"));

                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.label("Color picker:");
                        let mut color = Color32::from_rgb(100, 150, 200);
                        ui.color_edit_button_srgba(&mut color);
                    });
                });
            });
        });
}
