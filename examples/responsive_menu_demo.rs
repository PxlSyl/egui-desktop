use egui::{CentralPanel, Context, Vec2, ViewportBuilder, Visuals};
use egui_desktop::{
    MenuItem, SubMenuItem, TitleBar, TitleBarOptions, apply_rounded_corners, render_resize_handles,
    titlebar::HamburgerStyle,
};
use egui_extras::install_image_loaders;

fn main() -> Result<(), eframe::Error> {
    // Initialize application
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([200.0, 200.0])
            .with_decorations(false), // Disable native decorations to show custom title
        ..Default::default()
    };

    eframe::run_native(
        "Responsive Demo",
        options,
        Box::new(|cc| {
            // Setup egui
            cc.egui_ctx.set_visuals(Visuals::light());

            // Install image loaders to support SVG icons
            install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(ResponsiveMenuApp::default()))
        }),
    )
}

struct ResponsiveMenuApp {
    title_bar: TitleBar,
    previous_window_size: Vec2,
}

impl Default for ResponsiveMenuApp {
    fn default() -> Self {
        // Create menus using the correct TitleBar API
        let file_menu = MenuItem::new("File")
            .add_subitem(SubMenuItem::new("New").with_callback(Box::new(|| println!("New file"))))
            .add_subitem(SubMenuItem::new("Open").with_callback(Box::new(|| println!("Open file"))))
            .add_subitem(
                SubMenuItem::new("Recent Files")
                    .add_child(
                        SubMenuItem::new("Document1.txt")
                            .with_callback(Box::new(|| println!("Open Document1.txt"))),
                    )
                    .add_child(
                        SubMenuItem::new("Presentation.pptx")
                            .with_callback(Box::new(|| println!("Open Presentation.pptx"))),
                    )
                    .add_child(
                        SubMenuItem::new("Spreadsheet.xlsx")
                            .with_callback(Box::new(|| println!("Open Spreadsheet.xlsx"))),
                    ),
            )
            .add_subitem(SubMenuItem::new("Save").with_callback(Box::new(|| println!("Save file"))))
            .add_subitem(
                SubMenuItem::new("Export")
                    .add_child(
                        SubMenuItem::new("PDF")
                            .with_callback(Box::new(|| println!("Export to PDF"))),
                    )
                    .add_child(
                        SubMenuItem::new("Word")
                            .with_callback(Box::new(|| println!("Export to Word"))),
                    )
                    .add_child(
                        SubMenuItem::new("Excel")
                            .with_callback(Box::new(|| println!("Export to Excel"))),
                    ),
            )
            .add_subitem(SubMenuItem::new("Exit").with_callback(Box::new(|| println!("Exit"))));

        let edit_menu = MenuItem::new("Edit")
            .add_subitem(
                SubMenuItem::new("Find")
                    .add_child(
                        SubMenuItem::new("Find...").with_callback(Box::new(|| println!("Find..."))),
                    )
                    .add_child(
                        SubMenuItem::new("Replace...")
                            .with_callback(Box::new(|| println!("Replace..."))),
                    )
                    .add_child(
                        SubMenuItem::new("Find in Files")
                            .with_callback(Box::new(|| println!("Find in Files"))),
                    ),
            )
            .add_subitem(
                SubMenuItem::new("Transform")
                    .add_child(
                        SubMenuItem::new("Rotate").with_callback(Box::new(|| println!("Rotate"))),
                    )
                    .add_child(
                        SubMenuItem::new("Scale").with_callback(Box::new(|| println!("Scale"))),
                    )
                    .add_child(
                        SubMenuItem::new("Skew").with_callback(Box::new(|| println!("Skew"))),
                    ),
            );

        let title_bar = TitleBar::new(
            TitleBarOptions::new().with_hamburger_style(HamburgerStyle::Animated), // No title
        )
        .add_menu_with_submenu(file_menu) // File
        .add_menu_with_submenu(edit_menu) // Edit
        .add_menu_item("View", Some(Box::new(|| println!("View clicked")))) // View
        .add_menu_item("Tools", Some(Box::new(|| println!("Tools clicked")))) // Tools
        .add_menu_item("Help", Some(Box::new(|| println!("Help clicked")))); // Help

        Self {
            title_bar,
            previous_window_size: Vec2::new(800.0, 600.0), // Default size
        }
    }
}

impl eframe::App for ResponsiveMenuApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        // Check for window resize and close all menus
        let current_size = ctx.input(|i| i.content_rect().size());
        if current_size != self.previous_window_size {
            // Window was resized, close all menus
            self.title_bar.close_all_menus();
            self.previous_window_size = current_size;
        }

        apply_rounded_corners(frame);

        // Render resize handles for manual window resizing
        render_resize_handles(ctx);

        // Render the title bar
        self.title_bar.show(ctx, frame);

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Responsive Menu Demo");
            ui.label("Resize window to see responsive behavior:");
            ui.label("• Wide: All menus • Medium: First menus + dot • Narrow: Hamburger only");
            ui.label("(Menus shown in order, left to right)");
            ui.label("");
            ui.label("Keyboard: Alt/Ctrl+F2 to activate, arrows to navigate, Enter to select");
            ui.label("");
            ui.label("Hamburger menu: Select item → overlay closes, dots stay highlighted");
            ui.label("Press Enter again on dots to quickly reopen");

            if ui.button("Test Action").clicked() {
                println!("Test clicked");
            }
        });
    }
}
