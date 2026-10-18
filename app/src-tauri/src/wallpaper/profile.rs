use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::properties::PropertyValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperProfile {
    pub id: String,
    pub name: String,
    pub active_wallpaper_id: Option<String>,
    pub active_wallpaper_path: Option<String>,
    #[serde(default)]
    pub properties: HashMap<String, PropertyValue>,
    #[serde(default)]
    pub target_monitor: Option<String>,
    #[serde(default = "default_fps")]
    pub fps_limit: u32,
    #[serde(default = "default_volume")]
    pub volume: f32,
    #[serde(default)]
    pub paused: bool,
    #[serde(default = "default_true")]
    pub mute_on_focus_loss: bool,
}

fn default_fps() -> u32 {
    60
}

fn default_volume() -> f32 {
    1.0
}

fn default_true() -> bool {
    true
}

impl WallpaperProfile {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            active_wallpaper_id: None,
            active_wallpaper_path: None,
            properties: HashMap::new(),
            target_monitor: None,
            fps_limit: 60,
            volume: 1.0,
            paused: false,
            mute_on_focus_loss: true,
        }
    }

    pub fn set_property(&mut self, key: &str, value: PropertyValue) {
        self.properties.insert(key.to_string(), value);
    }

    pub fn get_property(&self, key: &str) -> Option<&PropertyValue> {
        self.properties.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_creation_and_properties() {
        let mut profile = WallpaperProfile::new("prof-1", "Default Desktop");
        assert_eq!(profile.fps_limit, 60);
        assert_eq!(profile.volume, 1.0);

        profile.set_property("star_count", PropertyValue::Number(500.0));
        assert_eq!(
            profile.get_property("star_count"),
            Some(&PropertyValue::Number(500.0))
        );
    }
}
