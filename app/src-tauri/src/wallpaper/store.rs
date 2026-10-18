use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use super::profile::WallpaperProfile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperStore {
    #[serde(default = "default_store_version")]
    pub version: u32,
    pub active_profile_id: String,
    pub profiles: HashMap<String, WallpaperProfile>,
    pub monitor_assignments: HashMap<String, String>, // monitor_name -> profile_id
}

fn default_store_version() -> u32 {
    1
}

impl Default for WallpaperStore {
    fn default() -> Self {
        let mut profiles = HashMap::new();
        let default_profile = WallpaperProfile::new("default", "Default Profile");
        profiles.insert("default".to_string(), default_profile);

        Self {
            version: 1,
            active_profile_id: "default".to_string(),
            profiles,
            monitor_assignments: HashMap::new(),
        }
    }
}

impl WallpaperStore {
    pub fn load_or_default<P: AsRef<Path>>(file_path: P) -> Self {
        if let Ok(content) = fs::read_to_string(file_path.as_ref()) {
            if let Ok(mut store) = serde_json::from_str::<WallpaperStore>(&content) {
                store.migrate();
                return store;
            }
        }
        Self::default()
    }

    pub fn save<P: AsRef<Path>>(&self, file_path: P) -> Result<(), String> {
        if let Some(parent) = file_path.as_ref().parent() {
            let _ = fs::create_dir_all(parent);
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize wallpaper store: {e}"))?;
        fs::write(file_path, json)
            .map_err(|e| format!("Failed to save wallpaper store file: {e}"))
    }

    pub fn migrate(&mut self) {
        if self.version < 1 {
            self.version = 1;
        }
        if !self.profiles.contains_key(&self.active_profile_id) {
            let default_prof = WallpaperProfile::new("default", "Default Profile");
            self.profiles.insert("default".to_string(), default_prof);
            self.active_profile_id = "default".to_string();
        }
    }

    pub fn active_profile(&self) -> Option<&WallpaperProfile> {
        self.profiles.get(&self.active_profile_id)
    }

    pub fn active_profile_mut(&mut self) -> Option<&mut WallpaperProfile> {
        self.profiles.get_mut(&self.active_profile_id)
    }

    pub fn assign_profile_to_monitor(&mut self, monitor: &str, profile_id: &str) -> Result<(), String> {
        if !self.profiles.contains_key(profile_id) {
            return Err(format!("Profile '{profile_id}' does not exist"));
        }
        self.monitor_assignments.insert(monitor.to_string(), profile_id.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_defaults_and_serialization() {
        let mut store = WallpaperStore::default();
        assert_eq!(store.version, 1);
        assert!(store.active_profile().is_some());

        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("wallpaper_store.json");

        store.assign_profile_to_monitor("primary", "default").unwrap();
        store.save(&path).unwrap();

        let loaded = WallpaperStore::load_or_default(&path);
        assert_eq!(loaded.active_profile_id, "default");
        assert_eq!(loaded.monitor_assignments.get("primary"), Some(&"default".to_string()));
    }
}
