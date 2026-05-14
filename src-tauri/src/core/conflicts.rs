use crate::core::types::{AppConfig, Conflict, ConflictSeverity};

pub fn check(config: &AppConfig) -> Vec<Conflict> {
    let mut conflicts = Vec::new();
    let active: Vec<&crate::core::types::InstalledMod> = config.installed_mods
        .iter()
        .filter(|m| m.enabled)
        .collect();

    for (category, rule) in &config.conflict_rules {
        let matching: Vec<&str> = active.iter()
            .filter(|m| {
                rule.members.iter().any(|member| {
                    m.id.to_lowercase().contains(member) ||
                    m.name.to_lowercase().contains(member)
                })
            })
            .map(|m| m.name.as_str())
            .collect();

        if matching.len() > rule.max_active {
            conflicts.push(Conflict {
                severity: ConflictSeverity::Warning,
                message: format!(
                    "Multiple {} mods active: {}. Only {} should be enabled at once.",
                    category,
                    matching.join(", "),
                    rule.max_active
                ),
                mod_ids: matching.iter().map(|s| s.to_string()).collect(),
            });
        }
    }

    conflicts
}
