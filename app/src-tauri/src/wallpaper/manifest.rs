use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use super::properties::{PropertyDefinition, PropertyValue};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum WallpaperType {
    #[default]
    Web,
    Video,
    Image,
    Application,
    Scene,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ContentRating {
    #[default]
    Everyone,
    Questionable,
    Mature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperManifest {
    #[serde(default = "default_version")]
    pub schema_version: u32,

    pub title: String,

    #[serde(default)]
    pub description: String,

    #[serde(rename = "type", default)]
    pub type_: WallpaperType,

    /// Main file entrypoint (e.g. "index.html" or "video.mp4" or "wallpaper.jpg")
    pub file: String,

    #[serde(default)]
    pub preview: Option<String>,

    #[serde(default)]
    pub author: String,

    #[serde(default)]
    pub version: String,

    #[serde(default)]
    pub rating: ContentRating,

    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    pub properties: HashMap<String, PropertyDefinition>,

    #[serde(default)]
    pub general: HashMap<String, serde_json::Value>,
}

fn default_version() -> u32 {
    1
}

impl WallpaperManifest {
    pub fn new(title: &str, file: &str, type_: WallpaperType) -> Self {
        Self {
            schema_version: 1,
            title: title.to_string(),
            description: String::new(),
            type_,
            file: file.to_string(),
            preview: None,
            author: String::new(),
            version: "1.0.0".to_string(),
            rating: ContentRating::Everyone,
            tags: Vec::new(),
            properties: HashMap::new(),
            general: HashMap::new(),
        }
    }

    pub fn from_json_str(json: &str) -> Result<Self, String> {
        let mut manifest: WallpaperManifest =
            serde_json::from_str(json).map_err(|e| format!("Failed to parse manifest: {e}"))?;
        manifest.migrate_and_validate()?;
        Ok(manifest)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = std::fs::read_to_string(path.as_ref())
            .map_err(|e| format!("Failed to read manifest file: {e}"))?;
        Self::from_json_str(&content)
    }

    pub fn migrate_and_validate(&mut self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Manifest 'title' cannot be empty".to_string());
        }
        if self.file.trim().is_empty() {
            return Err("Manifest 'file' cannot be empty".to_string());
        }

        // Migration from schema v0/v1
        if self.schema_version < 1 {
            self.schema_version = 1;
        }

        // Validate property definitions
        for (key, prop) in &mut self.properties {
            prop.validate_definition(key)?;
        }

        Ok(())
    }

    /// Extract default property values for wallpaper initialization
    pub fn default_property_values(&self) -> HashMap<String, PropertyValue> {
        let mut map = HashMap::new();
        for (key, prop) in &self.properties {
            map.insert(key.clone(), prop.default_value());
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_manifest() {
        let json = r#"{
            "schema_version": 1,
            "title": "Interactive Starfield",
            "type": "web",
            "file": "index.html",
            "author": "Tester",
            "properties": {
                "star_count": {
                    "type": "slider",
                    "text": "Number of Stars",
                    "value": 200,
                    "min": 10,
                    "max": 1000,
                    "step": 10
                }
            }
        }"#;

        let manifest = WallpaperManifest::from_json_str(json).unwrap();
        assert_eq!(manifest.title, "Interactive Starfield");
        assert_eq!(manifest.type_, WallpaperType::Web);
        assert_eq!(manifest.properties.len(), 1);
        assert!(manifest.properties.contains_key("star_count"));
    }

    #[test]
    fn rejects_empty_title() {
        let json = r#"{
            "title": "",
            "file": "index.html"
        }"#;
        assert!(WallpaperManifest::from_json_str(json).is_err());
    }
}
