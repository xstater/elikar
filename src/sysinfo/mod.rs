mod video_info;
mod cpu_info;
mod platform_info;

use crate::sysinfo::video_info::VideoInfo;
use crate::sysinfo::cpu_info::CPUInfo;
use crate::sysinfo::platform_info::PlatformInfo;
use crate::Elikar;

pub struct SystemInfo{}

impl SystemInfo {
    pub fn new(_ : &Elikar) -> SystemInfo{
        SystemInfo{}
    }

    pub fn video_info(&self) -> VideoInfo{
        VideoInfo{}
    }

    pub fn cpu_info(&self) -> CPUInfo{
        CPUInfo{}
    }

    pub fn platform_info(&self) -> PlatformInfo{
        PlatformInfo{}
    }

}
