use std::fs;
use std::path::PathBuf;

const CHAIN_PREFIX: &str = "external_dlls_";
const SECTION: &str = "[modengine]";

pub fn add_chain_entry(sekiro_path: &str, dll_name: &str) -> Result<(), String> {
    let ini_path = PathBuf::from(sekiro_path).join("modengine.ini");
    let content = fs::read_to_string(&ini_path).map_err(|e| e.to_string())?;
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    let next_idx = count_active_entries(&lines);
    let insert_pos = find_insert_pos(&lines);
    let entry = format!("{}{} = \"{}\"", CHAIN_PREFIX, next_idx, dll_name);
    lines.insert(insert_pos, entry);

    fs::write(&ini_path, lines.join("\n")).map_err(|e| e.to_string())
}

pub fn remove_chain_entry(sekiro_path: &str, dll_name: &str) -> Result<(), String> {
    let ini_path = PathBuf::from(sekiro_path).join("modengine.ini");
    let content = fs::read_to_string(&ini_path).map_err(|e| e.to_string())?;

    let mut remaining: Vec<String> = Vec::new();
    let mut chain_values: Vec<String> = Vec::new();
    let mut in_section = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == SECTION {
            in_section = true;
            remaining.push(line.to_string());
            continue;
        }
        if in_section && trimmed.starts_with('[') {
            in_section = false;
        }
        if in_section && trimmed.starts_with(CHAIN_PREFIX) && !trimmed.starts_with(';') {
            let val = extract_value(trimmed);
            if val != dll_name {
                chain_values.push(val.to_string());
            }
        } else {
            remaining.push(line.to_string());
        }
    }

    let insert_pos = find_insert_pos(&remaining);
    for (i, val) in chain_values.iter().enumerate() {
        remaining.insert(insert_pos + i, format!("{}{} = \"{}\"", CHAIN_PREFIX, i, val));
    }

    fs::write(&ini_path, remaining.join("\n")).map_err(|e| e.to_string())
}

pub fn disable_chain_entry(sekiro_path: &str, dll_name: &str) -> Result<(), String> {
    let ini_path = PathBuf::from(sekiro_path).join("modengine.ini");
    let content = fs::read_to_string(&ini_path).map_err(|e| e.to_string())?;
    let result = content
        .lines()
        .map(|line| {
            let t = line.trim();
            if t.starts_with(CHAIN_PREFIX) && extract_value(t) == dll_name {
                format!(";{}", line)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&ini_path, result).map_err(|e| e.to_string())
}

pub fn enable_chain_entry(sekiro_path: &str, dll_name: &str) -> Result<(), String> {
    let ini_path = PathBuf::from(sekiro_path).join("modengine.ini");
    let content = fs::read_to_string(&ini_path).map_err(|e| e.to_string())?;
    let result = content
        .lines()
        .map(|line| {
            let t = line.trim();
            if t.starts_with(';') {
                let uncommented = t.trim_start_matches(';').trim();
                if uncommented.starts_with(CHAIN_PREFIX) && extract_value(uncommented) == dll_name {
                    return uncommented.to_string();
                }
            }
            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&ini_path, result).map_err(|e| e.to_string())
}

fn extract_value(line: &str) -> &str {
    line.splitn(2, '=')
        .nth(1)
        .unwrap_or("")
        .trim()
        .trim_matches('"')
}

fn count_active_entries(lines: &[String]) -> usize {
    lines
        .iter()
        .filter(|l| {
            let t = l.trim();
            t.starts_with(CHAIN_PREFIX) && !t.starts_with(';')
        })
        .count()
}

fn find_insert_pos(lines: &[String]) -> usize {
    let mut last = None;
    let mut in_section = false;
    for (i, line) in lines.iter().enumerate() {
        let t = line.trim();
        if t == SECTION {
            in_section = true;
            continue;
        }
        if in_section && t.starts_with('[') {
            break;
        }
        if in_section && (t.starts_with(CHAIN_PREFIX) || (t.starts_with(';') && t.contains(CHAIN_PREFIX))) {
            last = Some(i);
        }
    }
    last.map(|p| p + 1).unwrap_or_else(|| {
        // Find end of [modengine] section
        let mut in_sec = false;
        for (i, line) in lines.iter().enumerate() {
            let t = line.trim();
            if t == SECTION {
                in_sec = true;
                continue;
            }
            if in_sec && t.starts_with('[') {
                return i;
            }
        }
        lines.len()
    })
}
