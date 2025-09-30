use egui::{Color32, Id, ImageSource, Painter};

use crate::TitleBarOptions;
use crate::menu::items::MenuItem;
use crate::theme::{ThemeMode, ThemeProvider, TitleBarTheme, detect_system_dark_mode};

/// Custom icon for the title bar
pub enum CustomIcon {
    /// SVG/PNG/JPEG image icon
    Image(ImageSource<'static>),
    /// Custom drawing function
    Drawn(Box<dyn Fn(&Painter, egui::Rect, Color32) + Send + Sync>),
    /// Animated icon with framework-managed animation state and context
    Animated(
        Box<
            dyn Fn(&Painter, egui::Rect, Color32, &mut IconAnimationState, AnimationCtx)
                + Send
                + Sync,
        >,
    ),
    /// Animated icon that renders using Ui primitives instead of Painter
    AnimatedUi(
        Box<
            dyn Fn(&mut egui::Ui, egui::Rect, Color32, &mut IconAnimationState, AnimationCtx)
                + Send
                + Sync,
        >,
    ),
}

/// Configuration for a custom icon button (internal use only)
pub struct CustomIconButton {
    pub icon: CustomIcon,
    pub tooltip: Option<String>,
    pub hover_color: Option<Color32>,
    pub icon_color: Option<Color32>,
    pub callback: Option<Box<dyn Fn() + Send + Sync>>,
    pub shortcut: Option<crate::KeyboardShortcut>,
}

pub struct TitleBar {
    pub title: Option<String>,
    pub id: Id,
    pub background_color: Color32,
    pub hover_color: Color32,
    pub close_hover_color: Color32,
    pub close_icon_color: Color32,
    pub maximize_icon_color: Color32,
    pub restore_icon_color: Color32,
    pub minimize_icon_color: Color32,
    pub menu_items: Vec<(String, Option<Box<dyn Fn() + Send + Sync>>)>,
    pub menu_items_with_submenus: Vec<MenuItem>,
    pub open_submenu: Option<usize>,    // Track which submenu is open
    pub submenu_open_time: Option<f64>, // Time when submenu was opened
    pub submenu_just_opened_frame: bool, // Prevent closing on the same frame it was opened
    pub last_click_time: f64,           // Track when the last click happened
    pub last_click_id: usize,           // Track the ID of the last click that opened submenu
    pub menu_positions: Vec<f32>,       // Store x positions of menu items for submenu alignment
    pub custom_icons: Vec<CustomIconButton>,
    pub app_icon: Option<ImageSource<'static>>,
    pub title_color: Color32,
    pub title_font_size: f32,
    pub theme_mode: ThemeMode,
    pub show_title_on_macos: bool,
    pub show_title_on_windows: bool,
    pub show_title_on_linux: bool,
    // Keyboard navigation state
    pub keyboard_navigation_active: bool, // Whether keyboard navigation is active
    pub selected_menu_index: Option<usize>, // Currently selected menu item
    pub selected_submenu_index: Option<usize>, // Currently selected submenu item (deprecated - use submenu_selections)
    pub last_keyboard_nav_time: f64,           // Time of last keyboard navigation
    pub force_open_child_subitem: Option<usize>, // When set, open child submenu for this subitem index
    pub selected_child_submenu_index: Option<usize>, // Currently selected child submenu item (deprecated - use child_submenu_selections)
    pub submenu_selections: std::collections::HashMap<usize, usize>, // Map submenu index to selected item index
    pub child_submenu_selections: std::collections::HashMap<usize, usize>, // Map submenu index to selected child index
    pub menu_text_color: Color32,
    pub menu_text_size: f32,
    pub menu_hover_color: Color32,
    pub keyboard_selection_color: Color32,
    // Submenu colors
    pub submenu_background_color: Color32,
    pub submenu_text_color: Color32,
    pub submenu_text_size: f32,
    pub submenu_hover_color: Color32,
    pub submenu_disabled_color: Color32,
    pub submenu_shortcut_color: Color32,
    pub submenu_border_color: Color32,
    pub submenu_keyboard_selection_color: Color32,
    // Optional external theme provider
    pub theme_provider: Option<Box<dyn ThemeProvider + Send + Sync>>,
    pub current_theme_id: Option<String>,
    // Control button visibility
    pub show_close_button: bool,
    pub show_maximize_button: bool,
    pub show_minimize_button: bool,
    // Per custom icon animation states (kept aligned with custom_icons)
    pub icon_animation_states: Vec<IconAnimationState>,
}

impl TitleBar {
    /// Create a new title bar with options
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Simple title bar with title
    /// TitleBar::new(TitleBarOptions::new().with_title("My App"))
    ///
    /// // Icon-only title bar
    /// TitleBar::new(TitleBarOptions::new())
    ///
    /// // Custom themed title bar
    /// TitleBar::new(
    ///     TitleBarOptions::new()
    ///         .with_title("My App")
    ///         .with_theme_mode(ThemeMode::Dark)
    ///         .with_background_color(Color32::from_rgb(30, 30, 30))
    /// )
    /// ```
    pub fn new(options: TitleBarOptions) -> Self {
        let theme = match options.theme_mode {
            ThemeMode::Light => TitleBarTheme::light(),
            ThemeMode::Dark => TitleBarTheme::dark(),
            ThemeMode::System => {
                if detect_system_dark_mode() {
                    TitleBarTheme::dark()
                } else {
                    TitleBarTheme::light()
                }
            }
        };

        let title_bar = Self {
            title: options.title,
            id: Id::new("title_bar"),
            background_color: options.background_color.unwrap_or(theme.background_color),
            hover_color: options.hover_color.unwrap_or(theme.hover_color),
            close_hover_color: options.close_hover_color.unwrap_or(theme.close_hover_color),
            close_icon_color: options.close_icon_color.unwrap_or(theme.close_icon_color),
            maximize_icon_color: options
                .maximize_icon_color
                .unwrap_or(theme.maximize_icon_color),
            restore_icon_color: options
                .restore_icon_color
                .unwrap_or(theme.restore_icon_color),
            minimize_icon_color: options
                .minimize_icon_color
                .unwrap_or(theme.minimize_icon_color),
            menu_items: Vec::new(),
            menu_items_with_submenus: Vec::new(),
            open_submenu: None,
            submenu_open_time: None,
            submenu_just_opened_frame: false,
            last_click_time: 0.0,
            last_click_id: 0,
            menu_positions: Vec::new(),
            custom_icons: Vec::new(),
            app_icon: options.app_icon,
            // Initialize keyboard navigation state
            keyboard_navigation_active: false,
            selected_menu_index: None,
            selected_submenu_index: None,
            last_keyboard_nav_time: 0.0,
            force_open_child_subitem: None,
            selected_child_submenu_index: None,
            submenu_selections: std::collections::HashMap::new(),
            child_submenu_selections: std::collections::HashMap::new(),
            title_color: options.title_color.unwrap_or(theme.title_color),
            title_font_size: options.title_font_size.unwrap_or(12.0),
            theme_mode: options.theme_mode,
            show_title_on_macos: options.show_title_on_macos,
            show_title_on_windows: options.show_title_on_windows,
            show_title_on_linux: options.show_title_on_linux,
            menu_text_color: options.menu_text_color.unwrap_or(theme.menu_text_color),
            menu_text_size: options.menu_text_size.unwrap_or(theme.menu_text_size),
            menu_hover_color: options.menu_hover_color.unwrap_or(theme.menu_hover_color),
            keyboard_selection_color: options
                .keyboard_selection_color
                .unwrap_or(theme.keyboard_selection_color),
            // Submenu colors
            submenu_background_color: theme.submenu_background_color,
            submenu_text_color: theme.submenu_text_color,
            submenu_text_size: theme.submenu_text_size,
            submenu_hover_color: theme.submenu_hover_color,
            submenu_disabled_color: theme.submenu_disabled_color,
            submenu_shortcut_color: theme.submenu_shortcut_color,
            submenu_border_color: theme.submenu_border_color,
            submenu_keyboard_selection_color: theme.submenu_keyboard_selection_color,
            // Theme provider
            theme_provider: None,
            current_theme_id: None,
            // Control button visibility (default to true if not specified)
            show_close_button: options.show_close_button.unwrap_or(true),
            show_maximize_button: options.show_maximize_button.unwrap_or(true),
            show_minimize_button: options.show_minimize_button.unwrap_or(true),
            icon_animation_states: Vec::new(),
        };

        title_bar
    }
}

/// Public animation context passed to animated icon callbacks
#[derive(Clone, Copy)]
pub struct AnimationCtx {
    pub time: f64,
    pub delta_seconds: f32,
    pub hovered: bool,
    pub pressed: bool,
}

/// Per-icon animation state managed by the framework
#[derive(Clone, Copy, Default)]
pub struct IconAnimationState {
    /// 0..1 smooth hover progress
    pub hover_t: f32,
    /// 0..1 smooth press progress
    pub press_t: f32,
    /// Last absolute time seen by this icon
    pub last_time: f64,
    /// Generic 0..1 progress you can drive from the callback
    pub progress: f32,
}
