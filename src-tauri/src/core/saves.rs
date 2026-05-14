use crate::core::types::SaveBackup;
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};

const SEKIRO_SAVE_SUBDIR: &str = "Sekiro";

pub fn sekiro_save_dir() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(appdata).join(SEKIRO_SAVE_SUBDIR)
}

fn backup_base_dir() -> PathBuf {
    crate::core::state::app_data_dir().join("backups")
}

pub fn backup(label: Option<String>) -> Result<SaveBackup, String> {
    backup_from(&sekiro_save_dir(), &backup_base_dir(), label)
}

pub fn backup_from(save_dir: &Path, base: &Path, label: Option<String>) -> Result<SaveBackup, String> {
    if !save_dir.exists() {
        return Err("Sekiro save directory not found. Launch Sekiro once to create it.".to_string());
    }
    let now = Utc::now();
    let id = format!("backup-{}", now.format("%Y-%m-%d-%H%M%S"));
    let dest = base.join(&id);

    fs::create_dir_all(&dest).map_err(|e| e.to_string())?;
    copy_dir_all(save_dir, &dest)?;

    let size_bytes = dir_size(&dest);
    Ok(SaveBackup {
        id,
        label: label.unwrap_or_else(|| now.format("%Y-%m-%d %H:%M").to_string()),
        created_at: now.to_rfc3339(),
        path: dest.to_string_lossy().to_string(),
        size_bytes,
    })
}

pub fn restore(backup: &SaveBackup) -> Result<(), String> {
    restore_to(backup, &sekiro_save_dir())
}

pub fn restore_to(backup: &SaveBackup, save_dir: &Path) -> Result<(), String> {
    let backup_path = PathBuf::from(&backup.path);
    if !backup_path.exists() {
        return Err("Backup directory not found. It may have been deleted.".to_string());
    }
    if save_dir.exists() {
        fs::remove_dir_all(save_dir).map_err(|e| e.to_string())?;
    }
    fs::create_dir_all(save_dir).map_err(|e| e.to_string())?;
    copy_dir_all(&backup_path, save_dir)
}

pub fn import(save_path: &str) -> Result<(), String> {
    let src = PathBuf::from(save_path);
    let save_dir = sekiro_save_dir();
    fs::create_dir_all(&save_dir).map_err(|e| e.to_string())?;
    let filename = src.file_name().ok_or("Invalid save file path")?;
    fs::copy(&src, save_dir.join(filename)).map_err(|e| e.to_string())?;
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), String> {
    fs::create_dir_all(dst).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let ty = entry.file_type().map_err(|e| e.to_string())?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name())).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn dir_size(path: &Path) -> u64 {
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.metadata().ok())
        .filter(|m| m.is_file())
        .map(|m| m.len())
        .sum()
}
