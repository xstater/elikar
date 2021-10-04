mod image_view;
mod pipeline;
mod pipeline_layout;
mod render_pass;
mod framebuffer;
mod device_memory;
mod semaphore;
mod buffer;
mod descriptor_set_layout;
mod command_buffers;
mod shader;
mod image;
mod sampler;
mod descriptor_sets;

use ash::{Entry, Instance, vk, Device};
use ash::extensions::khr;

pub use image::{
    Image,
    ImageBuilder
};
pub use image_view::{
    ImageView,
    ImageViewBuilder
};
pub use pipeline::*;
pub use pipeline_layout::{
    PipelineLayout,
    PipelineLayoutBuilder
};
pub use render_pass::{
    RenderPass,
    RenderPassBuilder
};
pub use framebuffer::{
    Framebuffer,
    FramebufferBuilder
};
pub use semaphore::Semaphore;
pub use buffer::{
    Buffer,
    BufferBuilder
};
pub use device_memory::{
    DeviceMemory,
    AllocateMemoryError
};
pub use descriptor_set_layout::{
    DescriptorSetLayout,
    DescriptorSetLayoutBuilder
};
pub use sampler::Sampler;
pub use descriptor_sets::DescriptorSets;
pub use command_buffers::CommandBuffers;
pub use shader::Shader;

pub trait AshRaw {
    type Raw : ?Sized;

    fn raw(&self) -> &Self::Raw;
}

pub struct Core {
    #[allow(unused)]
    pub(in crate::render) entry : Entry,
    pub(in crate::render) instance : Instance,
    pub(in crate::render) debug_utils : Option<ash::extensions::ext::DebugUtils>,
    pub(in crate::render) messenger : Option<vk::DebugUtilsMessengerEXT>,
    pub(in crate::render) physical_device : vk::PhysicalDevice,
    pub(in crate::render) properties : vk::PhysicalDeviceProperties,
    pub(in crate::render) memory_properties : vk::PhysicalDeviceMemoryProperties,
    pub(in crate::render) device : Device,
    pub(in crate::render) queue_family_index : u32,
    pub(in crate::render) graphics_queue : vk::Queue,
    pub(in crate::render) transfer_queue : vk::Queue,
    pub(in crate::render) command_pool : vk::CommandPool,
    pub(in crate::render) descriptor_pool : vk::DescriptorPool,
    pub(in crate::render) surface_manager : khr::Surface,
    pub(in crate::render) swapchain_manager : khr::Swapchain,
}

impl Core {
    pub fn entry(&self) -> &Entry {
        &self.entry
    }

    pub fn instance(&self) -> &Instance {
        &self.instance
    }

    pub fn physical_device(&self) -> vk::PhysicalDevice {
        self.physical_device
    }

    pub fn physical_device_properties(&self) -> &vk::PhysicalDeviceProperties {
        &self.properties
    }

    pub fn physical_device_memory_properties(&self) -> &vk::PhysicalDeviceMemoryProperties {
        &self.memory_properties
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }

    pub fn graphics_queue(&self) -> vk::Queue {
        self.graphics_queue
    }

    pub fn transfer_queue(&self) -> vk::Queue {
        self.transfer_queue
    }

    pub fn command_pool(&self) -> vk::CommandPool {
        self.command_pool
    }

    pub fn descriptor_pool(&self) -> vk::DescriptorPool {
        self.descriptor_pool
    }

    pub fn surface_manager(&self) -> &khr::Surface {
        &self.surface_manager
    }

    pub fn swapchain_manager(&self) -> &khr::Swapchain {
        &self.swapchain_manager
    }

}

impl Drop for Core {
    fn drop(&mut self) {
        unsafe { self.device.destroy_descriptor_pool(self.descriptor_pool,Option::None) }
        unsafe { self.device.destroy_command_pool(self.command_pool,Option::None) };
        unsafe { self.device.destroy_device(Option::None) };
        if let Some(debug_utils) = &self.debug_utils {
            if let Some(messenger) = &self.messenger {
                unsafe {
                    debug_utils.destroy_debug_utils_messenger(*messenger,Option::None)
                };
            }
        }
        unsafe { self.instance.destroy_instance(Option::None) }
    }
}