use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::path::Path;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Settings {
    #[serde(default)]
    pub colors: SectionColors,
    #[serde(default)]
    pub lyrics_font: String,
}

impl Settings {
    /// Load settings from a settings.yml file in the given directory.
    /// If not found, searches ancestor directories recursively.
    /// Returns an error if the file doesn't exist in any ancestor or fails to load or parse.
    pub fn load(srcdir: &Path) -> Result<Self, String> {
        let path = find_settings_file(srcdir)?;
        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("failed to read settings.yml: {e}"))?;
        serde_yaml::from_str(&content).map_err(|e| format!("failed to parse settings.yml: {e}"))
    }
}

/// Find settings.yml in the given directory or ancestor directories (recursively).
fn find_settings_file(srcdir: &Path) -> Result<std::path::PathBuf, String> {
    let mut current = Some(srcdir);

    while let Some(dir) = current {
        let path = dir.join("settings.yml");
        if path.exists() && path.is_file() {
            return Ok(path);
        }
        current = dir.parent();
    }

    Err(format!(
        "settings.yml not found in {} or any ancestor directory",
        srcdir.display()
    ))
}

/// Maps section types to their default colors
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SectionColors {
    #[serde(flatten)]
    pub colors: HashMap<String, String>,
}

impl fmt::Display for SectionColors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pairs: Vec<String> = self
            .colors
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect();
        write!(f, "SectionColors {{ {} }}", pairs.join(", "))
    }
}

impl SectionColors {
    /// Load section colors from a settings.yml file in the given directory.
    /// If not found, searches ancestor directories recursively.
    /// Returns an error if the file doesn't exist in any ancestor or fails to load or parse.
    pub fn load(srcdir: &Path) -> Result<Self, String> {
        let path = find_settings_file(srcdir)?;
        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("failed to read settings.yml: {e}"))?;
        serde_yaml::from_str(&content).map_err(|e| format!("failed to parse settings.yml: {e}"))
    }

    /// Get the color for a section type.
    /// If override is provided, returns it. Otherwise returns the color from the map.
    /// Returns an error if no color is found.
    pub fn color_of_section(
        &self,
        section_type: &str,
        r#override: Option<&str>,
    ) -> Result<String, String> {
        r#override
            .map(|s| s.to_string())
            .or_else(|| self.colors.get(section_type).cloned())
            .ok_or_else(|| format!("no color defined for section type '{section_type}'"))
    }
}
