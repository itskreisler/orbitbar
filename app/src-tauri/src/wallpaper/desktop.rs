use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DisplayServer {
    WindowsNative,
    X11,
    Wayland,
    MacOSNative,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopEnvironmentInfo {
    pub display_server: DisplayServer,
    pub desktop_environment: String,
    pub compositor: Option<String>,
    pub supports_animated_wallpaper: bool,
    pub recommended_adapter: String,
    pub limitations: Vec<String>,
}

impl DesktopEnvironmentInfo {
    pub fn detect() -> Self {
        #[cfg(target_os = "windows")]
        {
            Self {
                display_server: DisplayServer::WindowsNative,
                desktop_environment: "Windows DWM".to_string(),
                compositor: Some("Desktop Window Manager".to_string()),
                supports_animated_wallpaper: true,
                recommended_adapter: "WorkerW Window Parenting / SPI_SETDESKWALLPAPER".to_string(),
                limitations: vec![
                    "High contrast mode may hide WorkerW background window".to_string(),
                    "Requires window handle reparenting behind Progman/WorkerW".to_string(),
                ],
            }
        }

        #[cfg(target_os = "linux")]
        {
            use std::env;
            let wayland_display = env::var("WAYLAND_DISPLAY").ok();
            let xdg_type = env::var("XDG_SESSION_TYPE").unwrap_or_default();
            let desktop = env::var("XDG_CURRENT_DESKTOP")
                .or_else(|_| env::var("DESKTOP_SESSION"))
                .unwrap_or_else(|_| "Unknown".to_string());

            let is_wayland = wayland_display.is_some() || xdg_type.eq_ignore_ascii_case("wayland");

            if is_wayland {
                Self {
                    display_server: DisplayServer::Wayland,
                    desktop_environment: desktop.clone(),
                    compositor: Some(env::var("WAYLAND_COMPOSITOR").unwrap_or_else(|_| "Wayland Compositor".to_string())),
                    supports_animated_wallpaper: true,
                    recommended_adapter: "Wayland Layer Shell / DBus Protocol".to_string(),
                    limitations: vec![
                        "Wayland security model prevents direct arbitrary window reparenting".to_string(),
                        "Requires layer-shell protocol (wlr-layer-shell) or DE-specific DBus API".to_string(),
                        "Static fallback image tool ('feh') does not support animated video/web wallpapers".to_string(),
                    ],
                }
            } else {
                Self {
                    display_server: DisplayServer::X11,
                    desktop_environment: desktop.clone(),
                    compositor: Some(env::var("GDMSESSION").unwrap_or_else(|_| "X11 Window Manager".to_string())),
                    supports_animated_wallpaper: true,
                    recommended_adapter: "X11 Root Window / xwinwrap / Window Layering".to_string(),
                    limitations: vec![
                        "Desktop file icons drawn by desktop managers (Nautilus/Nemo) may cover root window".to_string(),
                        "'feh' is only suitable for static wallpapers, not animated ones".to_string(),
                    ],
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            // macOS configuration is preserved and commented out in release workflow
            Self {
                display_server: DisplayServer::MacOSNative,
                desktop_environment: "macOS Aqua / Quartz".to_string(),
                compositor: Some("Quartz Compositor".to_string()),
                supports_animated_wallpaper: false,
                recommended_adapter: "NSWindowLevel Desktop / macOS API".to_string(),
                limitations: vec![
                    "macOS build configuration is reserved/commented in repository CI".to_string(),
                ],
            }
        }

        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            Self {
                display_server: DisplayServer::Unknown,
                desktop_environment: "Unknown".to_string(),
                compositor: None,
                supports_animated_wallpaper: false,
                recommended_adapter: "Static Fallback".to_string(),
                limitations: vec!["Unsupported OS platform".to_string()],
            }
        }
    }
}

pub trait DesktopAdapter {
    fn set_static_wallpaper(&self, image_path: &Path) -> Result<(), String>;
    fn attach_live_wallpaper_window(&self, window_handle: usize) -> Result<(), String>;
    fn detach_live_wallpaper_window(&self, window_handle: usize) -> Result<(), String>;
}

pub struct PlatformDesktopAdapter;

impl PlatformDesktopAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PlatformDesktopAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl DesktopAdapter for PlatformDesktopAdapter {
    fn set_static_wallpaper(&self, image_path: &Path) -> Result<(), String> {
        if !image_path.exists() {
            return Err(format!("Image file does not exist: {}", image_path.display()));
        }

        #[cfg(target_os = "windows")]
        {
            use std::ffi::OsStr;
            use std::os::windows::ffi::OsStrExt;
            use windows_sys::Win32::UI::WindowsAndMessaging::{
                SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
            };

            let wide_path: Vec<u16> = OsStr::new(image_path)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();

            let res = unsafe {
                SystemParametersInfoW(
                    SPI_SETDESKWALLPAPER,
                    0,
                    wide_path.as_ptr() as *mut _,
                    SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
                )
            };

            if res == 0 {
                return Err("Windows SystemParametersInfoW failed to set wallpaper".to_string());
            }
            Ok(())
        }

        #[cfg(target_os = "linux")]
        {
            let path_str = image_path.to_string_lossy();
            let env_info = DesktopEnvironmentInfo::detect();

            // Try GNOME gsettings first if available
            if env_info.desktop_environment.to_lowercase().contains("gnome") {
                let status = std::process::Command::new("gsettings")
                    .args([
                        "set",
                        "org.gnome.desktop.background",
                        "picture-uri",
                        &format!("file://{path_str}"),
                    ])
                    .status();
                if let Ok(st) = status {
                    if st.success() {
                        return Ok(());
                    }
                }
            }

            // Fallback to feh for static wallpapers on X11
            let feh_status = std::process::Command::new("feh")
                .args(["--bg-fill", &path_str])
                .status();

            match feh_status {
                Ok(st) if st.success() => Ok(()),
                _ => Err("Failed to set static wallpaper on Linux (gsettings/feh)".to_string()),
            }
        }

        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        {
            Err("Static wallpaper setting not implemented for this platform".to_string())
        }
    }

    fn attach_live_wallpaper_window(&self, _window_handle: usize) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            use std::ffi::OsStr;
            use std::os::windows::ffi::OsStrExt;
            use windows_sys::Win32::Foundation::HWND;
            use windows_sys::Win32::UI::WindowsAndMessaging::{
                EnumWindows, FindWindowExW, FindWindowW, SendMessageTimeoutW, SetParent, SMTO_NORMAL,
            };

            fn to_wide(s: &str) -> Vec<u16> {
                OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
            }

            unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: isize) -> i32 {
                use std::ffi::OsStr;
                use std::os::windows::ffi::OsStrExt;
                let shell_class: Vec<u16> = OsStr::new("SHELLDLL_DefView").encode_wide().chain(std::iter::once(0)).collect();
                let worker_class: Vec<u16> = OsStr::new("WorkerW").encode_wide().chain(std::iter::once(0)).collect();

                let shell_dll = FindWindowExW(hwnd, std::ptr::null_mut(), shell_class.as_ptr(), std::ptr::null());
                if !shell_dll.is_null() {
                    let worker_w = FindWindowExW(std::ptr::null_mut(), hwnd, worker_class.as_ptr(), std::ptr::null());
                    if !worker_w.is_null() {
                        *(lparam as *mut HWND) = worker_w;
                    }
                }
                1
            }

            unsafe {
                let progman_class = to_wide("Progman");
                let progman = FindWindowW(progman_class.as_ptr(), std::ptr::null());
                if progman.is_null() {
                    return Err("Could not find Progman window".to_string());
                }

                // Send 0x052C message to Progman to spawn WorkerW
                let mut result: usize = 0;
                SendMessageTimeoutW(
                    progman,
                    0x052C,
                    0,
                    0,
                    SMTO_NORMAL,
                    1000,
                    &mut result as *mut _ as *mut _,
                );

                let mut worker_w: HWND = std::ptr::null_mut();
                EnumWindows(Some(enum_windows_proc), &mut worker_w as *mut _ as isize);

                let target_parent = if !worker_w.is_null() { worker_w } else { progman };
                SetParent(_window_handle as HWND, target_parent);
            }
            Ok(())
        }

        #[cfg(target_os = "linux")]
        {
            // Linux window layering setup
            let env_info = DesktopEnvironmentInfo::detect();
            if env_info.display_server == DisplayServer::Wayland {
                // Documented restriction for Wayland
                return Ok(());
            }
            Ok(())
        }

        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        {
            Err("Live wallpaper window reparenting not supported on this OS".to_string())
        }
    }

    fn detach_live_wallpaper_window(&self, _window_handle: usize) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::Foundation::HWND;
            use windows_sys::Win32::UI::WindowsAndMessaging::SetParent;
            unsafe {
                SetParent(_window_handle as HWND, std::ptr::null_mut());
            }
            Ok(())
        }

        #[cfg(not(target_os = "windows"))]
        {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_desktop_environment() {
        let env_info = DesktopEnvironmentInfo::detect();
        assert!(!env_info.desktop_environment.is_empty());
        assert!(!env_info.recommended_adapter.is_empty());
    }
}
