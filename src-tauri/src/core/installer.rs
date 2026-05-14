use crate::core::mod_engine;
use crate::core::types::{InstalledMod, ModCategory, ModType};
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

const CONTENT_EXTENSIONS: &[&str] = &[".dcx", ".msgbnd", ".param", ".lua", ".tpf", ".bnd"];
const DLL_ENTRY: &str = "dinput8.dll";

pub fn classify_archive(archive_path: &str) -> Result<ModType, String> {
    let ext = Path::new(archive_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if ext == "rar" {
        return Err(
            "RAR archives are not supported. Please re-download the mod as .zip or .7z."
                .to_string(),
        );
    }

    let entries = list_archive_entries(archive_path)?;
    let has_dll = entries.iter().any(|e| e.to_lowercase().ends_with(DLL_ENTRY));
    let has_content = entries
        .iter()
        .any(|e| CONTENT_EXTENSIONS.iter().any(|ext| e.to_lowercase().ends_with(ext)));

    Ok(match (has_dll, has_content) {
        (true, true) => ModType::Hybrid,
        (true, false) => ModType::Dll,
        (false, true) => ModType::Content,
        (false, false) => ModType::Unknown,
    })
}

pub fn derive_mod_name(filename: &str) -> String {
    let stem = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(filename);
    let re = regex_lite::Regex::new(r"[-_]v?\d[\d.]*$").unwrap();
    re.replace(stem, "").to_string()
}

pub fn install(archive_path: &str, sekiro_path: &str) -> Result<InstalledMod, String> {
    let mod_type = classify_archive(archive_path)?;
    let filename = Path::new(archive_path)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("unknown");
    let name = derive_mod_name(filename);
    let id = format!(
        "{}-{}",
        name.to_lowercase().replace(' ', "-"),
        Utc::now().timestamp()
    );

    let mut files: Vec<String> = Vec::new();
    let mut dll_name: Option<String> = None;

    match mod_type {
        ModType::Content => {
            let dest = PathBuf::from(sekiro_path).join("mods").join(&name);
            extract_to_dir(archive_path, &dest)?;
            files.push(format!("mods/{}/", name));
        }
        ModType::Dll => {
            let (dll, others) = extract_dll_mod(archive_path, sekiro_path, &name)?;
            dll_name = Some(dll.clone());
            files.push(dll.clone());
            files.extend(others);
            mod_engine::add_chain_entry(sekiro_path, &dll)?;
        }
        ModType::Hybrid => {
            let dest = PathBuf::from(sekiro_path).join("mods").join(&name);
            let dll = extract_hybrid(archive_path, sekiro_path, &dest, &name)?;
            dll_name = Some(dll.clone());
            files.push(format!("mods/{}/", name));
            files.push(dll.clone());
            mod_engine::add_chain_entry(sekiro_path, &dll)?;
        }
        ModType::Unknown => {
            let dest = PathBuf::from(sekiro_path).join("mods").join(&name);
            extract_to_dir(archive_path, &dest)?;
            files.push(format!("mods/{}/", name));
        }
    }

    Ok(InstalledMod {
        id,
        name,
        mod_type,
        category: infer_category(filename),
        enabled: true,
        installed_at: Utc::now().to_rfc3339(),
        files,
        dll_name,
        source_archive: filename.to_string(),
    })
}

pub fn uninstall(mod_entry: &InstalledMod, sekiro_path: &str) -> Result<(), String> {
    if let Some(dll) = &mod_entry.dll_name {
        mod_engine::remove_chain_entry(sekiro_path, dll)?;
        let dll_path = PathBuf::from(sekiro_path).join(dll);
        if dll_path.exists() {
            fs::remove_file(&dll_path).map_err(|e| e.to_string())?;
        }
    }
    for file in &mod_entry.files {
        let path = PathBuf::from(sekiro_path).join(file.trim_end_matches('/'));
        if path.is_dir() {
            fs::remove_dir_all(&path).map_err(|e| e.to_string())?;
        } else if path.is_file() {
            fs::remove_file(&path).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

pub fn toggle(mod_entry: &InstalledMod, enabled: bool, sekiro_path: &str) -> Result<(), String> {
    match &mod_entry.mod_type {
        ModType::Content | ModType::Unknown => {
            toggle_content_mod(&mod_entry.name, sekiro_path, enabled)
        }
        ModType::Dll => {
            if let Some(dll) = &mod_entry.dll_name {
                toggle_dll_mod(dll, sekiro_path, enabled)
            } else {
                Ok(())
            }
        }
        ModType::Hybrid => {
            toggle_content_mod(&mod_entry.name, sekiro_path, enabled)?;
            if let Some(dll) = &mod_entry.dll_name {
                toggle_dll_mod(dll, sekiro_path, enabled)?;
            }
            Ok(())
        }
    }
}

pub fn install_mod_engine(archive_path: &str, sekiro_path: &str) -> Result<(), String> {
    extract_to_dir(archive_path, Path::new(sekiro_path))?;
    let root = PathBuf::from(sekiro_path);
    if !root.join("dinput8.dll").exists() {
        return Err(
            "ModEngine extracted but dinput8.dll not found. Check the archive.".to_string(),
        );
    }
    if !root.join("modengine.ini").exists() {
        return Err(
            "ModEngine extracted but modengine.ini not found. Check the archive.".to_string(),
        );
    }
    Ok(())
}

fn list_archive_entries(archive_path: &str) -> Result<Vec<String>, String> {
    let ext = Path::new(archive_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if ext == "zip" {
        let file = fs::File::open(archive_path).map_err(|e| e.to_string())?;
        let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
        Ok((0..archive.len())
            .map(|i| archive.by_index(i).unwrap().name().to_string())
            .collect())
    } else if ext == "7z" {
        let mut reader =
            sevenz_rust::SevenZReader::open(archive_path, sevenz_rust::Password::empty())
                .map_err(|e| e.to_string())?;
        let entries = reader
            .archive()
            .files
            .iter()
            .map(|f| f.name().to_string())
            .collect();
        Ok(entries)
    } else {
        Err(format!(
            "Unsupported archive format: .{}. Shinobi Forge supports .zip and .7z.",
            ext
        ))
    }
}

fn extract_to_dir(archive_path: &str, dest: &Path) -> Result<(), String> {
    let ext = Path::new(archive_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    fs::create_dir_all(dest).map_err(|e| e.to_string())?;

    if ext == "zip" {
        let file = fs::File::open(archive_path).map_err(|e| e.to_string())?;
        let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
        archive.extract(dest).map_err(|e| e.to_string())
    } else if ext == "7z" {
        sevenz_rust::decompress_file(archive_path, dest).map_err(|e| e.to_string())
    } else {
        Err(format!("Unsupported archive format: .{}", ext))
    }
}

fn extract_dll_mod(
    archive_path: &str,
    sekiro_path: &str,
    mod_name: &str,
) -> Result<(String, Vec<String>), String> {
    let temp_dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    extract_to_dir(archive_path, temp_dir.path())?;

    let dll_dest_name = format!("{}.dll", mod_name);
    let dll_src = temp_dir.path().join(DLL_ENTRY);
    let dll_dest = PathBuf::from(sekiro_path).join(&dll_dest_name);
    fs::copy(&dll_src, &dll_dest).map_err(|e| e.to_string())?;

    let mut others = Vec::new();
    for entry in fs::read_dir(temp_dir.path()).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.file_name().to_str() == Some(DLL_ENTRY) {
            continue;
        }
        let dest = PathBuf::from(sekiro_path).join(entry.file_name());
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            copy_dir_all(entry.path(), &dest)?;
            others.push(format!("{}/", entry.file_name().to_string_lossy()));
        } else {
            fs::copy(entry.path(), &dest).map_err(|e| e.to_string())?;
            others.push(entry.file_name().to_string_lossy().to_string());
        }
    }

    Ok((dll_dest_name, others))
}

fn extract_hybrid(
    archive_path: &str,
    sekiro_path: &str,
    content_dest: &Path,
    mod_name: &str,
) -> Result<String, String> {
    let temp_dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    extract_to_dir(archive_path, temp_dir.path())?;

    let dll_dest_name = format!("{}.dll", mod_name);
    let dll_src = temp_dir.path().join(DLL_ENTRY);
    let dll_dest = PathBuf::from(sekiro_path).join(&dll_dest_name);
    fs::copy(&dll_src, &dll_dest).map_err(|e| e.to_string())?;

    fs::create_dir_all(content_dest).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(temp_dir.path()).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.file_name().to_str() == Some(DLL_ENTRY) {
            continue;
        }
        let dest = content_dest.join(entry.file_name());
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            copy_dir_all(entry.path(), &dest)?;
        } else {
            fs::copy(entry.path(), &dest).map_err(|e| e.to_string())?;
        }
    }

    Ok(dll_dest_name)
}

fn toggle_content_mod(mod_name: &str, sekiro_path: &str, enabled: bool) -> Result<(), String> {
    let enabled_path = PathBuf::from(sekiro_path).join("mods").join(mod_name);
    let disabled_dir = PathBuf::from(sekiro_path).join("mods").join(".disabled");
    let disabled_path = disabled_dir.join(mod_name);

    if enabled && disabled_path.exists() {
        fs::rename(&disabled_path, &enabled_path).map_err(|e| e.to_string())
    } else if !enabled && enabled_path.exists() {
        fs::create_dir_all(&disabled_dir).map_err(|e| e.to_string())?;
        fs::rename(&enabled_path, &disabled_path).map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}

fn toggle_dll_mod(dll_name: &str, sekiro_path: &str, enabled: bool) -> Result<(), String> {
    let enabled_path = PathBuf::from(sekiro_path).join(dll_name);
    let disabled_dir = PathBuf::from(sekiro_path).join(".disabled");
    let disabled_path = disabled_dir.join(dll_name);

    if enabled {
        if disabled_path.exists() {
            fs::copy(&disabled_path, &enabled_path).map_err(|e| e.to_string())?;
            fs::remove_file(&disabled_path).map_err(|e| e.to_string())?;
        }
        mod_engine::enable_chain_entry(sekiro_path, dll_name)
    } else {
        mod_engine::disable_chain_entry(sekiro_path, dll_name)?;
        if enabled_path.exists() {
            fs::create_dir_all(&disabled_dir).map_err(|e| e.to_string())?;
            fs::copy(&enabled_path, &disabled_path).map_err(|e| e.to_string())?;
            fs::remove_file(&enabled_path).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), String> {
    fs::create_dir_all(&dst).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(&src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let ty = entry.file_type().map_err(|e| e.to_string())?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn infer_category(filename: &str) -> ModCategory {
    let name = filename.to_lowercase();
    if name.contains("resurrection") || name.contains("lmtsr") || name.contains("damned") {
        ModCategory::Overhaul
    } else if name.contains("incarnation") || name.contains("spirit") {
        ModCategory::Ai
    } else if name.contains("wheel") || name.contains("hotkey") || name.contains("idol") {
        ModCategory::Qol
    } else if name.contains("art") || name.contains("skin") || name.contains("cosmetic") {
        ModCategory::Cosmetic
    } else {
        ModCategory::Unknown
    }
}
