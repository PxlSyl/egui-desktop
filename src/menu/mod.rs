/// Public API for rendering menus in the title bar.
pub mod api;
/// Menu item types and submenu structures.
pub mod items;
/// Minimal horizontal menu bar component.
pub mod menu_bar;
/// Keyboard shortcuts parsing and handling.
pub mod shortcuts;

pub use items::{MenuItem, SubMenuItem};
