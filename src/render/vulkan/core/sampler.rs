use std::sync::Arc;
use crate::render::vulkan::core::{Core, AshRaw};
use ash::vk;

pub struct Sampler {
    pub(in crate::render) core : Arc<Core>,
    pub(in crate::render) sampler : vk::Sampler
}

impl Sampler{
    pub(in crate::render) fn build(core : Arc<Core>,info : vk::SamplerCreateInfo) -> Result<Sampler,vk::Result> {
        let sampler = unsafe {
            core.device.create_sampler(&info,Option::None)
        }?;
        Ok(Sampler{
            core,
            sampler
        })
    }
}

impl AshRaw for Sampler {
    type Raw = vk::Sampler;

    fn raw(&self) -> &Self::Raw {
        &self.sampler
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            self.core.device.destroy_sampler(self.sampler,Option::None)
        }
    }
}
