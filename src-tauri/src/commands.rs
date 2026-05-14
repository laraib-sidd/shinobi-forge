use crate::core::{conflicts, detect, installer, saves, state, types::*};
use std::sync::Mutex;
use tauri::State;

pub struct AppState(pub Mutex<AppConfig>);

#[tauri::command]
pub fn detect_sekiro_path() -> Option<String> {
    detect::find_sekiro_path()
}

#[tauri::command]
pub fn set_sekiro_path(path: String, state: State<'_, AppState>) -> Result<GameInfo, String> {
    if !detect::validate_sekiro_path(&path) {
        return Err("sekiro.exe not found at the specified path.".to_string());
    }
    let mod_engine_installed = detect::check_mod_engine(&path);
    let mut config = state.0.lock().unwrap();
    config.sekiro_path = Some(path.clone());
    config.mod_engine_installed = mod_engine_installed;
    state::save(&config)?;
    Ok(GameInfo { path, mod_engine_installed })
}

#[tauri::command]
pub fn get_status(state: State<'_, AppState>) -> AppStatus {
    let config = state.0.lock().unwrap();
    let active = config.installed_mods.iter().filter(|m| m.enabled).count();
    AppStatus {
        sekiro_path: config.sekiro_path.clone(),
        mod_engine_installed: config.mod_engine_installed,
        active_mod_count: active,
        total_mod_count: config.installed_mods.len(),
        save_backup_count: config.save_backups.len(),
        conflicts: conflicts::check(&config),
        first_run: config.sekiro_path.is_none(),
    }
}

#[tauri::command]
pub fn scan_installed_mods(state: State<'_, AppState>) -> Vec<InstalledMod> {
    state.0.lock().unwrap().installed_mods.clone()
}

#[tauri::command]
pub fn install_mod(archive_path: String, state: State<'_, AppState>) -> Result<InstalledMod, String> {
    let mut config = state.0.lock().unwrap();
    let sekiro_path = config.sekiro_path.clone()
        .ok_or("Sekiro path not configured. Complete setup first.")?;
    let installed = installer::install(&archive_path, &sekiro_path)?;
    config.installed_mods.push(installed.clone());
    state::save(&config)?;
    Ok(installed)
}

#[tauri::command]
pub fn uninstall_mod(mod_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut config = state.0.lock().unwrap();
    let sekiro_path = config.sekiro_path.clone()
        .ok_or("Sekiro path not configured.")?;
    let pos = config.installed_mods.iter().position(|m| m.id == mod_id)
        .ok_or("Mod not found in installed list.")?;
    let mod_entry = config.installed_mods[pos].clone();
    installer::uninstall(&mod_entry, &sekiro_path)?;
    config.installed_mods.remove(pos);
    state::save(&config)?;
    Ok(())
}

#[tauri::command]
pub fn toggle_mod(mod_id: String, enabled: bool, state: State<'_, AppState>) -> Result<(), String> {
    let mut config = state.0.lock().unwrap();
    let sekiro_path = config.sekiro_path.clone()
        .ok_or("Sekiro path not configured.")?;
    let mod_entry_clone = {
        let m = config.installed_mods.iter().find(|m| m.id == mod_id)
            .ok_or("Mod not found.")?;
        m.clone()
    };
    installer::toggle(&mod_entry_clone, enabled, &sekiro_path)?;
    if let Some(m) = config.installed_mods.iter_mut().find(|m| m.id == mod_id) {
        m.enabled = enabled;
    }
    state::save(&config)?;
    Ok(())
}

#[tauri::command]
pub fn check_conflicts(state: State<'_, AppState>) -> Vec<Conflict> {
    conflicts::check(&state.0.lock().unwrap())
}

#[tauri::command]
pub fn install_mod_engine(archive_path: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut config = state.0.lock().unwrap();
    let sekiro_path = config.sekiro_path.clone()
        .ok_or("Sekiro path not configured.")?;
    installer::install_mod_engine(&archive_path, &sekiro_path)?;
    config.mod_engine_installed = true;
    state::save(&config)?;
    Ok(())
}

#[tauri::command]
pub fn backup_save(label: Option<String>, state: State<'_, AppState>) -> Result<SaveBackup, String> {
    let mut config = state.0.lock().unwrap();
    let backup = saves::backup(label)?;
    config.save_backups.push(backup.clone());
    state::save(&config)?;
    Ok(backup)
}

#[tauri::command]
pub fn restore_save(backup_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let config = state.0.lock().unwrap();
    let backup = config.save_backups.iter()
        .find(|b| b.id == backup_id)
        .ok_or("Backup not found.")?
        .clone();
    saves::restore(&backup)
}

#[tauri::command]
pub fn import_save(save_path: String) -> Result<(), String> {
    saves::import(&save_path)
}

#[tauri::command]
pub fn list_save_backups(state: State<'_, AppState>) -> Vec<SaveBackup> {
    state.0.lock().unwrap().save_backups.clone()
}
