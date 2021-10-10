use crate::render::vulkan::core::{AshRaw, Core};
use ash::vk;
use std::sync::Arc;

pub struct Shader {
    core: Arc<Core>,
    shader_module: vk::ShaderModule,
}

impl Shader {
    pub(in crate::render) fn new(core: Arc<Core>, shader_module: vk::ShaderModule) -> Self {
        Shader {
            core,
            shader_module,
        }
    }
}

impl AshRaw for Shader {
    type Raw = vk::ShaderModule;

    fn raw(&self) -> &Self::Raw {
        &self.shader_module
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.core
                .device
                .destroy_shader_module(self.shader_module, Option::None)
        }
    }
}
