use eframe::Frame;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::{ffi::c_void, sync::Once};

use crate::utils::os::apply_native_rounded_corners;

/// Applies native rounded corners to the window if supported on the current platform.
/// This should be called once after the window is created.
pub fn apply_rounded_corners(frame: &Frame) {
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        if let Ok(window_handle) = frame.window_handle() {
            let handle = window_handle.into();

            let ptr: Option<*mut c_void> = match handle {
                RawWindowHandle::Win32(h) => {
                    println!("🪟 Windows: Using Win32 window handle");
                    Some(h.hwnd.get() as *mut _)
                },
                RawWindowHandle::AppKit(h) => {
                    println!("🍎 macOS: Using AppKit window handle");
                    Some(h.ns_view.as_ptr() as *mut _)
                },
                RawWindowHandle::Xlib(h) => {
                    println!("🐧 Linux X11: Using Xlib window handle");
                    Some(h.window as *mut _)
                },
                RawWindowHandle::Wayland(h) => {
                    println!("🐧 Linux Wayland: Using Wayland surface handle");
                    Some(h.surface.as_ptr() as *mut _)
                },
                _ => {
                    println!("ℹ️ Platform: Native rounded corners not supported for this window handle type: {:?}", handle);
                    None
                }
            };

            if let Some(native_ptr) = ptr {
                match apply_native_rounded_corners(native_ptr) {
                    Ok(_) => println!("🎉 Native rounded corners applied successfully!"),
                    Err(e) => eprintln!("⚠️ Failed to apply native rounded corners: {}", e),
                }
            }
        }
    });
}
