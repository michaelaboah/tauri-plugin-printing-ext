#[cfg(target_os = "macos")]
mod dialogs;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

#[tauri::command]
fn greet() {}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    #[cfg(not(target_os = "macos"))]
    return Builder::new("printing-ext").build();

    #[cfg(target_os = "macos")]
    return Builder::new("printing-ext")
        .invoke_handler(tauri::generate_handler![dialogs::print_dialog])
        .build();
}
