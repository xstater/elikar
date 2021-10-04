use crate::render::vulkan::core::{Core, AshRaw, DeviceMemory, CommandBuffers};
use std::sync::Arc;
use ash::vk;

pub struct Buffer {
    pub(in crate::render) core : Arc<Core>,
    pub(in crate::render) buffer : vk::Buffer,
    pub(in crate::render) size : vk::DeviceSize,
    pub(in crate::render) memory : Option<DeviceMemory>
}

impl Buffer{
    pub fn builder() -> BufferBuilder{
        BufferBuilder{
            size: 0,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            usage: vk::BufferUsageFlags::TRANSFER_SRC,
            queue_family_indices: vec![]
        }
    }

    pub fn memory_requirements(&self) -> vk::MemoryRequirements {
        unsafe {
            self.core.device.get_buffer_memory_requirements(self.buffer)
        }
    }

    pub fn bind_memory(&mut self,memory : DeviceMemory) -> Result<(),vk::Result>{
        debug_assert!(self.memory.is_none(),
            "Bind memory to buffer failed! buffer has already been bound to a memory");
        self.memory.replace(memory);
        unsafe {
            self.core.device.bind_buffer_memory(
                self.buffer,
                *self.memory.as_ref().unwrap().raw(),
                0)
        }
    }

    pub fn copy_from_buffer(&mut self, source: &Buffer) -> Result<(),vk::Result> {
        debug_assert!(self.has_memory() | source.has_memory(),
                      "Cannot copy data from buffer! buffer has not bound any memory.");
        debug_assert!(self.size >= source.size,
                      "Cannot copy data from buffer! buffer is not larger than source buffer! self:{},source:{}",
            self.size,source.size);
        let command_buffers = CommandBuffers::allocate(self.core.clone(),1)?;
        let command_buffer = command_buffers.command_buffers[0];
        let begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe {
            self.core.device.begin_command_buffer(command_buffer,&begin_info)?;
            let copy = [vk::BufferCopy{
                src_offset: 0,
                dst_offset: 0,
                size: source.size
            }];
            self.core.device.cmd_copy_buffer(command_buffer, source.buffer, self.buffer, &copy);
            self.core.device.end_command_buffer(command_buffer)?;
        }

        // submit
        let submit_info = [vk::SubmitInfo::builder()
            .command_buffers(command_buffers.raw())
            .build()];
        unsafe {
            self.core.device.queue_submit(self.core.transfer_queue,&submit_info,vk::Fence::null())?;
            self.core.device.queue_wait_idle(self.core.transfer_queue)?;
        }
        Ok(())
    }

    pub fn size(&self) -> vk::DeviceSize {
        self.size
    }

    pub fn has_memory(&self) -> bool {
        self.memory.is_some()
    }

    pub fn memory(&self) -> Option<&DeviceMemory> {
        self.memory.as_ref()
    }

    pub fn memory_mut(&mut self) -> Option<&mut DeviceMemory> {
        self.memory.as_mut()
    }
}

impl AshRaw for Buffer {
    type Raw = vk::Buffer;

    fn raw(&self) -> &Self::Raw {
        &self.buffer
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.core.device.destroy_buffer(self.buffer,Option::None)
        }
    }
}

pub struct BufferBuilder{
    size : vk::DeviceSize,
    sharing_mode : vk::SharingMode,
    usage : vk::BufferUsageFlags,
    queue_family_indices : Vec<u32>
}

impl BufferBuilder {
    pub fn size(mut self,size : usize) -> Self {
        self.size = size as _;
        self
    }

    pub fn sharing_mode(mut self,mode : vk::SharingMode) -> Self {
        self.sharing_mode = mode;
        self
    }

    pub fn usage(mut self,usage : vk::BufferUsageFlags) -> Self {
        self.usage = usage;
        self
    }

    pub fn queue_family_index(mut self,index : u32) -> Self {
        self.queue_family_indices.push(index);
        self
    }

    pub(in crate::render) fn build(self,core : Arc<Core>) -> Result<Buffer,vk::Result> {
        let info = vk::BufferCreateInfo::builder()
            .usage(self.usage)
            .sharing_mode(self.sharing_mode)
            .size(self.size)
            .queue_family_indices(&self.queue_family_indices);
        let buffer = unsafe {
            core.device.create_buffer(&info, Option::None)
        }?;
        Ok(Buffer{
            core,
            buffer,
            size: self.size,
            memory: Option::None
        })
    }
}