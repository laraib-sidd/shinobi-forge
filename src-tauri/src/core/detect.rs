use std::path::PathBuf;
use std::fs;

const SEKIRO_EXE: &str = "sekiro.exe";

pub fn find_sekiro_path() -> Option<String> {
    if let Some(path) = find_via_steam() {
        return Some(path);
    }
    let fallbacks = [
        r"C:\Program Files (x86)\Steam\steamapps\common\Sekiro",
        r"C:\Program Files\Steam\steamapps\common\Sekiro",
        r"D:\Steam\steamapps\common\Sekiro",
        r"D:\SteamLibrary\steamapps\common\Sekiro",
    ];
    for p in &fallbacks {
        if PathBuf::from(p).join(SEKIRO_EXE).exists() {
            return Some(p.to_string());
        }
    }
    None
}

fn find_via_steam() -> Option<String> {
    let steam_base = find_steam_base()?;
    let vdf_path = steam_base.join("steamapps").join("libraryfolders.vdf");
    let content = fs::read_to_string(&vdf_path).ok()?;
    for lib in parse_library_paths(&content) {
        let candidate = PathBuf::from(&lib)
            .join("steamapps")
            .join("common")
            .join("Sekiro");
        if candidate.join(SEKIRO_EXE).exists() {
            return Some(candidate.to_string_lossy().to_string());
        }
    }
    None
}

fn find_steam_base() -> Option<PathBuf> {
    let prog_x86 = std::env::var("ProgramFiles(x86)").ok()?;
    let path = PathBuf::from(prog_x86).join("Steam");
    if path.exists() { Some(path) } else { None }
}

pub fn parse_library_paths(vdf: &str) -> Vec<String> {
    let mut paths = Vec::new();
    for line in vdf.lines() {
        let line = line.trim();
        if line.starts_with("\"path\"") {
            // Line format: "path"    "some/value"
            // Splitting by '"' yields: ["", "path", "    ", "some/value", ""]
            let parts: Vec<&str> = line.split('"').collect();
            if parts.len() >= 4 {
                paths.push(parts[3].replace("\\\\", "\\"));
            }
        }
    }
    paths
}

pub fn validate_sekiro_path(path: &str) -> bool {
    PathBuf::from(path).join(SEKIRO_EXE).exists()
}

pub fn check_mod_engine(sekiro_path: &str) -> bool {
    let root = PathBuf::from(sekiro_path);
    root.join("dinput8.dll").exists() && root.join("modengine.ini").exists()
}
