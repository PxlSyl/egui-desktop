//! Windows-specific utilities for egui-desktop-ui
//!
//! This module provides Windows-specific functionality for Windows 11 snap layouts
//! on custom maximize buttons in egui-desktop-ui applications.
use core::ffi::c_void;
use egui::Rect;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::{mem::transmute, sync::Once};
use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM},
    UI::{
        Input::KeyboardAndMouse::{GetAsyncKeyState, VK_LBUTTON},
        WindowsAndMessaging::{
            DefWindowProcW, GWLP_WNDPROC, GetWindowLongPtrW, GetWindowRect, HTCAPTION, HTCLIENT,
            HTMAXBUTTON, SetWindowLongPtrW, WM_NCHITTEST,
        },
    },
};

/// Global state for Windows hit testing
static mut MAXIMIZE_BUTTON_SCREEN_RECT: Option<RECT> = None;
static mut MINIMIZE_BUTTON_SCREEN_RECT: Option<RECT> = None;
static mut CLOSE_BUTTON_SCREEN_RECT: Option<RECT> = None;
static mut ORIGINAL_WNDPROC: Option<
    unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT,
> = None;
static mut WINDOW_PROC_INSTALLED: bool = false;

/// Set the maximize button screen rectangle directly
pub fn set_maximize_button_screen_rect(rect: RECT) {
    unsafe {
        MAXIMIZE_BUTTON_SCREEN_RECT = Some(rect);
    }
}

/// Set the minimize button screen rectangle
pub fn set_minimize_button_screen_rect(rect: RECT) {
    unsafe {
        MINIMIZE_BUTTON_SCREEN_RECT = Some(rect);
    }
}

/// Set the close button screen rectangle
pub fn set_close_button_screen_rect(rect: RECT) {
    unsafe {
        CLOSE_BUTTON_SCREEN_RECT = Some(rect);
    }
}

/// Convert egui rect to Windows screen coordinates (pixel-perfect)
pub fn egui_rect_to_screen(hwnd: HWND, rect: Rect, scale: f32) -> RECT {
    let mut window_rect = RECT::default();
    unsafe {
        let _ = GetWindowRect(hwnd, &mut window_rect);
    }

    let border = 8.0 * scale;

    let x = window_rect.left as f32 + rect.min.x * scale;
    let y = window_rect.top as f32 + rect.min.y * scale - border;
    let w = rect.width() * scale;
    let h = rect.height() * scale;

    RECT {
        left: x as i32,
        top: y as i32,
        right: (x + w) as i32,
        bottom: (y + h) as i32,
    }
}

/// Clear all button screen rectangles
pub fn clear_all_button_screen_rects() {
    unsafe {
        MAXIMIZE_BUTTON_SCREEN_RECT = None;
        MINIMIZE_BUTTON_SCREEN_RECT = None;
        CLOSE_BUTTON_SCREEN_RECT = None;
    }
}

/// Custom window procedure for handling WM_NCHITTEST
unsafe extern "system" fn custom_window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_NCHITTEST => {
            let x = (lparam.0 as i32 & 0xFFFF) as i16;
            let y = ((lparam.0 as i32 >> 16) & 0xFFFF) as i16;
            let point = POINT {
                x: x as i32,
                y: y as i32,
            };

            if let Some(rect) = unsafe { MAXIMIZE_BUTTON_SCREEN_RECT } {
                let inside = point.x >= rect.left
                    && point.x <= rect.right
                    && point.y >= rect.top
                    && point.y <= rect.bottom;

                if inside {
                    let key_state = unsafe { GetAsyncKeyState(VK_LBUTTON.0 as i32) } as i16;
                    let mouse_down = (key_state & 0x8000u16 as i16) != 0;

                    if mouse_down {
                        // Clic → laisser egui gérer
                        return LRESULT(HTCLIENT as isize);
                    } else {
                        // Hover → Snap Layout Windows
                        return LRESULT(HTMAXBUTTON as isize);
                    }
                }
            }

            // Handle titlebar drag excluding buttons
            let titlebar_top = unsafe { MAXIMIZE_BUTTON_SCREEN_RECT.map(|r| r.top).unwrap_or(0) };
            let titlebar_bottom = titlebar_top + 32;

            if point.y >= titlebar_top && point.y <= titlebar_bottom {
                let in_minimize_area = unsafe {
                    MINIMIZE_BUTTON_SCREEN_RECT
                        .map(|r| {
                            point.x >= r.left
                                && point.x <= r.right
                                && point.y >= r.top
                                && point.y <= r.bottom
                        })
                        .unwrap_or(false)
                };

                let in_close_area = unsafe {
                    CLOSE_BUTTON_SCREEN_RECT
                        .map(|r| {
                            point.x >= r.left
                                && point.x <= r.right
                                && point.y >= r.top
                                && point.y <= r.bottom
                        })
                        .unwrap_or(false)
                };

                let in_maximize_area = unsafe {
                    MAXIMIZE_BUTTON_SCREEN_RECT
                        .map(|r| {
                            point.x >= r.left
                                && point.x <= r.right
                                && point.y >= r.top
                                && point.y <= r.bottom
                        })
                        .unwrap_or(false)
                };

                if !in_minimize_area && !in_close_area && !in_maximize_area {
                    return LRESULT(HTCAPTION as isize);
                }
            }

            LRESULT(HTCLIENT as isize)
        }
        _ => {
            if let Some(original_proc) = unsafe { ORIGINAL_WNDPROC } {
                unsafe { original_proc(hwnd, msg, wparam, lparam) }
            } else {
                unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
            }
        }
    }
}

/// Install the custom window procedure
fn install_window_proc(hwnd: HWND) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        if WINDOW_PROC_INSTALLED {
            return Ok(());
        }

        let current_proc = GetWindowLongPtrW(hwnd, GWLP_WNDPROC);
        if current_proc == 0 {
            return Err("Failed to get current window procedure".into());
        }

        ORIGINAL_WNDPROC = Some(transmute(current_proc));
        SetWindowLongPtrW(
            hwnd,
            GWLP_WNDPROC,
            custom_window_proc as *const () as usize as isize,
        );

        WINDOW_PROC_INSTALLED = true;
        Ok(())
    }
}

/// Initialize Windows snap layouts support
pub fn initialize_windows_snap_layouts(
    frame: &eframe::Frame,
) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        static INIT: Once = Once::new();

        INIT.call_once(|| {
            if let Ok(window_handle) = frame.window_handle() {
                let raw_handle: RawWindowHandle = window_handle.into();
                if let RawWindowHandle::Win32(handle) = raw_handle {
                    let hwnd = HWND(handle.hwnd.get() as *mut c_void);
                    if let Err(e) = install_window_proc(hwnd) {
                        eprintln!("Failed to install Windows snap layouts support: {}", e);
                    }
                }
            }
        });
    }

    Ok(())
}
