use crate::render::vulkan::core::AshRaw;
use crate::render::vulkan::systems::acquire_next_image::AcquireNextImage;
use crate::render::vulkan::Vulkan;
use ash::vk;
use std::cell::{Ref, RefMut};
use xecs::{End, System};

pub struct ExecuteRenderCommands;

impl ExecuteRenderCommands {
    pub fn new() -> Self {
        ExecuteRenderCommands
    }
}

impl<'a> System<'a> for ExecuteRenderCommands {
    type InitResource = ();
    type Resource = (&'a AcquireNextImage, &'a mut Vulkan);
    type Dependencies = End;
    type Error = vk::Result;

    fn init(&'a mut self, _: ()) -> Result<(), Self::Error> {
        Ok(())
    }

    fn update(
        &'a mut self,
        (acq_next_img, mut vulkan): (Ref<'a, AcquireNextImage>, RefMut<'a, Vulkan>),
    ) -> Result<(), Self::Error> {
        if vulkan.render_command_buffers.is_empty() {
            let signal_semaphores = [*vulkan.image_available_semaphore.raw()];
            let swapchains = [vulkan.swapchain];
            let image_indices = [acq_next_img.image_index()];
            let present_info = vk::PresentInfoKHR::builder()
                .wait_semaphores(&signal_semaphores)
                .swapchains(&swapchains)
                .image_indices(&image_indices);
            unsafe {
                vulkan
                    .core
                    .swapchain_manager
                    .queue_present(vulkan.core.graphics_queue, &present_info)?;
                vulkan
                    .core
                    .device
                    .queue_wait_idle(vulkan.core.graphics_queue)?;
            }
            return Ok(());
        }

        let image_index = acq_next_img.image_index();

        let wait_semaphores = [*vulkan.image_available_semaphore.raw()];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffer = &vulkan.render_command_buffers;
        let signal_semaphores = [*vulkan.render_finish_semaphore.raw()];

        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&command_buffer)
            .signal_semaphores(&signal_semaphores);
        let submit_infos = [submit_info.build()];

        unsafe {
            vulkan.core.device.queue_submit(
                vulkan.core.graphics_queue,
                &submit_infos,
                vk::Fence::null(),
            )
        }?;

        let swapchains = [vulkan.swapchain];
        let image_indices = [image_index];

        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        unsafe {
            vulkan
                .core
                .swapchain_manager
                .queue_present(vulkan.core.graphics_queue, &present_info)?;
            vulkan
                .core
                .device
                .queue_wait_idle(vulkan.core.graphics_queue)?;
        }

        vulkan.render_command_buffers.clear();

        Ok(())
    }
}
