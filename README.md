# egui-desktop

A comprehensive desktop UI framework for egui applications with native window decorations, advanced theming, and cross-platform desktop integration.

## ⚠️ Disclaimer

**This project is currently in alpha version and under active development.** It may not be fully tested on all platforms. While we strive for cross-platform compatibility, some features may behave differently or have limitations on certain operating systems. Please test thoroughly in your target environments before using in production applications.

**Known limitations:**

- System theme detection may not work on all Linux distributions
- Native rounded corners support varies by platform and window manager
- Some advanced features may require specific OS versions or configurations

We welcome feedback, bug reports, and contributions to help improve platform compatibility! Feel free to open pull requests or report issues - your input helps make this framework better for everyone.

## 🎯 Goal

As a developer who uses this framework to build my own desktop applications, I'm passionate about rust and egui and want to make desktop development easier for Rust programmers. This framework addresses the common pain points when building native-feeling desktop apps with egui and custom window bars:

- **Native integration**: Seamless window decorations and platform-specific behaviors
- **Developer experience**: Simple APIs that handle complex cross-platform differences
- **Customization**: Flexible theming and styling options for modern applications

Not WebAssembly-oriented, but designed to give egui desktop apps professional look and features (custom title bar, menus, system icons, Windows/macOS/Linux themes).

The goal is to provide a solid foundation so you can focus on building your application logic instead of wrestling with platform-specific UI details.

## ✨ Features

### 🪟 **Title Bars**

- **macOS**: Custom title bar with authentic traffic light buttons (close, minimize, maximize)
- **Windows/Linux**: Generic title bar with standard window controls
  - _Note: Linux currently uses the same title bar style as Windows. Contributions are welcome to add Linux-specific styling!_
- **Auto-detection**: Automatically selects the appropriate title bar for your OS
- **Custom app icons**: Support for SVG, PNG, JPEG and other image formats
- **Custom title bar icons**: Add your own icons with automatic platform positioning
- **Icon keyboard shortcuts**: Bind keyboard shortcuts to custom icons with tooltip display
- **Optional titles**: Hide title text while keeping the icon and controls
- **Menu integration**: Add menu items or icons directly in the title bar
- **Advanced menu system**: Multi-level menus with submenus and cascading sidemenus
- **Keyboard navigation**: Full keyboard support following platform standards
- **Cross-platform shortcuts**: Comprehensive keyboard shortcut system with global state management
- **Native control replacement**: All window control buttons (minimize, maximize, close) are replaced with custom egui-drawn buttons for complete visual control

### 🎨 **Theme System**

- **Light/Dark themes**: Built-in light and dark themes with proper contrast
- **System theme detection**: Automatically follows your OS theme (Windows, macOS, Linux)
- **Custom themes**: Create your own color schemes
- **Theme synchronization**: Sync with egui's theme system
- **Cross-platform detection**: Detects system dark mode on all major platforms

### 🪟 **Window Features**

- **Native rounded corners**: Platform-specific rounded window corners
- **Manual resizing**: Interactive resize handles for custom window resizing
- **Decorative windows**: Disable native decorations for full customization
- **Cross-platform**: Works on Windows, macOS, and Linux

### 🎛️ **Customization**

- **Colors**: Customize background, hover, close button, and title colors
- **Fonts**: Adjustable title font size
- **Icons**: Custom app icons and window control icons
- **Layouts**: Flexible menu layouts and spacing
- **Behaviors**: Customizable button behaviors and interactions

## 🚀 Quick Start

### CLI Tool

The easiest way to get started is using our CLI tool that generates a complete starter project:

#### Installation

```bash
# Install the CLI globally from crates.io
cargo install egui-desktop-cli

# Or install from local development:
# cargo install --path cli
```

#### Usage

```bash
# Generate a new project
egui-desktop my-super-project

# This creates a complete project with:
# - Modular structure (main.rs, app.rs, theme_provider.rs, etc.)
# - All dependencies configured
# - Ready-to-run example with themes, sidebar, and custom UI
```

The CLI generates a professional project structure with:

- ✅ **Modular architecture**: Clean separation of concerns
- ✅ **Complete setup**: All dependencies and imports configured
- ✅ **Working example**: Interactive theme demo with sidebar
- ✅ **Cross-platform**: Works on Windows, macOS, and Linux

For more details about the CLI and generated project structure, see the [CLI README](cli/README.md).

### Basic Usage

```rust
use egui_desktop::{TitleBar, apply_native_rounded_corners_to_window, render_resize_handles};

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Apply native rounded corners (call once)
        apply_native_rounded_corners_to_window(frame);

        // Render title bar with default light theme
        TitleBar::new("My App").show(ctx);

        // Add manual resize handles
        render_resize_handles(ctx);

        // Your app content here
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}
```

### Native Control Replacement

The framework completely replaces native window control buttons with custom egui-drawn buttons:

- **Complete visual control**: All buttons (minimize, maximize, close) are drawn using egui
- **Custom styling**: Full control over button appearance, colors, hover effects, and animations
- **Platform consistency**: Buttons look identical across Windows, macOS, and Linux
- **Theme integration**: Buttons automatically adapt to your custom themes
- **No native dependencies**: No reliance on platform-specific button rendering

```rust
// Custom control button colors
let title_bar = TitleBar::new("My App")
    .with_custom_theme(TitleBarTheme {
        close_hover_color: egui::Color32::RED,           // Red close button on hover
        minimize_icon_color: egui::Color32::BLUE,        // Blue minimize icon
        maximize_icon_color: egui::Color32::GREEN,       // Green maximize icon
        restore_icon_color: egui::Color32::YELLOW,       // Yellow restore icon
        ..Default::default()
    });
```

### Theme Customization

```rust
use egui_desktop::{TitleBar, ThemeMode, TitleBarTheme};

// Light theme
TitleBar::new("My App")
    .with_theme_mode(ThemeMode::Light)
    .show(ctx);

// Dark theme
TitleBar::new("My App")
    .with_theme_mode(ThemeMode::Dark)
    .show(ctx);

// System theme (follows OS)
TitleBar::new("My App")
    .with_theme_mode(ThemeMode::System)
    .sync_with_system_theme()
    .show(ctx);

// Custom theme
TitleBar::new("My App")
    .with_theme(TitleBarTheme {
        background_color: Color32::from_rgb(45, 45, 65),
        hover_color: Color32::from_rgb(65, 65, 85),
        close_hover_color: Color32::from_rgb(220, 20, 40),
        close_icon_color: Color32::from_rgb(180, 180, 180),
        title_color: Color32::from_rgb(200, 200, 255),
    })
    .show(ctx);
```

### Custom Icons

Add custom icons to the title bar with optional keyboard shortcuts:

```rust
use egui_desktop::{TitleBar, CustomIcon, KeyboardShortcut};

TitleBar::new("My App")
    // Custom app icon (supports SVG, PNG, JPEG, etc.)
    .with_custom_app_icon(include_bytes!("icon.svg"), "app-icon.svg")

    // Add custom icon with callback and tooltip
    .add_icon(
        CustomIcon::Image(ImageSource::from_bytes("settings.svg", include_bytes!("settings.svg"))),
        Some(Box::new(|| println!("Settings clicked!"))),
        Some("Settings".to_string()),
        None // No keyboard shortcut
    )

    // Add custom icon with keyboard shortcut
    .add_icon(
        CustomIcon::Drawn(Box::new(|painter, rect, color| {
            // Custom drawing code for notification bell
            painter.circle_filled(rect.center(), rect.width() * 0.4, color);
        })),
        Some(Box::new(|| println!("Notifications clicked!"))),
        Some("Notifications".to_string()),
        Some(KeyboardShortcut::parse("ctrl+n")) // Ctrl+N shortcut
    )

    .show(ctx);

// Don't forget to handle shortcuts in your app's update loop
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.title_bar.handle_icon_shortcuts(ctx);
        // ... rest of your app logic
    }
}
```

#### Icon Types

- **Image Icons**: Use `CustomIcon::Image()` with SVG, PNG, JPEG, etc.
- **Drawn Icons**: Use `CustomIcon::Drawn()` with custom drawing functions

#### Keyboard Shortcuts

Icons can have keyboard shortcuts that trigger their callbacks:

- Shortcuts are displayed in tooltips: "Settings (Ctrl+,)"
- Use `KeyboardShortcut::parse()` for simple string-based shortcuts
- Call `handle_icon_shortcuts(ctx)` in your app's update loop

### Multi-Window Applications

The framework supports independent title bars for each window in multi-window applications:

```rust
use egui_desktop::{TitleBar, TitleBarOptions};

struct MultiWindowApp {
    main_window: TitleBar,
    settings_window: TitleBar,
    about_window: TitleBar,
}

impl MultiWindowApp {
    fn new() -> Self {
        Self {
            // Each window has its own independent TitleBar instance
            main_window: TitleBar::new(TitleBarOptions::new()
                .with_title("Main Window")
                .with_show_close_button(true)
                .with_show_maximize_button(true)
                .with_show_minimize_button(true)),

            settings_window: TitleBar::new(TitleBarOptions::new()
                .with_title("Settings")
                .with_show_close_button(true)
                .with_show_maximize_button(false)  // Settings window can't be maximized
                .with_show_minimize_button(false)), // Settings window can't be minimized

            about_window: TitleBar::new(TitleBarOptions::new()
                .with_title("About")
                .with_show_close_button(true)
                .with_show_maximize_button(false)
                .with_show_minimize_button(false)),
        }
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Each window renders its title bar independently
        self.main_window.show(ctx, frame);
        self.settings_window.show(ctx, frame);
        self.about_window.show(ctx, frame);

        // Each window can have different themes
        if self.settings_window.is_open() {
            self.settings_window.set_theme_mode(ThemeMode::Dark);
        }
    }
}
```

**Benefits of independent TitleBar instances:**

- **Isolated state**: Each window maintains its own menu state, keyboard navigation, and theme
- **Custom behavior**: Different windows can have different control buttons and behaviors
- **Independent themes**: Each window can use a different theme or color scheme
- **Separate menus**: Each window can have completely different menu structures
- **Memory efficient**: Only active windows consume resources

### Custom Title Bar Icons

```rust
use egui_desktop::{CustomIcon, ImageSource};

// Add custom icons to the title bar (automatically positioned by platform)
TitleBar::new("My App")
    // Image-based icon (SVG, PNG, JPEG, etc.)
    .add_icon(
        CustomIcon::Image(ImageSource::from_bytes("settings.svg", include_bytes!("settings.svg"))),
        Some(Box::new(|| println!("Settings clicked!")))
    )
    // Custom drawn icon
    .add_icon(
        CustomIcon::Drawn(Box::new(|painter, rect, color| {
            // Draw a custom notification bell
            let center = rect.center();
            let radius = rect.width().min(rect.height()) * 0.4;
            painter.circle_stroke(center, radius, egui::Stroke::new(2.0, color));
        })),
        Some(Box::new(|| println!("Notifications!")))
    )
    .show(ctx);
```

### Animated Title Bar Icons

You can add animated icons that the framework will drive every frame with timing, hover/press state, and theme colors.

API overview:

- `CustomIcon::Animated(Box<dyn Fn(&Painter, Rect, Color32, &mut IconAnimationState, AnimationCtx) + Send + Sync>)`
- `CustomIcon::AnimatedUi(Box<dyn Fn(&mut Ui, Rect, Color32, &mut IconAnimationState, AnimationCtx) + Send + Sync>)`
- `TitleBar::add_animated_icon(...)` and `TitleBar::add_animated_ui_icon(...)`
- `IconAnimationState { hover_t, press_t, progress, last_time }`
- `AnimationCtx { time, delta_seconds, hovered, pressed }`

Notes:

- Make your `TitleBar` persistent (store it in your app struct) so per-icon animation state is preserved across frames.
- Clicks automatically request a repaint, so animations start immediately.
- Theme colors are passed as `icon_color`; hover backgrounds use the title bar theme. Theme changes update these automatically.
- You can override an icon color with `set_custom_icon_color(index, Some(color))`; pass `None` to return to theme-driven color.

Painter-based example (minimal):

```rust
use egui_desktop::{TitleBar, TitleBarOptions};

// Build once and store in your app struct
let mut title_bar = TitleBar::new(TitleBarOptions::new().with_title("Animated"));

title_bar = title_bar.add_animated_icon(
    Box::new(|painter, rect, icon_color, state, actx| {
        // Simple pulse driven by hover
        let target = if actx.hovered { 1.0 } else { 0.0 };
        state.progress += (target - state.progress) * (actx.delta_seconds * 6.0);
        let r = rect.width().min(rect.height()) * (0.25 + 0.15 * state.progress);
        painter.circle_filled(rect.center(), r, icon_color);
    }),
    Some(Box::new(|| println!("Animated icon clicked"))),
    Some("Animated".to_string()),
    None,
);
```

Ui-based example (no Painter required) – sun→moon style:

```rust
title_bar = title_bar.add_animated_ui_icon(
    Box::new(|ui, rect, icon_color, state, actx| {
        let speed = 6.0;
        let target = if actx.hovered { 1.0 } else { 0.0 };
        state.progress += (target - state.progress) * (actx.delta_seconds * speed);
        let mut child = ui.child_ui(rect, egui::Layout::default(), None);
        let center = rect.center();
        let size = rect.height().min(rect.width());
        let radius = size * 0.35;
        if state.progress < 0.5 {
            let sun_p = 1.0 - (state.progress * 2.0);
            child.painter().circle(center, radius * 0.8 * sun_p, icon_color, egui::Stroke::NONE);
        } else {
            let moon_p = (state.progress - 0.5) * 2.0;
            child.painter().circle(center, radius, icon_color, egui::Stroke::NONE);
            let offset = radius * 0.6 * moon_p;
            let angle = std::f32::consts::FRAC_PI_4;
            let mask_center = egui::Pos2::new(center.x + angle.cos() * offset, center.y - angle.sin() * offset);
            child.painter().circle(mask_center, radius, ui.visuals().widgets.noninteractive.bg_fill, egui::Stroke::NONE);
        }
    }),
    None,
    Some("Theme".to_string()),
    None,
);
```

Theme-aware coloring:

```rust
title_bar.set_custom_icon_color(0, None); // use theme color
// Or override:
title_bar.set_custom_icon_color(0, Some(egui::Color32::from_rgb(255, 200, 0)));
```

**Platform-specific positioning:**

- **Windows/Linux**: Icons appear to the left of window control buttons
- **macOS**: Icons appear to the right of traffic light buttons

### Menu Integration

```rust
TitleBar::new("My App")
    .add_menu_item("File", Some(Box::new(|| println!("File clicked"))))
    .add_menu_item("Edit", Some(Box::new(|| println!("Edit clicked"))))
    .add_menu_item("Help", Some(Box::new(|| println!("Help clicked"))))
    .show(ctx);
```

### Advanced Menu System with Submenus

```rust
use egui_desktop::{TitleBar, MenuItem, SubMenuItem, KeyboardShortcut};

// Create complex menu structures with submenus and shortcuts
let file_menu = MenuItem::new("File")
    .add_subitem(
        SubMenuItem::new("New")
            .with_shortcut(KeyboardShortcut::parse("ctrl+n"))
            .with_callback(Box::new(|| println!("New file!")))
    )
    .add_subitem(
        SubMenuItem::new("Open")
            .with_shortcut(KeyboardShortcut::parse("ctrl+o"))
            .with_callback(Box::new(|| println!("Open file!")))
    )
    .add_subitem(
        SubMenuItem::new("Save")
            .with_shortcut(KeyboardShortcut::parse("ctrl+s"))
            .with_callback(Box::new(|| println!("Save file!")))
            .with_separator() // Add separator after this item
    )
    .add_subitem(
        SubMenuItem::new("Exit")
            .with_shortcut(KeyboardShortcut::parse("ctrl+q"))
            .with_callback(Box::new(|| std::process::exit(0)))
    );

// Add submenus with nested sidemenus (cascading menus)
let edit_menu = MenuItem::new("Edit")
    .add_subitem(
        SubMenuItem::new("Undo")
            .with_shortcut(KeyboardShortcut::parse("ctrl+z"))
            .with_callback(Box::new(|| println!("Undo!")))
    )
    .add_subitem(
        SubMenuItem::new("Cut")
            .with_shortcut(KeyboardShortcut::parse("ctrl+x"))
            .with_callback(Box::new(|| println!("Cut!")))
    )
    .add_subitem(
        SubMenuItem::new("Copy")
            .with_shortcut(KeyboardShortcut::parse("ctrl+c"))
            .with_callback(Box::new(|| println!("Copy!")))
    )
    .add_subitem(
        SubMenuItem::new("Paste")
            .with_shortcut(KeyboardShortcut::parse("ctrl+v"))
            .with_callback(Box::new(|| println!("Paste!")))
            .with_separator()
    )
    .add_subitem(
        SubMenuItem::new("Find")
            .with_shortcut(KeyboardShortcut::parse("ctrl+f"))
            .with_callback(Box::new(|| println!("Find!")))
            .add_child_subitem(
                SubMenuItem::new("Find Next")
                    .with_shortcut(KeyboardShortcut::parse("f3"))
                    .with_callback(Box::new(|| println!("Find next!")))
            )
            .add_child_subitem(
                SubMenuItem::new("Find Previous")
                    .with_shortcut(KeyboardShortcut::parse("shift+f3"))
                    .with_callback(Box::new(|| println!("Find previous!")))
            )
    );

TitleBar::new("My App")
    .add_menu_with_submenu(file_menu)
    .add_menu_with_submenu(edit_menu)
    .show(ctx);
```

### Keyboard Navigation System

The framework provides comprehensive keyboard navigation that follows platform standards:

#### Activation

- **Alt** (Windows/Linux standard)
- **Ctrl+F2** (macOS standard)

#### Navigation

- **Arrow Keys**: Navigate through menu items
  - **Left/Right**: Navigate between top-level menus
  - **Up/Down**: Navigate within submenus
  - **Right**: Open sidemenus (when available)
  - **Left**: Close sidemenus or go back

#### Selection

- **Enter**: Select menu item or trigger action
- **Space**: Alternative selection (Qt/macOS/Linux standard)

#### Menu Management

- **Escape**: Close all menus and deactivate keyboard navigation
- **Mouse click outside**: Close menus but keep keyboard navigation active

#### Smart Navigation Logic

The navigation system intelligently handles different contexts:

1. **Top-level menus**: Left/right navigation between menu categories
2. **Submenus**: Up/down navigation within menu items
3. **Sidemenus**: Up/down navigation within cascading menu items
4. **Context-aware**: Navigation is disabled only when on highlighted items with sidemenus

#### Cross-Platform Compatibility

The keyboard navigation follows platform conventions:

- **Windows**: Alt activation, Enter selection
- **macOS**: Ctrl+F2 activation, Space/Enter selection
- **Linux**: Alt activation, Space/Enter selection

### Keyboard Shortcuts

The framework supports simple, string-based keyboard shortcuts:

```rust
use egui_desktop::KeyboardShortcut;

// Simple shortcuts with string parsing
KeyboardShortcut::parse("s")                  // Single key
KeyboardShortcut::parse("ctrl+s")             // Ctrl+S
KeyboardShortcut::parse("alt+s")              // Alt+S
KeyboardShortcut::parse("shift+s")            // Shift+S
KeyboardShortcut::parse("cmd+s")              // Cmd+S (macOS)

// Complex combinations
KeyboardShortcut::parse("f3")                 // F3
KeyboardShortcut::parse("shift+f3")           // Shift+F3
KeyboardShortcut::parse("ctrl+shift+f3")      // Ctrl+Shift+F3

// Special keys
KeyboardShortcut::parse("enter")              // Enter
KeyboardShortcut::parse("space")              // Space
KeyboardShortcut::parse("escape")             // Escape
KeyboardShortcut::parse("tab")                // Tab
KeyboardShortcut::parse("delete")             // Delete

// Numbers and punctuation
KeyboardShortcut::parse("1")                  // Number 1
KeyboardShortcut::parse("ctrl+=")             // Ctrl+=
KeyboardShortcut::parse("ctrl+-")             // Ctrl+-
```

**Supported modifiers:** `ctrl`, `alt`, `shift`, `cmd` (macOS)
**Supported keys:** All letters (a-z), numbers (0-9), function keys (f1-f12), special keys, and punctuation.

### Menu Rendering and Interaction

#### Visual States

- **Normal**: Default appearance
- **Hovered**: Mouse hover highlight
- **Keyboard Selected**: Distinct blue highlight for keyboard navigation
- **Disabled**: Grayed out appearance

#### Interaction Modes

1. **Mouse Mode**: Hover to open submenus, click to select
2. **Keyboard Mode**: Arrow keys for navigation, Enter/Space to select
3. **Mixed Mode**: Both mouse and keyboard work simultaneously

#### Menu Positioning

- **Automatic positioning**: Menus position themselves to stay on screen
- **Cascading sidemenus**: Automatically align with parent menu items
- **Platform-aware**: Follows OS conventions for menu placement

### Best Practices for Menu Design

1. **Use consistent shortcuts**: Follow platform conventions (Ctrl+Z for undo, etc.)
2. **Group related items**: Use separators to group logical menu sections
3. **Provide keyboard alternatives**: Every mouse action should have a keyboard equivalent
4. **Test navigation flow**: Ensure smooth navigation through all menu levels
5. **Follow platform guidelines**: Use appropriate activation keys for each platform

### Platform-Specific Title Visibility

```rust
use egui_desktop::{TitleBar, TitleBarOptions};

// Control title visibility per platform
TitleBar::new(
    TitleBarOptions::new()
        .with_title("My App")
        .with_title_visibility(
            false, // macOS: hide title (follows macOS conventions)
            true,  // Windows: show title
            true,  // Linux: show title
        )
)
.show(ctx);

// Or use the new unified API
TitleBar::new(
    TitleBarOptions::new()
        .with_title("My App")
        .with_title_visibility(true, true, false)
)
.show(ctx);
```

### Advanced Customization

```rust
TitleBar::new(
    TitleBarOptions::new()
        .with_title("My App")
        .with_background_color(Color32::from_rgb(30, 30, 30))
        .with_hover_color(Color32::from_rgb(60, 60, 60))
        .with_close_hover_color(Color32::from_rgb(232, 17, 35))
        .with_title_color(Color32::from_rgb(200, 200, 200))
        .with_title_font_size(14.0)
        .with_title_visibility(true, true, false) // Platform-specific title visibility
        .with_custom_app_icon(include_bytes!("icon.svg"), "app-icon.svg")
)
.show(ctx);
```

### Customizing Keyboard Selection Colors

You can customize the highlight color for keyboard navigation:

```rust
TitleBar::new(
    TitleBarOptions::new()
        .with_title("My App")
        .with_keyboard_selection_color(Color32::from_rgba_premultiplied(255, 100, 100, 120)) // Custom red highlight
)
.show(ctx);
```

**Default colors:**

- **Light theme**: Windows blue (`rgb(0, 120, 215)`) - Universal blue that works everywhere
- **Dark theme**: Dodger blue (`rgb(30, 144, 255)`) - Brighter blue for dark backgrounds

## 📋 Examples

### CLI Generated Project

The easiest way to see all features in action is to generate a complete project:

```bash
# Install CLI from crates.io (if not already installed)
cargo install egui-desktop-cli

# Generate and run demo project
egui-desktop mon-demo-projet
cd mon-demo-projet
cargo run
```

This creates a fully-featured demo with:

- Interactive theme switching (Light/Dark/System/Custom)
- Sidebar with theme controls
- All framework features demonstrated
- Professional project structure

### Built-in Examples

| Example               | Description                                                                               |
| --------------------- | ----------------------------------------------------------------------------------------- |
| `basic_app.rs`        | Simple app with default light theme and title bar                                         |
| `custom_title_bar.rs` | Customized title bar with dark theme and menu items                                       |
| `multi_platform.rs`   | Cross-platform demo showing OS-specific features                                          |
| `no_title_app.rs`     | Title bar without title text (macOS: traffic lights only, Windows/Linux: icon + controls) |

### Testing Keyboard Navigation

To test the keyboard navigation features:

1. **Run any example**: `cargo run --example theme_demo`
2. **Activate keyboard mode**: Press `Alt` (Windows/Linux) or `Ctrl+F2` (macOS)
3. **Navigate menus**: Use arrow keys to navigate through menu items
4. **Open submenus**: Press `Right` arrow on items with submenus
5. **Select items**: Press `Enter` or `Space` to activate menu items
6. **Close menus**: Press `Escape` or click outside the menu area

### Keyboard Navigation Features Demonstrated

- **Cross-platform activation**: Alt vs Ctrl+F2 based on OS
- **Multi-level navigation**: Top-level menus → submenus → sidemenus
- **Smart context handling**: Navigation disabled only when appropriate
- **Platform-standard shortcuts**: Enter/Space for selection
- **Visual feedback**: Distinct highlighting for keyboard vs mouse interaction

Run examples with:

```bash
cargo run --example basic_app
cargo run --example theme_demo
cargo run --example custom_title_bar
```

## 🎨 Theme System Details

### Built-in Themes

- **Light Theme**: White background, dark text, light gray hover
- **Dark Theme**: Dark background, light text, dark gray hover
- **System Theme**: Automatically detects and follows OS theme

### System Detection

The framework automatically detects system themes on:

- **Windows**: Registry key `AppsUseLightTheme`
- **macOS**: `defaults read -g AppleInterfaceStyle`
- **Linux**: `gsettings` (GNOME) or `GTK_THEME` environment variable

### Theme Methods

- `with_theme_mode(mode)` - Set theme mode (Light/Dark/System)
- `with_theme(theme)` - Use custom theme
- `sync_with_egui_theme(ctx)` - Sync with egui's theme
- `sync_with_system_theme()` - Sync with system theme

### Platform-Specific Title Visibility

Control whether the app title is displayed on each platform:

- `with_title_visibility(macos, windows, linux)` - Set visibility per platform
- **Default**: macOS = true, Windows = true, Linux = true
- Useful for following platform conventions or custom requirements

## 🪟 Window Features

### Native Rounded Corners

- **Windows**: Uses `DwmSetWindowAttribute` API
- **macOS**: Uses `NSWindow.cornerRadius`
- **Linux**: Detects X11/Wayland and applies appropriate shapes

### Manual Resizing

Interactive resize handles around window edges and corners:

- Top, bottom, left, right edges
- All four corners
- Proper cursor icons for each handle
- Sends `ViewportCommand::BeginResize` to egui

## 📦 Dependencies

The framework uses minimal dependencies:

- `egui` - UI framework
- `egui_extras` - Image loaders and utilities
- `raw-window-handle` - Window handle abstraction
- `lazy_static` - Global state management for keyboard shortcuts

Platform-specific dependencies are included automatically:

- **Windows**: `windows` crate for native APIs
- **macOS**: `cocoa` and `objc` for native APIs
- **Linux**: `x11`, `wayland-client` for native APIs

## 🔧 Technical Implementation

### Global State Management with `lazy_static`

The framework uses `lazy_static` to manage global state for keyboard shortcuts, ensuring proper "just pressed" detection across the entire application. This is essential for reliable keyboard shortcut handling.

**Why `lazy_static` is needed:**

- **Frame-based detection**: egui's `key_pressed()` method returns `true` for the entire duration a key is held, not just on the first press
- **Global state tracking**: We need to track which shortcuts were pressed in the previous frame to detect the transition from "not pressed" to "pressed"
- **Cross-component coordination**: Multiple components (menus, custom icons) can use the same shortcuts

**Implementation details:**

```rust
lazy_static! {
    static ref SHORTCUT_STATES: Mutex<HashMap<String, bool>> = Mutex::new(HashMap::new());
}
```

This global state allows the framework to:

- Track the previous state of each unique shortcut combination
- Detect true "just pressed" events (transition from false to true)
- Coordinate shortcut handling across different UI components
- Provide consistent keyboard behavior throughout the application

**Benefits:**

- ✅ Reliable shortcut detection
- ✅ No duplicate shortcut triggers
- ✅ Consistent behavior across all components
- ✅ Minimal performance overhead

## 🛠️ Setup

1. Add to your `Cargo.toml`:

```toml
[dependencies]
egui-desktop = "0.1.0"
egui_extras = { version = "0.32", features = ["all_loaders"] }
eframe = "0.32"
egui = "0.32"
```

2. Initialize in your app:

```rust
use egui_extras::install_image_loaders;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false), // Disable native decorations
        ..Default::default()
    };

    eframe::run_native(
        "My App",
        options,
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx); // Enable image loading
            Ok(Box::new(MyApp::default()))
        }),
    )
}
```

## 🎯 Best Practices

### General Setup

1. **Always disable native decorations** when using custom title bars
2. **Call `apply_native_rounded_corners_to_window` once** in your update loop
3. **Install image loaders** for custom icons to work
4. **Use `ThemeMode::System`** for the best user experience
5. **Test on multiple platforms** to ensure cross-platform compatibility

### Menu and Keyboard Navigation

6. **Follow platform conventions**: Use appropriate activation keys (Alt for Windows/Linux, Ctrl+F2 for macOS)
7. **Provide consistent shortcuts**: Use standard shortcuts like Ctrl+Z for undo, Ctrl+S for save
8. **Test navigation flow**: Ensure smooth keyboard navigation through all menu levels
9. **Group related items**: Use separators to organize menu items logically
10. **Handle mixed input**: Design for both mouse and keyboard users
11. **Provide visual feedback**: Ensure keyboard-selected items are clearly highlighted
12. **Test edge cases**: Verify behavior when switching between mouse and keyboard input
13. **Customize selection colors**: Choose keyboard highlight colors that work well with your app's theme
14. **Consider accessibility**: Ensure sufficient contrast between selection colors and background

## 🤝 Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.

**Particularly needed:**

- Linux-specific title bar styling and window decorations
- Additional Linux desktop environment support (KDE, XFCE, etc.)
- Platform-specific theme detection improvements
- Cross-platform testing and bug fixes

We'd love to see Linux users contribute their expertise to make this framework work great on all Linux distributions and desktop environments!

## 📄 License

MIT License - see LICENSE file for details.
