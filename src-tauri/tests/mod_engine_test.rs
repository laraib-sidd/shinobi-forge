use std::fs;
use tempfile::TempDir;

fn write_ini(dir: &TempDir, content: &str) -> String {
    let path = dir.path().join("modengine.ini");
    fs::write(&path, content).unwrap();
    dir.path().to_str().unwrap().to_string()
}

#[test]
fn add_first_chain_entry() {
    let dir = TempDir::new().unwrap();
    let sekiro = write_ini(&dir, "[modengine]\ndebug = 0\n");
    shinobi_forge::core::mod_engine::add_chain_entry(&sekiro, "WeaponWheel.dll").unwrap();
    let content = fs::read_to_string(dir.path().join("modengine.ini")).unwrap();
    assert!(content.contains("WeaponWheel.dll"));
    assert!(content.contains("external_dlls_0"));
}

#[test]
fn add_second_chain_entry_uses_next_index() {
    let dir = TempDir::new().unwrap();
    let ini = "[modengine]\nexternal_dlls_0 = \"WeaponWheel.dll\"\n";
    let sekiro = write_ini(&dir, ini);
    shinobi_forge::core::mod_engine::add_chain_entry(&sekiro, "HotkeySystem.dll").unwrap();
    let content = fs::read_to_string(dir.path().join("modengine.ini")).unwrap();
    assert!(content.contains("external_dlls_1"));
    assert!(content.contains("HotkeySystem.dll"));
}

#[test]
fn disable_chain_entry_comments_out_line() {
    let dir = TempDir::new().unwrap();
    let ini = "[modengine]\nexternal_dlls_0 = \"WeaponWheel.dll\"\n";
    let sekiro = write_ini(&dir, ini);
    shinobi_forge::core::mod_engine::disable_chain_entry(&sekiro, "WeaponWheel.dll").unwrap();
    let content = fs::read_to_string(dir.path().join("modengine.ini")).unwrap();
    assert!(content.contains(";external_dlls_0"));
}

#[test]
fn enable_chain_entry_uncomments_line() {
    let dir = TempDir::new().unwrap();
    let ini = "[modengine]\n;external_dlls_0 = \"WeaponWheel.dll\"\n";
    let sekiro = write_ini(&dir, ini);
    shinobi_forge::core::mod_engine::enable_chain_entry(&sekiro, "WeaponWheel.dll").unwrap();
    let content = fs::read_to_string(dir.path().join("modengine.ini")).unwrap();
    assert!(!content.contains(";external_dlls_0"));
    assert!(content.contains("external_dlls_0"));
}

#[test]
fn remove_chain_entry_and_reindex() {
    let dir = TempDir::new().unwrap();
    let ini = "[modengine]\nexternal_dlls_0 = \"WeaponWheel.dll\"\nexternal_dlls_1 = \"HotkeySystem.dll\"\n";
    let sekiro = write_ini(&dir, ini);
    shinobi_forge::core::mod_engine::remove_chain_entry(&sekiro, "WeaponWheel.dll").unwrap();
    let content = fs::read_to_string(dir.path().join("modengine.ini")).unwrap();
    assert!(!content.contains("WeaponWheel.dll"));
    assert!(content.contains("external_dlls_0 = \"HotkeySystem.dll\""));
}
