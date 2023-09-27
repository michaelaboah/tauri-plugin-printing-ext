mod dialogs;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let builder = Builder::new("printing-ext");

    if cfg!(feature = "macos-fix") {
        #[cfg(target_os = "macos")]
        let builder = builder.invoke_handler(tauri::generate_handler![dialogs::print_dialog]);
        return builder.build();
    }

    return builder.build();
}
