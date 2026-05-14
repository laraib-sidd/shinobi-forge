use shinobi_forge::core::types::AppConfig;

#[test]
fn round_trip_default_config() {
    let config = AppConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    let parsed: AppConfig = serde_json::from_str(&json).unwrap();
    assert!(parsed.sekiro_path.is_none());
    assert!(!parsed.mod_engine_installed);
    assert!(parsed.installed_mods.is_empty());
    assert!(parsed.conflict_rules.contains_key("overhaul"));
}

#[test]
fn default_conflict_rules_are_correct() {
    let config = AppConfig::default();
    let rule = config.conflict_rules.get("overhaul").unwrap();
    assert_eq!(rule.max_active, 1);
    assert!(rule.members.contains(&"resurrection".to_string()));
    assert!(rule.members.contains(&"lmtsr".to_string()));
}
