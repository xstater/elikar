use crate::render::vulkan::core::{Core, AshRaw};
use std::sync::Arc;
use ash::vk;

pub struct Semaphore {
    pub(in crate::render) core : Arc<Core>,
    pub(in crate::render) semaphore : vk::Semaphore
}

impl Semaphore {
    pub(in crate::render) fn new(core : Arc<Core>) -> Result<Semaphore,vk::Result> {
        let info = vk::SemaphoreCreateInfo::default();
        let semaphore = unsafe {
            core.device.create_semaphore(&info,Option::None)
        }?;
        Ok(Semaphore{
            core,
            semaphore
        })
    }
}

impl AshRaw for Semaphore {
    type Raw = vk::Semaphore;

    fn raw(&self) -> &Self::Raw {
        &self.semaphore
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe {
            self.core.device.destroy_semaphore(self.semaphore,Option::None)
        }
    }
}