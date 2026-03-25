/// OS interop helpers and platform-specific utilities.
pub mod os;
/// Viewport resize handle utilities.
pub mod resize_handles;
/// Fallback rounded corners drawing helpers.
pub mod rounded_corners;
/// Windows-specific utilities for native window behavior
#[cfg(target_os = "windows")]
pub mod win11_snap_layouts;

pub use os::*;
pub use resize_handles::*;
pub use rounded_corners::*;
#[cfg(target_os = "windows")]
pub use win11_snap_layouts::*;
