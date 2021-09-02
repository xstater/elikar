use crate::sysinfo::cpu_info::CPUInfo;
use crate::sysinfo::platform_info::PlatformInfo;
use crate::sysinfo::video_info::VideoInfo;

pub mod video_info;
pub mod cpu_info;
pub mod platform_info;

#[derive(Debug,Copy, Clone)]
pub struct SystemInfo;

impl SystemInfo {
    pub fn cpu(&self) -> CPUInfo{
        CPUInfo
    }

    pub fn platform(&self) -> PlatformInfo{
        PlatformInfo
    }
    pub fn video(&self) -> VideoInfo {
        VideoInfo
    }
}
