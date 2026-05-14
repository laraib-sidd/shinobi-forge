use shinobi_forge::core::saves;
use std::fs;
use tempfile::TempDir;

#[test]
fn backup_copies_save_dir() {
    let save_dir = TempDir::new().unwrap();
    fs::write(save_dir.path().join("S0000.sl2"), b"save data").unwrap();

    let backup_base = TempDir::new().unwrap();
    let backup = saves::backup_from(
        save_dir.path(),
        backup_base.path(),
        None,
    ).unwrap();

    assert!(!backup.id.is_empty());
    assert!(backup.size_bytes > 0);
    let restored_file = std::path::PathBuf::from(&backup.path).join("S0000.sl2");
    assert!(restored_file.exists());
}

#[test]
fn restore_overwrites_save_dir() {
    let save_dir = TempDir::new().unwrap();
    fs::write(save_dir.path().join("S0000.sl2"), b"original").unwrap();

    let backup_base = TempDir::new().unwrap();
    let backup = saves::backup_from(
        save_dir.path(),
        backup_base.path(),
        None,
    ).unwrap();

    // Overwrite original
    fs::write(save_dir.path().join("S0000.sl2"), b"modified").unwrap();

    saves::restore_to(&backup, save_dir.path()).unwrap();
    let content = fs::read(save_dir.path().join("S0000.sl2")).unwrap();
    assert_eq!(content, b"original");
}
