use eframe::Frame;
use egui::Context;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::{
    collections::HashSet,
    ffi::c_void,
    sync::{LazyLock, Mutex, Once},
};

use crate::utils::os::apply_native_rounded_corners;

// Track which viewports have had rounded corners applied
static APPLIED_VIEWPORTS: LazyLock<Mutex<HashSet<egui::ViewportId>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

/// Applies native rounded corners to the window if supported on the current platform.
/// This should be called once after the window is created.
///
/// For the main window, use this function with the `Frame` from `eframe::App::update`.
pub fn apply_rounded_corners(frame: &Frame) {
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        if let Ok(window_handle) = frame.window_handle() {
            apply_rounded_corners_from_handle(window_handle);
        }
    });
}

/// Stores window handle information in the egui context for later use in viewport callbacks.
/// This allows applying rounded corners to secondary viewports.
///
/// **Important**: This function stores the handle from the main window's Frame.
/// For secondary viewports, you need to get the handle from the viewport's Frame.
/// See `apply_rounded_corners_to_viewport` for a better approach.
///
/// # Arguments
/// * `ctx` - The egui context
/// * `frame` - The Frame containing the window handle to store
/// * `viewport_id` - The ID of the viewport this handle is for
pub fn store_window_handle_for_viewport(
    ctx: &Context,
    frame: &Frame,
    viewport_id: egui::ViewportId,
) {
    if let Ok(window_handle) = frame.window_handle() {
        let raw_handle: RawWindowHandle = window_handle.into();

        // Extract the platform-specific pointer (which is Send + Sync)
        let ptr: Option<*mut c_void> = match raw_handle {
            RawWindowHandle::Win32(h) => Some(h.hwnd.get() as *mut _),
            RawWindowHandle::AppKit(h) => Some(h.ns_view.as_ptr() as *mut _),
            RawWindowHandle::Xlib(h) => Some(h.window as *mut _),
            RawWindowHandle::Wayland(h) => Some(h.surface.as_ptr() as *mut _),
            _ => None,
        };

        if let Some(native_ptr) = ptr {
            let id = egui::Id::new(("rounded_corners_ptr", viewport_id));
            // Store as usize (which is Send + Sync) instead of raw pointer
            let ptr_as_usize = native_ptr as usize;
            ctx.data_mut(|data| {
                data.insert_temp(id, ptr_as_usize);
            });
        }
    }
}

/// Applies native rounded corners to a viewport window.
/// This should be called once per viewport after the window is created.
///
/// For secondary windows (viewports), use this function in the viewport's callback.
/// It will only apply rounded corners once per viewport, but will reapply if the window
/// was closed and reopened.
///
/// This function attempts multiple methods to get the window handle:
/// 1. First tries to get a stored handle from the context
/// 2. Then tries to get the handle using platform-specific APIs (by window title)
///
/// # Example
///
/// ```rust
/// // In your viewport callback:
/// ctx.show_viewport_deferred(viewport_id, viewport_builder, move |ctx, _class| {
///     // Apply rounded corners (will attempt to find the window handle)
///     apply_rounded_corners_to_viewport(ctx);
///     // ... rest of your UI
/// });
/// ```
pub fn apply_rounded_corners_to_viewport(ctx: &Context) {
    let viewport_id = ctx.viewport_id();

    // Check if viewport is being closed - if so, remove it from the applied list
    // so it can be reapplied when reopened
    if ctx.input(|i| i.viewport().close_requested()) {
        let mut applied = APPLIED_VIEWPORTS.lock().unwrap();
        applied.remove(&viewport_id);
        // Also clear the stored handle
        let id = egui::Id::new(("rounded_corners_ptr", viewport_id));
        ctx.data_mut(|data| {
            data.remove::<usize>(id);
        });
        return;
    }

    // Try to get a window handle - if we can get one, apply rounded corners
    // We don't check if we've already applied because the window handle might have changed
    // (e.g., if the window was closed and reopened, it's a new native window)

    // Try method 1: Get stored window handle pointer from the context
    let id = egui::Id::new(("rounded_corners_ptr", viewport_id));
    let mut handle_found = false;

    if let Some(ptr_as_usize) = ctx.data(|data| data.get_temp::<usize>(id)) {
        // Convert back from usize to pointer
        let native_ptr = ptr_as_usize as *mut c_void;

        match apply_native_rounded_corners(native_ptr) {
            Ok(_) => {
                // println!("🎉 Native rounded corners applied successfully to viewport!");
                handle_found = true;
            }
            Err(e) => {
                eprintln!(
                    "⚠️ Failed to apply native rounded corners to viewport (stored handle): {}",
                    e
                );
                // Handle might be invalid (window closed), try method 2
            }
        }
    }

    // Try method 2: Get window handle using platform-specific APIs
    // This is especially useful when the window is reopened (new native window)
    if !handle_found {
        if let Some(native_ptr) = get_viewport_window_handle(ctx) {
            match apply_native_rounded_corners(native_ptr) {
                Ok(_) => {
                    println!("🎉 Native rounded corners applied successfully to viewport!");
                    // Store the new handle for future use
                    let ptr_as_usize = native_ptr as usize;
                    ctx.data_mut(|data| {
                        data.insert_temp(id, ptr_as_usize);
                    });
                    handle_found = true;
                }
                Err(e) => {
                    eprintln!(
                        "⚠️ Failed to apply native rounded corners to viewport (found handle): {}",
                        e
                    );
                }
            }
        }
    }

    if !handle_found {
        eprintln!(
            "⚠️ Could not apply rounded corners to viewport {:?}: Window handle not found. \
             This may happen if the window hasn't been fully created yet.",
            viewport_id
        );
    }
}

/// Attempts to get the window handle for a viewport using platform-specific APIs.
/// This is a fallback when the handle isn't stored in the context.
fn get_viewport_window_handle(ctx: &Context) -> Option<*mut c_void> {
    // Get the viewport title to find the window
    let viewport_title = ctx.input(|i| i.viewport().title.clone())?;

    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowThreadProcessId};
        use windows::core::PCWSTR;

        // Convert title to wide string
        let title_wide: Vec<u16> = OsStr::new(&viewport_title)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        unsafe {
            // Try to find the window by its title
            // FindWindowW returns Result<HWND, Error>, so we need to handle it
            if let Ok(hwnd) = FindWindowW(None, PCWSTR::from_raw(title_wide.as_ptr())) {
                if !hwnd.is_invalid() {
                    // Verify it's actually our window by checking the process ID
                    let mut process_id = 0u32;
                    GetWindowThreadProcessId(hwnd, Some(&mut process_id));
                    if process_id == std::process::id() {
                        return Some(hwnd.0 as *mut c_void);
                    }
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        unsafe {
            // Get all windows and find the one with matching title
            use objc2::{MainThreadMarker, msg_send, runtime::AnyObject};
            use objc2_app_kit::NSApp;

            let app = NSApp(MainThreadMarker::new().unwrap());
            let windows: *mut AnyObject = msg_send![<_ as AsRef<AnyObject>>::as_ref(&app), windows];
            let count: usize = msg_send![windows, count];

            for i in 0..count {
                use objc2::ffi::nil;

                let window: *mut AnyObject = msg_send![windows, objectAtIndex: i];
                let title: *mut AnyObject = msg_send![window, title];

                if title != nil {
                    let title_str: String = {
                        let c_str: *const std::os::raw::c_char = msg_send![title, UTF8String];
                        if c_str.is_null() {
                            continue;
                        }
                        let c_str = std::ffi::CStr::from_ptr(c_str);
                        c_str.to_string_lossy().into_owned()
                    };
                    if title_str == viewport_title {
                        // Get the content view
                        let content_view: *mut AnyObject = msg_send![window, contentView];
                        if content_view != nil {
                            return Some(content_view as *mut c_void);
                        }
                    }
                }
            }
        }
    }

    // Linux/X11 and Wayland - not implemented yet
    #[cfg(target_os = "linux")]
    {
        // TODO: Implement for Linux
    }

    None
}

/// Internal helper to apply rounded corners from a window handle
fn apply_rounded_corners_from_handle(window_handle: raw_window_handle::WindowHandle) {
    let handle: RawWindowHandle = window_handle.into();

    let ptr: Option<*mut c_void> = match handle {
        RawWindowHandle::Win32(h) => {
            println!("🪟 Windows: Using Win32 window handle");
            Some(h.hwnd.get() as *mut _)
        }
        RawWindowHandle::AppKit(h) => {
            println!("🍎 macOS: Using AppKit window handle");
            Some(h.ns_view.as_ptr() as *mut _)
        }
        RawWindowHandle::Xlib(h) => {
            println!("🐧 Linux X11: Using Xlib window handle");
            Some(h.window as *mut _)
        }
        RawWindowHandle::Wayland(h) => {
            println!("🐧 Linux Wayland: Using Wayland surface handle");
            Some(h.surface.as_ptr() as *mut _)
        }
        _ => {
            println!(
                "ℹ️ Platform: Native rounded corners not supported for this window handle type: {:?}",
                handle
            );
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
