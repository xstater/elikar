use crate::sysinfo::cpu_info::CPUInfo;
use crate::sysinfo::platform_info::PlatformInfo;
use crate::sysinfo::video_info::VideoInfo;

pub mod cpu_info;
pub mod platform_info;
pub mod video_info;

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

