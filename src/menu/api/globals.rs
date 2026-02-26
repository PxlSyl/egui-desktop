use std::sync::atomic::AtomicUsize;

// Global state for submenu management
pub static SUBMENU_CLICK_COUNTER: AtomicUsize = AtomicUsize::new(0);
