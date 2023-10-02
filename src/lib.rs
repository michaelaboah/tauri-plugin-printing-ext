mod dialogs;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Wry,
};

/// Initializes the plugin.
pub fn init() -> TauriPlugin<Wry> {
    #[cfg(target_os = "windows")]
    return Builder::new("printing-ext").build();

    #[cfg(target_os = "linux")]
    return Builder::new("printing-ext")
        .invoke_handler(tauri::generate_handler![dialogs::linux::print_dialog])
        .build();

    #[cfg(target_os = "macos")]
    return Builder::new("printing-ext")
        .invoke_handler(tauri::generate_handler![dialogs::macos::print_dialog])
        .build();
}
