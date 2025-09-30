/// Public API for constructing and interacting with the title bar.
pub mod api;
/// Window control icons and drawing helpers.
pub mod control_buttons;
/// Core title bar types and data structures.
pub mod main;
/// Options and configuration for the title bar.
pub mod options;
/// Platform-specific rendering helpers for the title bar.
pub mod render_bar;

pub use main::*;
pub use options::*;
