pub mod commands;
pub mod core;

use commands::AppState;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = core::state::load();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState(Mutex::new(config)))
        .invoke_handler(tauri::generate_handler![
            commands::detect_sekiro_path,
            commands::set_sekiro_path,
            commands::get_status,
            commands::scan_installed_mods,
            commands::install_mod,
            commands::uninstall_mod,
            commands::toggle_mod,
            commands::check_conflicts,
            commands::install_mod_engine,
            commands::backup_save,
            commands::restore_save,
            commands::import_save,
            commands::list_save_backups,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
