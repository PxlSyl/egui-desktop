//! Windows-specific utilities for egui-desktop-ui
//!
//! This module provides Windows-specific functionality for Windows 11 snap layouts
//! on custom maximize buttons in egui-desktop-ui applications.
use core::ffi::c_void;
use egui::Rect;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::{mem::transmute, sync::Once, time::Instant};
use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM},
    UI::{
        Input::KeyboardAndMouse::{GetAsyncKeyState, VK_LBUTTON},
        WindowsAndMessaging::{
            DefWindowProcW, GWLP_WNDPROC, GetWindowLongPtrW, GetWindowRect, HTCLIENT, HTMAXBUTTON,
            SetWindowLongPtrW, WM_NCHITTEST,
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
static mut HOVERING_MAXIMIZE: bool = false;
static mut HOVER_START_TIME: Option<Instant> = None;

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

    // Pas de border correction - on veut que les zones soient parfaitement alignées
    // egui rect est déjà en coordonnées client correctes

    let x = window_rect.left as f32 + rect.min.x * scale;
    let y = window_rect.top as f32 + rect.min.y * scale;
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

/// Check if currently hovering over maximize button
pub fn is_hovering_maximize() -> bool {
    unsafe { HOVERING_MAXIMIZE }
}

/// Custom window procedure for handling WM_NCHITTEST - SNAP LAYOUT SEULEMENT SUR BOUTON MAXIMIZE
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

            // Détecter UNIQUEMENT la zone exacte du bouton maximize
            // PAS de marge, PAS de zone étendue - juste le bouton lui-même
            if let Some(rect) = unsafe { MAXIMIZE_BUTTON_SCREEN_RECT } {
                let inside = point.x >= rect.left
                    && point.x <= rect.right
                    && point.y >= rect.top
                    && point.y <= rect.bottom;

                // Mettre à jour l'état de hover et le timer
                unsafe {
                    if inside {
                        if HOVERING_MAXIMIZE == false {
                            // Nouveau hover - démarrer le timer
                            HOVERING_MAXIMIZE = true;
                            HOVER_START_TIME = Some(Instant::now());
                        }
                    } else {
                        // Fin du hover - réinitialiser
                        HOVERING_MAXIMIZE = false;
                        HOVER_START_TIME = None;
                    }
                }

                if inside {
                    let key_state = unsafe { GetAsyncKeyState(VK_LBUTTON.0 as i32) } as i16;
                    let mouse_down = (key_state & 0x8000u16 as i16) != 0;

                    if mouse_down {
                        // Clic → laisser egui gérer normalement
                        return LRESULT(HTCLIENT as isize);
                    } else {
                        // Hover sur bouton maximize EXACT →
                        // Vérifier si on a assez de hover pour déclencher le snap layout
                        unsafe {
                            if let Some(start_time) = HOVER_START_TIME {
                                if start_time.elapsed().as_millis() > 500 {
                                    // 500ms de hover → déclencher le snap layout
                                    // Retourner HTMAXBUTTON uniquement pour le snap layout
                                    return LRESULT(HTMAXBUTTON as isize);
                                }
                            }
                        }

                        // Sinon, laisser egui gérer le hover
                        return LRESULT(HTCLIENT as isize);
                    }
                }
            } else {
                // Pas de rectangle défini → réinitialiser le hover
                unsafe {
                    HOVERING_MAXIMIZE = false;
                    HOVER_START_TIME = None;
                }
            }

            // Pour tout le reste: laisser egui gérer complètement
            // Pas de logique de titlebar, pas d'exclusions, juste HTCLIENT
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
