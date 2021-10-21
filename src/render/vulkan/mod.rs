pub mod builder;
pub mod core;
pub mod systems;

use crate::render::vulkan::builder::VulkanBuilder;
use crate::window::WindowId;
use ash::vk;
use std::sync::Arc;
use xecs::System;

use self::core::AshRaw;

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
    pub(in crate::render) core: Arc<core::Core>,
    pub(in crate::render) surface: vk::SurfaceKHR,
    pub(in crate::render) surface_format: vk::SurfaceFormatKHR,
    pub(in crate::render) surface_extent: vk::Extent2D,
    pub(in crate::render) swapchain: vk::SwapchainKHR,
    pub(in crate::render) swapchain_image_views: Vec<core::ImageView>,
    pub(in crate::render) render_command_buffers: Vec<vk::CommandBuffer>,
    pub(in crate::render) image_available_semaphore: core::Semaphore,
    pub(in crate::render) render_finish_semaphore: core::Semaphore,
    pub(in crate::render) window_id: WindowId,
    image_index : u32,
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

    pub fn swapchain_image_view(&self) -> &core::ImageView {
        unsafe {
            self.swapchain_image_views
                .get_unchecked(self.image_index as usize)
        }
    }

    pub fn render_commands(&self) -> &[vk::CommandBuffer] {
        &self.render_command_buffers
    }

    pub fn queue_render_command(&mut self,index : usize,command_buffers: &core::CommandBuffers) {
        let command_buffer = *unsafe {
            command_buffers.raw().get_unchecked(index)
        };
        self.render_command_buffers.push(command_buffer);
    }

    pub fn queue_current_render_command(&mut self,command_buffers: &core::CommandBuffers) {
        self.queue_render_command(self.image_index as usize, command_buffers);
    }

    pub fn window_id(&self) -> WindowId {
        self.window_id
    }
    
    pub fn image_index(&self) -> u32 {
        self.image_index
    }

    pub(in crate::render::vulkan) fn present_queue(&mut self) -> Result<(),vk::Result> {
        let graphics_queue = self.core.graphics_queue;
        if self.render_command_buffers.is_empty() {
            let signal_semaphores = [*self.image_available_semaphore.raw()];
            let swapchains = [self.swapchain];
            let image_indices = [self.image_index];
            let present_info = vk::PresentInfoKHR::builder()
                .wait_semaphores(&signal_semaphores)
                .swapchains(&swapchains)
                .image_indices(&image_indices);
            unsafe {
                self.core
                    .swapchain_manager
                    .queue_present(graphics_queue, &present_info)?;
                self.core
                    .device
                    .queue_wait_idle(graphics_queue)?;
            }
            return Ok(());
        }

        let wait_semaphores = [*self.image_available_semaphore.raw()];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffer = &self.render_command_buffers;
        let signal_semaphores = [*self.render_finish_semaphore.raw()];

        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&command_buffer)
            .signal_semaphores(&signal_semaphores);
        let submit_infos = [submit_info.build()];

        unsafe {
            self.core.device.queue_submit(
                self.core.graphics_queue,
                &submit_infos,
                vk::Fence::null(),
            )
        }?;

        let swapchains = [self.swapchain];
        let image_indices = [self.image_index];

        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        unsafe {
            self .core
                .swapchain_manager
                .queue_present(graphics_queue, &present_info)?;
            self .core
                .device
                .queue_wait_idle(graphics_queue)?;
        }

        self.render_command_buffers.clear();

        Ok(())
    }
}

impl<'a> System<'a> for Vulkan {
    type InitResource = ();
    type Resource = ();
    type Dependencies = ();
    type Error = vk::Result;

    fn update(&'a mut self,_ : ()) -> Result<(),Self::Error>{
        let (image_index, _) = unsafe {
            self.core.swapchain_manager.acquire_next_image(
                self.swapchain,
                u64::MAX,
                *self.image_available_semaphore.raw(),
                vk::Fence::null(),
            )
        }?;
        self.image_index = image_index;
        Ok(())
    }
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
