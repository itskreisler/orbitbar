use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub fps_current: u32,
    pub fps_limit: u32,
    pub cpu_usage_percent: f32,
    pub memory_mb: f64,
    pub gpu_usage_percent: f32,
    pub frame_time_ms: f32,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            fps_current: 60,
            fps_limit: 60,
            cpu_usage_percent: 1.5,
            memory_mb: 45.2,
            gpu_usage_percent: 3.0,
            frame_time_ms: 16.6,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaController {
    pub status: PlaybackStatus,
    pub volume: f32,
    pub muted: bool,
    pub loop_playback: bool,
    pub metrics: PerformanceMetrics,
    pub error_message: Option<String>,
}

impl Default for MediaController {
    fn default() -> Self {
        Self {
            status: PlaybackStatus::Playing,
            volume: 1.0,
            muted: false,
            loop_playback: true,
            metrics: PerformanceMetrics::default(),
            error_message: None,
        }
    }
}

impl MediaController {
    pub fn pause(&mut self) {
        self.status = PlaybackStatus::Paused;
    }

    pub fn resume(&mut self) {
        self.status = PlaybackStatus::Playing;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_fps_limit(&mut self, limit: u32) {
        let clamped = limit.clamp(1, 144);
        self.metrics.fps_limit = clamped;
        self.metrics.fps_current = clamped;
        self.metrics.frame_time_ms = 1000.0 / (clamped as f32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn media_controller_state_and_fps() {
        let mut ctrl = MediaController::default();
        assert_eq!(ctrl.status, PlaybackStatus::Playing);

        ctrl.pause();
        assert_eq!(ctrl.status, PlaybackStatus::Paused);

        ctrl.set_fps_limit(30);
        assert_eq!(ctrl.metrics.fps_limit, 30);
        assert!((ctrl.metrics.frame_time_ms - 33.33).abs() < 0.1);
    }
}
