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
    /// Returns an error if the file doesn't exist or fails to load or parse.
    pub fn load(srcdir: &Path) -> Result<Self, String> {
        let path = srcdir.join("settings.yml");
        if !path.exists() {
            return Err(format!("settings.yml not found in {}", srcdir.display()));
        }
        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("failed to read settings.yml: {}", e))?;
        serde_yaml::from_str(&content)
            .map_err(|e| format!("failed to parse settings.yml: {}", e))
    }
}

/// Maps section types to their default colors
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SectionColors {
    #[serde(flatten)]
    pub colors: HashMap<String, String>,
}

impl fmt::Display for SectionColors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pairs: Vec<String> = self.colors.iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect();
        write!(f, "SectionColors {{ {} }}", pairs.join(", "))
    }
}

impl SectionColors {
    /// Load section colors from a settings.yml file in the given directory.
    /// Returns an error if the file doesn't exist or fails to load or parse.
    pub fn load(srcdir: &Path) -> Result<Self, String> {
        let colors_path = srcdir.join("settings.yml");
        if !colors_path.exists() {
            return Err(format!("settings.yml not found in {}", srcdir.display()));
        }
        let content = std::fs::read_to_string(&colors_path)
            .map_err(|e| format!("failed to read settings.yml: {}", e))?;
        serde_yaml::from_str(&content)
            .map_err(|e| format!("failed to parse settings.yml: {}", e))
    }

    /// Get the color for a section type.
    /// If override is provided, returns it. Otherwise returns the color from the map.
    /// Returns an error if no color is found.
    pub fn color_of_section(&self, section_type: &str, r#override: Option<&str>) -> Result<String, String> {
        r#override
            .map(|s| s.to_string())
            .or_else(|| self.colors.get(section_type).cloned())
            .ok_or_else(|| format!("no color defined for section type '{}'", section_type))
    }
}
