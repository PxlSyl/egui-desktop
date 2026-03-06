use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex, atomic::AtomicUsize},
};

// Global state for submenu management
pub static SUBMENU_CLICK_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Global state for cascading submenu management
pub static MENU_STATE: LazyLock<Mutex<HashMap<String, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
