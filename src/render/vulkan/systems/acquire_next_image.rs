use std::cell::Ref;
use ash::vk;
use xecs::System;
use crate::render::vulkan::core::AshRaw;
use crate::render::vulkan::Vulkan;

#[derive(Debug)]
pub struct AcquireNextImage{
    image_index : u32
}

impl AcquireNextImage{
    pub fn new() -> AcquireNextImage{
        AcquireNextImage{
            image_index: 0
        }
    }

    pub fn image_index(&self) -> u32 {
        self.image_index
    }
}

impl<'a> System<'a> for AcquireNextImage {
    type InitResource = ();
    type Resource = &'a Vulkan;
    type Dependencies = ();
    type Error = vk::Result;

    fn init(&'a mut self, _ : ()) -> Result<(), Self::Error> {
        Ok(())
    }

    fn update(&'a mut self, vulkan : Ref<'a,Vulkan>) -> Result<(), Self::Error> {
        let (image_index,_) = unsafe {
            vulkan.core.swapchain_manager.acquire_next_image(
                vulkan.swapchain,
                u64::MAX,
                *vulkan.image_available_semaphore.raw(),
                vk::Fence::null())
        }?;
        self.image_index = image_index;
        Ok(())
    }
}

