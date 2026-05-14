use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ModType {
    Content,
    Dll,
    Hybrid,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ModCategory {
    Overhaul,
    Ai,
    Qol,
    Cosmetic,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConflictSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledMod {
    pub id: String,
    pub name: String,
    pub mod_type: ModType,
    pub category: ModCategory,
    pub enabled: bool,
    pub installed_at: String,
    pub files: Vec<String>,
    pub dll_name: Option<String>,
    pub source_archive: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveBackup {
    pub id: String,
    pub label: String,
    pub created_at: String,
    pub path: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    pub severity: ConflictSeverity,
    pub message: String,
    pub mod_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictRule {
    pub max_active: usize,
    pub members: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub sekiro_path: Option<String>,
    pub mod_engine_installed: bool,
    pub installed_mods: Vec<InstalledMod>,
    pub save_backups: Vec<SaveBackup>,
    pub conflict_rules: HashMap<String, ConflictRule>,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut rules = HashMap::new();
        rules.insert(
            "overhaul".to_string(),
            ConflictRule {
                max_active: 1,
                members: vec![
                    "resurrection".to_string(),
                    "lmtsr".to_string(),
                    "dream-of-the-damned".to_string(),
                ],
            },
        );
        Self {
            sekiro_path: None,
            mod_engine_installed: false,
            installed_mods: vec![],
            save_backups: vec![],
            conflict_rules: rules,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInfo {
    pub path: String,
    pub mod_engine_installed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStatus {
    pub sekiro_path: Option<String>,
    pub mod_engine_installed: bool,
    pub active_mod_count: usize,
    pub total_mod_count: usize,
    pub save_backup_count: usize,
    pub conflicts: Vec<Conflict>,
    pub first_run: bool,
}
