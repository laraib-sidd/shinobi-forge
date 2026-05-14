use shinobi_forge::core::types::*;
use std::collections::HashMap;

fn make_mod(id: &str, name: &str, category: ModCategory, enabled: bool) -> InstalledMod {
    InstalledMod {
        id: id.to_string(),
        name: name.to_string(),
        mod_type: ModType::Content,
        category,
        enabled,
        installed_at: "2026-01-01T00:00:00Z".to_string(),
        files: vec![],
        dll_name: None,
        source_archive: format!("{}.zip", id),
    }
}

fn config_with_mods(mods: Vec<InstalledMod>) -> AppConfig {
    let mut rules = HashMap::new();
    rules.insert("overhaul".to_string(), ConflictRule {
        max_active: 1,
        members: vec!["resurrection".to_string(), "lmtsr".to_string()],
    });
    AppConfig {
        sekiro_path: None,
        mod_engine_installed: false,
        installed_mods: mods,
        save_backups: vec![],
        conflict_rules: rules,
    }
}

#[test]
fn no_conflict_single_overhaul() {
    let config = config_with_mods(vec![
        make_mod("resurrection", "Sekiro Resurrection", ModCategory::Overhaul, true),
    ]);
    assert!(shinobi_forge::core::conflicts::check(&config).is_empty());
}

#[test]
fn conflict_two_overhauls_active() {
    let config = config_with_mods(vec![
        make_mod("resurrection", "Sekiro Resurrection", ModCategory::Overhaul, true),
        make_mod("lmtsr", "LMTSR", ModCategory::Overhaul, true),
    ]);
    let result = shinobi_forge::core::conflicts::check(&config);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].severity, ConflictSeverity::Warning);
}

#[test]
fn no_conflict_two_overhauls_one_disabled() {
    let config = config_with_mods(vec![
        make_mod("resurrection", "Sekiro Resurrection", ModCategory::Overhaul, true),
        make_mod("lmtsr", "LMTSR", ModCategory::Overhaul, false),
    ]);
    assert!(shinobi_forge::core::conflicts::check(&config).is_empty());
}
