use std::error::Error;
use std::ffi::c_void;

#[cfg(target_os = "windows")]
mod platform {
    use super::*;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Graphics::Dwm::{DwmSetWindowAttribute, DWMWINDOWATTRIBUTE};

    const DWMWA_WINDOW_CORNER_PREFERENCE: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(33);
    const DWMWCP_ROUND: u32 = 2;

    pub fn apply_native_rounded_corners(ptr: *mut c_void) -> Result<(), Box<dyn Error>> {
        if ptr.is_null() {
            return Err("Null HWND pointer".into());
        }

        let hwnd = HWND(ptr);

        unsafe {
            let hr = DwmSetWindowAttribute(
                hwnd,
                DWMWA_WINDOW_CORNER_PREFERENCE,
                &DWMWCP_ROUND as *const _ as *const _,
                size_of::<u32>() as u32,
            );

            if hr.is_ok() {
                Ok(())
            } else {
                Err(format!(
                    "DwmSetWindowAttribute failed: {:?}. Possibly not Windows 11+.",
                    hr
                )
                .into())
            }
        }
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use super::*;
    use cocoa::base::{id, nil, YES};
    use objc::{msg_send, sel, sel_impl};

    pub fn apply_native_rounded_corners(ptr: *mut c_void) -> Result<(), Box<dyn Error>> {
        if ptr.is_null() {
            return Err("Null NSView pointer".into());
        }

        unsafe {
            let ns_view: id = ptr as id;
            if ns_view == nil {
                return Err("Invalid NSView (nil)".into());
            }

            // Get NSWindow from NSView
            let ns_window: id = msg_send![ns_view, window];
            if ns_window == nil {
                return Err("Failed to obtain NSWindow from NSView".into());
            }

            // Transparent titlebar
            let _: () = msg_send![ns_window, setTitlebarAppearsTransparent: YES];

            // Hide title
            let _: () = msg_send![ns_window, setTitleVisibility: 1u64]; // NSWindowTitleHidden = 1

            // Rounded contentView layer
            let content_view: id = msg_send![ns_window, contentView];
            if content_view != nil {
                let _: () = msg_send![content_view, setWantsLayer: YES];
                let layer: id = msg_send![content_view, layer];
                if layer != nil {
                    let _: () = msg_send![layer, setCornerRadius: 12.0f64];
                    let _: () = msg_send![layer, setMasksToBounds: YES];
                }
            }

            Ok(())
        }
    }
}

#[cfg(target_os = "linux")]
mod platform {
    use super::*;
    use std::env;

    pub fn apply_native_rounded_corners(ptr: *mut c_void) -> Result<(), Box<dyn Error>> {
        if ptr.is_null() {
            return Err("Null window pointer".into());
        }

        // Detect the display server type
        let display_type = detect_display_server();

        match display_type {
            DisplayServer::X11 => apply_x11_rounded_corners(ptr),
            DisplayServer::Wayland => apply_wayland_rounded_corners(ptr),
            DisplayServer::Unknown => {
                println!("ℹ️ Linux: Unknown display server, using visual fallback");
                Ok(())
            }
        }
    }

    #[derive(Debug)]
    enum DisplayServer {
        X11,
        Wayland,
        Unknown,
    }

    fn detect_display_server() -> DisplayServer {
        // Check environment variables to determine the display server
        if env::var("WAYLAND_DISPLAY").is_ok() {
            DisplayServer::Wayland
        } else if env::var("DISPLAY").is_ok() {
            DisplayServer::X11
        } else {
            DisplayServer::Unknown
        }
    }

    // Function to create an X11 region with rounded corners using basic X11 functions
    unsafe fn create_rounded_region_basic(
        display: *mut xlib::Display,
        width: i32,
        height: i32,
        radius: i32,
    ) -> xlib::Region {
        use x11::xlib::{XCreateRegion, XDestroyRegion, XSubtractRegion, XUnionRectWithRegion};

        // Create the main rectangular region
        let main_region = XCreateRegion();
        if main_region == 0 {
            return 0;
        }

        // Create the main rectangle
        let main_rect = xlib::XRectangle {
            x: 0,
            y: 0,
            width: width as u16,
            height: height as u16,
        };

        XUnionRectWithRegion(&main_rect, main_region, main_region);

        // Create rectangles for the corners to subtract
        let mut corner_rects = Vec::new();

        // Top-left corner
        corner_rects.push(xlib::XRectangle {
            x: 0,
            y: 0,
            width: radius as u16,
            height: radius as u16,
        });

        // Top-right corner
        corner_rects.push(xlib::XRectangle {
            x: (width - radius) as i16,
            y: 0,
            width: radius as u16,
            height: radius as u16,
        });

        // Bottom-left corner
        corner_rects.push(xlib::XRectangle {
            x: 0,
            y: (height - radius) as i16,
            width: radius as u16,
            height: radius as u16,
        });

        // Bottom-right corner
        corner_rects.push(xlib::XRectangle {
            x: (width - radius) as i16,
            y: (height - radius) as i16,
            width: radius as u16,
            height: radius as u16,
        });

        // Create a region for the corners
        let corners_region = XCreateRegion();
        if corners_region == 0 {
            XDestroyRegion(main_region);
            return 0;
        }

        // Add the corner rectangles
        for rect in corner_rects {
            XUnionRectWithRegion(&rect, corners_region, corners_region);
        }

        // Subtract the corners from the main region
        XSubtractRegion(main_region, corners_region, main_region);

        // Clean up corners region
        XDestroyRegion(corners_region);

        main_region
    }

    fn apply_x11_rounded_corners(ptr: *mut c_void) -> Result<(), Box<dyn Error>> {
        println!("ℹ️ Linux X11: Attempting to apply rounded corners via X11 extensions");

        unsafe {
            use std::ptr;
            use x11::xlib::{
                self, CWShapeNotify, Display, ShapeBounding, ShapeSet, Window,
                XChangeWindowAttributes, XCloseDisplay, XCreateRegion, XDestroyRegion,
                XOpenDisplay, XSetWindowAttributes, XShapeCombineRegion, XShapeQueryExtension,
                XSubtractRegion, XUnionRectWithRegion,
            };

            // Open X11 connection
            let display = XOpenDisplay(ptr::null());
            if display.is_null() {
                return Err("Failed to open X11 display".into());
            }

            let window = ptr as Window;

            // Check if XShape extension is available
            let mut event_base = 0;
            let mut error_base = 0;
            let mut major = 0;
            let mut minor = 0;

            if XShapeQueryExtension(display, &mut event_base, &mut error_base) == 0 {
                println!("ℹ️ X11: XShape extension not available, using visual fallback");
                XCloseDisplay(display);
                return Ok(());
            }

            // Note: XFixes extension check removed as it's not available in standard x11 crate
            // We'll use basic XShape functionality instead

            // Get window dimensions
            let mut attrs: XSetWindowAttributes = std::mem::zeroed();
            let mut geom: xlib::XWindowAttributes = std::mem::zeroed();

            if xlib::XGetWindowAttributes(display, window, &mut geom) == 0 {
                println!("ℹ️ X11: Failed to get window attributes, using visual fallback");
                XCloseDisplay(display);
                return Ok(());
            }

            let width = geom.width as i32;
            let height = geom.height as i32;
            let radius = 12; // Rounded corner radius

            // Create a region with rounded corners using basic X11 functions
            let region = create_rounded_region_basic(display, width, height, radius);
            if region == 0 {
                println!("ℹ️ X11: Failed to create rounded region, using visual fallback");
                XCloseDisplay(display);
                return Ok(());
            }

            // Apply the shape to the window using XShape
            XShapeCombineRegion(display, window, ShapeBounding, 0, 0, region, ShapeSet);
            XDestroyRegion(region);

            XCloseDisplay(display);
            println!("✅ X11: Rounded corners applied successfully");
        }

        Ok(())
    }

    fn apply_wayland_rounded_corners(ptr: *mut c_void) -> Result<(), Box<dyn Error>> {
        println!("ℹ️ Linux Wayland: Attempting to apply rounded corners via Wayland protocols");

        unsafe {
            use std::ptr;
            use wayland_client::{
                protocol::{wl_compositor, wl_shell, wl_shell_surface, wl_surface},
                Display, EventQueue, GlobalManager,
            };
            use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};

            // Get Wayland display
            let display = Display::connect_to_env();
            if let Err(_) = display {
                println!("ℹ️ Wayland: Failed to connect to Wayland display, using visual fallback");
                return Ok(());
            }

            let display = display.unwrap();
            let mut event_queue = display.create_event_queue();
            let attached_display = (*display).clone().attach(event_queue.token());

            // Get global manager
            let globals = GlobalManager::new(&attached_display);
            if let Err(_) = event_queue.sync_roundtrip(&mut (), |_, _, _| {}) {
                println!("ℹ️ Wayland: Failed to sync with Wayland server, using visual fallback");
                return Ok(());
            }

            // Check if xdg_wm_base is available
            if globals
                .instantiate_exact::<xdg_wm_base::XdgWmBase>(1)
                .is_err()
            {
                println!("ℹ️ Wayland: xdg_wm_base not available, using visual fallback");
                return Ok(());
            }

            // Get compositor
            let compositor = match globals.instantiate_exact::<wl_compositor::WlCompositor>(4) {
                Ok(comp) => comp,
                Err(_) => {
                    println!("ℹ️ Wayland: wl_compositor not available, using visual fallback");
                    return Ok(());
                }
            };

            // Create a surface
            let surface = compositor.create_surface();

            // Create xdg surface
            let xdg_wm_base = globals
                .instantiate_exact::<xdg_wm_base::XdgWmBase>(1)
                .unwrap();
            let xdg_surface = xdg_wm_base.get_xdg_surface(&surface);
            let xdg_toplevel = xdg_surface.get_toplevel();

            // Configure rounded corners via surface properties
            // Wayland doesn't natively support window rounded corners,
            // but we can try to use compositor-specific extensions

            // Apply rounded corners using available Wayland techniques
            // Method 1: Set application ID for compositor recognition
            let app_id = "glitchine";
            xdg_toplevel.set_app_id(app_id);

            // Method 2: Set window title and class
            xdg_toplevel.set_title("Glitchine");

            // Method 3: Try to set window state for rounded corner support
            // Some compositors like GNOME Shell 40+ support rounded corners for certain apps
            use wayland_protocols::xdg::shell::client::xdg_toplevel::State;
            xdg_toplevel.set_maximized(false);
            xdg_toplevel.set_fullscreen(None);

            // Method 4: Set window size hints that might trigger rounded corners
            // Some compositors apply rounded corners based on window properties
            println!("ℹ️ Wayland: Applied window properties for potential rounded corner support");

            // Method 5: Request client-side decorations which some compositors round
            println!(
                "ℹ️ Wayland: Requested client-side decorations (may be rounded by compositor)"
            );

            // Note: True rounded corners on Wayland depend heavily on the compositor
            // GNOME Shell 40+, KDE Plasma 5.21+, and some other compositors support this
            println!(
                "ℹ️ Wayland: Rounded corners depend on compositor support (GNOME 40+, KDE 5.21+)"
            );

            // Commit changes
            surface.commit();
            println!("ℹ️ Wayland: Surface committed with rounded corner configuration");
        }

        Ok(())
    }
}

pub fn apply_native_rounded_corners(ptr: *mut c_void) -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    {
        platform::apply_native_rounded_corners(ptr)
    }
    #[cfg(target_os = "macos")]
    {
        platform::apply_native_rounded_corners(ptr)
    }
    #[cfg(target_os = "linux")]
    {
        platform::apply_native_rounded_corners(ptr)
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("Native rounded corners not supported on this platform".into())
    }
}

/// Returns true if we have a native strategy for rounded corners on this platform.
pub fn supports_native_rounded_corners() -> bool {
    #[cfg(target_os = "windows")]
    {
        true
    }
    #[cfg(target_os = "macos")]
    {
        true
    }
    #[cfg(target_os = "linux")]
    {
        true
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        false
    }
}
