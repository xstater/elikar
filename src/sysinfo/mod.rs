mod video_info;
mod cpu_info;
mod platform_info;

use crate::sysinfo::video_info::VideoInfo;
use crate::sysinfo::cpu_info::CPUInfo;
use crate::sysinfo::platform_info::PlatformInfo;

pub struct SystemInfo{
    video_info : VideoInfo,
    cpu_info : CPUInfo,
    platform_info : PlatformInfo
}

impl SystemInfo {
    pub fn new() -> SystemInfo{
        SystemInfo{
            video_info : VideoInfo{},
            cpu_info : CPUInfo{},
            platform_info : PlatformInfo{}
        }
    }

    pub fn video_info(&self) -> &VideoInfo{
        &self.video_info
    }

    pub fn cpu_info(&self) -> &CPUInfo{
        &self.cpu_info
    }

    pub fn platform_info(&self) -> &PlatformInfo{
        &self.platform_info
    }

}
