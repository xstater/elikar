use std::sync::Arc;
use crate::render::vulkan::core::{Core, AshRaw};
use ash::vk;

pub struct CommandBuffers {
    pub(in crate::render) core : Arc<Core>,
    pub(in crate::render) command_buffers: Vec<vk::CommandBuffer>
}

impl CommandBuffers {
    pub(in crate::render) fn allocate(core : Arc<Core>,count : usize) -> Result<CommandBuffers,vk::Result> {
        let info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(core.command_pool)
            .command_buffer_count(count as _)
            .level(vk::CommandBufferLevel::PRIMARY);
        let command_buffers = unsafe {
            core.device.allocate_command_buffers(&info)
        }?;
        Ok(CommandBuffers{
            core,
            command_buffers
        })
    }
}

impl AshRaw for CommandBuffers {
    type Raw = [vk::CommandBuffer];

    fn raw(&self) -> &Self::Raw {
        &self.command_buffers
    }
}

impl Drop for CommandBuffers {
    fn drop(&mut self) {
        unsafe {
            self.core.device.free_command_buffers(self.core.command_pool,&self.command_buffers)
        }
    }
}
