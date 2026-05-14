use std::fs;
use std::io::Write;
use tempfile::TempDir;
use zip::write::{FileOptions, ZipWriter};

fn make_content_zip(dir: &TempDir) -> String {
    let path = dir.path().join("TestMod_v1.0.zip");
    let file = fs::File::create(&path).unwrap();
    let mut zip = ZipWriter::new(file);
    let opts = FileOptions::<()>::default();
    zip.start_file("TestMod/param/enemy.param", opts).unwrap();
    zip.write_all(b"param data").unwrap();
    zip.finish().unwrap();
    path.to_str().unwrap().to_string()
}

fn make_dll_zip(dir: &TempDir) -> String {
    let path = dir.path().join("WeaponWheel_v1.2.zip");
    let file = fs::File::create(&path).unwrap();
    let mut zip = ZipWriter::new(file);
    let opts = FileOptions::<()>::default();
    zip.start_file("dinput8.dll", opts).unwrap();
    zip.write_all(b"dll bytes").unwrap();
    zip.start_file("WeaponWheelResources/config.txt", opts).unwrap();
    zip.write_all(b"config").unwrap();
    zip.finish().unwrap();
    path.to_str().unwrap().to_string()
}

#[test]
fn classify_content_mod() {
    let dir = TempDir::new().unwrap();
    let zip_path = make_content_zip(&dir);
    let result = shinobi_forge::core::installer::classify_archive(&zip_path).unwrap();
    assert_eq!(result, shinobi_forge::core::types::ModType::Content);
}

#[test]
fn classify_dll_mod() {
    let dir = TempDir::new().unwrap();
    let zip_path = make_dll_zip(&dir);
    let result = shinobi_forge::core::installer::classify_archive(&zip_path).unwrap();
    assert_eq!(result, shinobi_forge::core::types::ModType::Dll);
}

#[test]
fn derive_mod_name_strips_version() {
    assert_eq!(
        shinobi_forge::core::installer::derive_mod_name("Resurrection_v1.16.1.zip"),
        "Resurrection"
    );
    assert_eq!(
        shinobi_forge::core::installer::derive_mod_name("WeaponWheel_v1.2.0.rar"),
        "WeaponWheel"
    );
    assert_eq!(
        shinobi_forge::core::installer::derive_mod_name("SimpleMod.zip"),
        "SimpleMod"
    );
}

#[test]
fn rar_returns_error() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test.rar");
    fs::write(&path, b"Rar!").unwrap();
    let result = shinobi_forge::core::installer::classify_archive(path.to_str().unwrap());
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("RAR"));
}
