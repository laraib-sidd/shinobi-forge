use std::fs;
use tempfile::TempDir;

#[test]
fn validate_valid_path() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("sekiro.exe"), b"").unwrap();
    assert!(shinobi_forge::core::detect::validate_sekiro_path(
        dir.path().to_str().unwrap()
    ));
}

#[test]
fn validate_missing_exe() {
    let dir = TempDir::new().unwrap();
    assert!(!shinobi_forge::core::detect::validate_sekiro_path(
        dir.path().to_str().unwrap()
    ));
}

#[test]
fn check_mod_engine_present() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("dinput8.dll"), b"").unwrap();
    fs::write(dir.path().join("modengine.ini"), b"[modengine]\n").unwrap();
    assert!(shinobi_forge::core::detect::check_mod_engine(
        dir.path().to_str().unwrap()
    ));
}

#[test]
fn check_mod_engine_missing() {
    let dir = TempDir::new().unwrap();
    assert!(!shinobi_forge::core::detect::check_mod_engine(
        dir.path().to_str().unwrap()
    ));
}

#[test]
fn parse_library_paths_from_vdf() {
    let vdf = r#"
"libraryfolders"
{
    "0"
    {
        "path"    "C:\\Program Files (x86)\\Steam"
    }
    "1"
    {
        "path"    "D:\\SteamLibrary"
    }
}
"#;
    let paths = shinobi_forge::core::detect::parse_library_paths(vdf);
    assert_eq!(paths.len(), 2);
    assert!(paths[0].contains("Steam"));
    assert!(paths[1].contains("SteamLibrary"));
}
