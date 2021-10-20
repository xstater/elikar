use crate::render::vulkan::core::{AshRaw, Core};
use ash::vk;
use std::sync::Arc;

pub struct CommandBuffers {
    pub(in crate::render) core: Arc<Core>,
    pub(in crate::render) command_buffers: Vec<vk::CommandBuffer>,
}

impl CommandBuffers {
    pub(in crate::render) fn allocate(
        core: Arc<Core>,
        count: usize,
    ) -> Result<CommandBuffers, vk::Result> {
        let info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(core.command_pool)
            .command_buffer_count(count as _)
            .level(vk::CommandBufferLevel::PRIMARY);
        let command_buffers = unsafe { core.device.allocate_command_buffers(&info) }?;
        Ok(CommandBuffers {
            core,
            command_buffers,
        })
    }

    pub fn reset(&mut self,index: usize) -> Result<(),vk::Result> {
        unsafe{
            self.core.device
                .reset_command_buffer(
                    self.command_buffers[index],
                    vk::CommandBufferResetFlags::all())
        }
    }

    pub fn record<F>(&mut self,index: usize,cmds : F) -> Result<(),vk::Result> 
        where F: FnOnce(&ash::Device,vk::CommandBuffer) -> Result<(),vk::Result> {
        let command_buffer = self.command_buffers[index];
        let begin_info = vk::CommandBufferBeginInfo::default();
        unsafe{ self.core.device.begin_command_buffer(command_buffer, &begin_info)?; }
        cmds(&self.core.device,command_buffer)?;
        unsafe{ self.core.device.end_command_buffer(command_buffer)?; }
        Ok(())
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
            self.core
                .device
                .free_command_buffers(self.core.command_pool, &self.command_buffers)
        }
    }
}
