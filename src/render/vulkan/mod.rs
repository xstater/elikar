mod builder;

use ash::{Instance, Entry, vk, Device,extensions::khr};
use crate::render::vulkan::builder::VulkanBuilder;
use crate::window::{WindowId};

pub struct Vulkan {
    #[allow(unused)]
    pub entry : Entry,
    pub instance : Instance,
    pub debug_utils : Option<ash::extensions::ext::DebugUtils>,
    pub messenger : Option<vk::DebugUtilsMessengerEXT>,
    pub physical_device : vk::PhysicalDevice,
    pub memory_properties : vk::PhysicalDeviceMemoryProperties,
    pub device : Device,
    pub graphics_queue : vk::Queue,
    pub transfer_queue : vk::Queue,
    pub command_pool : vk::CommandPool,
    pub surface_manager : khr::Surface,
    pub surface : vk::SurfaceKHR,
    pub surface_format : vk::SurfaceFormatKHR,
    pub surface_extent : vk::Extent2D,
    pub swapchain_manager : khr::Swapchain,
    pub swapchain : vk::SwapchainKHR,
    pub swapchain_image_views : Vec<vk::ImageView>,
    window_id : WindowId,
}

impl Vulkan {
    pub fn builder() -> VulkanBuilder{
        VulkanBuilder::default()
    }

    pub fn rebuild_swapchain(&mut self) {
    }

    pub fn window_id(&self) -> WindowId {
        self.window_id
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        for view in &self.swapchain_image_views {
            unsafe { self.device.destroy_image_view(*view,Option::None) };
        }
        unsafe { self.swapchain_manager.destroy_swapchain(self.swapchain,Option::None) }
        unsafe { self.surface_manager.destroy_surface(self.surface,Option::None) };
        unsafe { self.device.destroy_command_pool(self.command_pool,Option::None) };
        unsafe { self.device.destroy_device(Option::None) };
        if let Some(debug_utils) = &self.debug_utils {
            if let Some(messenger) = &self.messenger {
                unsafe {
                    debug_utils.destroy_debug_utils_messenger(*messenger,Option::None)
                };
            }
        }
        unsafe { self.instance.destroy_instance(Option::None) };
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum PresentMode {
    Immediate,
    Mailbox,
    FIFO,
    FIFORelaxed
}
