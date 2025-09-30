//! egui-desktop
//!
//! High-level, cross-platform desktop UI components for `egui` applications.
//!
//! This crate provides:
//! - A customizable window title bar (`TitleBar`) with platform-aware behavior
//! - A menu system with keyboard shortcuts
//! - Theming utilities and helpers
//!
//! Quick start:
//! ```no_run
//! use egui_desktop::{TitleBar, TitleBarOptions, ThemeProvider};
//! # fn app_ui(ctx: &egui::Context) {
//! let theme = ThemeProvider::default();
//! let options = TitleBarOptions::default();
//! TitleBar::new("My App", options).show(ctx);
//! # }
//! ```
//!
//! See the README for detailed examples and the `examples/` directory for
//! runnable demos.
//!
/// Menu system primitives (items, menu bar, shortcuts).
pub mod menu;
/// Theming primitives and provider traits.
pub mod theme;
/// Title bar widgets, options and rendering helpers.
pub mod titlebar;
/// Utility helpers (OS interop, resize handles, rounded corners).
pub mod utils;

pub use menu::shortcuts::KeyboardShortcut;
pub use menu::{MenuItem, SubMenuItem};
pub use theme::{ThemeError, ThemeMode, ThemeProvider, TitleBarTheme, detect_system_dark_mode};
pub use titlebar::{main::CustomIcon, main::TitleBar, options::TitleBarOptions};
pub use utils::*;
