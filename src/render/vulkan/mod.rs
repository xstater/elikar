pub mod builder;
pub mod core;
pub mod systems;

use crate::render::vulkan::builder::VulkanBuilder;
use crate::window::WindowId;
use ash::vk;
use std::convert::Infallible;
use std::sync::Arc;
use xecs::System;

#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let b: $base = std::mem::zeroed();
            (&b.$field as *const _ as isize) - (&b as *const _ as isize)
        }
    }};
}

pub struct Vulkan {
    core: Arc<core::Core>,
    surface: vk::SurfaceKHR,
    surface_format: vk::SurfaceFormatKHR,
    surface_extent: vk::Extent2D,
    swapchain: vk::SwapchainKHR,
    swapchain_image_views: Vec<core::ImageView>,
    render_command_buffers: Vec<vk::CommandBuffer>,
    image_available_semaphore: core::Semaphore,
    render_finish_semaphore: core::Semaphore,
    window_id: WindowId,
}

impl Vulkan {
    pub fn builder() -> VulkanBuilder {
        VulkanBuilder::default()
    }

    pub fn core(&self) -> Arc<core::Core> {
        self.core.clone()
    }

    pub fn surface_format(&self) -> vk::SurfaceFormatKHR {
        self.surface_format
    }

    pub fn surface_extent(&self) -> vk::Extent2D {
        self.surface_extent
    }

    pub fn swapchain_image_views(&self) -> &[core::ImageView] {
        &self.swapchain_image_views
    }

    pub fn render_commands(&self) -> &[vk::CommandBuffer] {
        &self.render_command_buffers
    }

    pub fn queue_render_command(&mut self, command_buffer: vk::CommandBuffer) {
        self.render_command_buffers.push(command_buffer)
    }

    pub fn window_id(&self) -> WindowId {
        self.window_id
    }
}

impl<'a> System<'a> for Vulkan {
    type InitResource = ();
    type Resource = ();
    type Dependencies = ();
    type Error = Infallible;
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        unsafe {
            self.core
                .swapchain_manager
                .destroy_swapchain(self.swapchain, Option::None)
        }
        unsafe {
            self.core
                .surface_manager
                .destroy_surface(self.surface, Option::None)
        };
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PresentMode {
    Immediate,
    Mailbox,
    FIFO,
    FIFORelaxed,
}
