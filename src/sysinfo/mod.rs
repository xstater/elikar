mod cpu_info;
mod platform_info;
mod video_info;

pub use cpu_info::CPUInfo;
pub use platform_info::PlatformInfo;
pub use video_info::{
    DPI,
    DisplayMode,
    Screen,
    VideoInfo
};

#[derive(Debug)]
pub struct SystemInfo {
    cpu: CPUInfo,
    platform: PlatformInfo,
    video: VideoInfo,
}

impl SystemInfo {
    pub(in crate) fn new() -> SystemInfo {
        SystemInfo {
            cpu: CPUInfo::new(),
            platform: PlatformInfo::new(),
            video: VideoInfo::new(),
        }
    }
    pub fn cpu(&self) -> &CPUInfo {
        &self.cpu
    }
    pub fn platform(&self) -> &PlatformInfo {
        &self.platform
    }
    pub fn video(&self) -> &VideoInfo {
        &self.video
    }
}

