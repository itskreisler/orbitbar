use std::path::{Path, PathBuf};
use super::manifest::WallpaperManifest;
use super::media::MediaController;
use super::profile::WallpaperProfile;

pub struct WallpaperRuntimeInstance {
    pub manifest: WallpaperManifest,
    pub base_directory: PathBuf,
    pub profile: WallpaperProfile,
    pub media: MediaController,
}

impl WallpaperRuntimeInstance {
    pub fn load_from_dir<P: AsRef<Path>>(dir_path: P) -> Result<Self, String> {
        let base_dir = dir_path.as_ref().to_path_buf();
        if !base_dir.is_dir() {
            return Err(format!("Directory does not exist: {}", base_dir.display()));
        }

        // Look for project.json or wallpaper.json
        let manifest_path = if base_dir.join("project.json").exists() {
            base_dir.join("project.json")
        } else if base_dir.join("wallpaper.json").exists() {
            base_dir.join("wallpaper.json")
        } else {
            return Err(format!(
                "No project.json or wallpaper.json manifest found in {}",
                base_dir.display()
            ));
        };

        let manifest = WallpaperManifest::from_file(&manifest_path)?;

        // Ensure entrypoint file exists
        let entrypoint = base_dir.join(&manifest.file);
        if !entrypoint.exists() {
            return Err(format!(
                "Manifest entrypoint file does not exist: {}",
                entrypoint.display()
            ));
        }

        let mut profile = WallpaperProfile::new("instance_profile", &manifest.title);
        profile.active_wallpaper_path = Some(entrypoint.to_string_lossy().to_string());
        profile.properties = manifest.default_property_values();

        Ok(Self {
            manifest,
            base_directory: base_dir,
            profile,
            media: MediaController::default(),
        })
    }

    /// Path traversal security validation
    pub fn resolve_resource(&self, relative_path: &str) -> Result<PathBuf, String> {
        let path = Path::new(relative_path);
        if path.is_absolute() || path.components().any(|c| c == std::path::Component::ParentDir) {
            return Err("Access outside of wallpaper directory is forbidden".to_string());
        }

        let full_path = self.base_directory.join(path);
        if full_path.starts_with(&self.base_directory) {
            Ok(full_path)
        } else {
            Err("Resource path validation failed".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prevents_path_traversal() {
        let temp_dir = tempfile::tempdir().unwrap();
        let manifest_json = r#"{
            "title": "Test Wallpaper",
            "file": "index.html"
        }"#;

        std::fs::write(temp_dir.path().join("project.json"), manifest_json).unwrap();
        std::fs::write(temp_dir.path().join("index.html"), "<html></html>").unwrap();

        let instance = WallpaperRuntimeInstance::load_from_dir(temp_dir.path()).unwrap();
        assert!(instance.resolve_resource("style.css").is_ok());
        assert!(instance.resolve_resource("../etc/passwd").is_err());
        assert!(instance.resolve_resource("/etc/passwd").is_err());
    }
}
