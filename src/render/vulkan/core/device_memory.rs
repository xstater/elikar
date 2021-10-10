use crate::render::vulkan::core::{AshRaw, Core};
use ash::{util, vk};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

pub struct DeviceMemory {
    pub(in crate::render) core: Arc<Core>,
    pub(in crate::render) reqs: vk::MemoryRequirements,
    pub(in crate::render) device_memory: vk::DeviceMemory,
}

impl DeviceMemory {
    pub fn copy_from_slice<T: Sized + Copy>(&mut self, slice: &[T]) -> Result<(), vk::Result> {
        let ptr = unsafe {
            self.core.device.map_memory(
                self.device_memory,
                0,
                self.reqs.size,
                vk::MemoryMapFlags::empty(),
            )
        }?;
        let mut align =
            unsafe { util::Align::new(ptr, std::mem::align_of::<T>() as _, self.reqs.size) };
        align.copy_from_slice(slice);
        unsafe { self.core.device.unmap_memory(self.device_memory) };
        Ok(())
    }

    fn find_memory(
        memory_properties: &vk::PhysicalDeviceMemoryProperties,
        reqs: vk::MemoryRequirements,
        flags: vk::MemoryPropertyFlags,
    ) -> Option<usize> {
        memory_properties
            .memory_types
            .iter()
            .enumerate()
            .find(|(index, memory_type)| {
                (reqs.memory_type_bits & (1 << *index)) != 0
                    && (memory_type.property_flags & flags == flags)
            })
            .map(|(index, _)| index)
    }

    pub(in crate::render) fn allocate(
        core: Arc<Core>,
        reqs: vk::MemoryRequirements,
        flags: vk::MemoryPropertyFlags,
    ) -> Result<DeviceMemory, AllocateMemoryError> {
        let index = DeviceMemory::find_memory(&core.memory_properties, reqs, flags)
            .ok_or(AllocateMemoryError::FindMemoryFailed)?;
        let info = vk::MemoryAllocateInfo::builder()
            .allocation_size(reqs.size)
            .memory_type_index(index as _);
        let memory = unsafe { core.device.allocate_memory(&info, Option::None) }?;
        Ok(DeviceMemory {
            core,
            reqs,
            device_memory: memory,
        })
    }
}

impl AshRaw for DeviceMemory {
    type Raw = vk::DeviceMemory;

    fn raw(&self) -> &Self::Raw {
        &self.device_memory
    }
}

impl Drop for DeviceMemory {
    fn drop(&mut self) {
        unsafe {
            self.core
                .device
                .free_memory(self.device_memory, Option::None);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AllocateMemoryError {
    VulkanError(vk::Result),
    FindMemoryFailed,
}

impl From<vk::Result> for AllocateMemoryError {
    fn from(vk_err: vk::Result) -> Self {
        AllocateMemoryError::VulkanError(vk_err)
    }
}

impl Display for AllocateMemoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AllocateMemoryError::VulkanError(vk_err) => {
                write!(f, "Allocate memory error! vulkan error:{}", vk_err)
            }
            AllocateMemoryError::FindMemoryFailed => {
                write!(f, "Allocate memory error! Cannot find a memory")
            }
        }
    }
}

impl Error for AllocateMemoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AllocateMemoryError::VulkanError(vk_err) => Option::Some(vk_err),
            _ => Option::None,
        }
    }
}
