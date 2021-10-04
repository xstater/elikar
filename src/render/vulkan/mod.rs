pub mod builder;
pub mod systems;
pub mod core;

use std::convert::Infallible;
use crate::render::vulkan::builder::VulkanBuilder;
use crate::window::{WindowId};
use std::sync::Arc;
use ash::vk;
use xecs::System;
use crate::render::vulkan::core::Sampler;

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
    core : Arc<core::Core>,
    surface : vk::SurfaceKHR,
    surface_format : vk::SurfaceFormatKHR,
    surface_extent : vk::Extent2D,
    swapchain : vk::SwapchainKHR,
    swapchain_image_views : Vec<core::ImageView>,
    render_command_buffers : Vec<vk::CommandBuffer>,
    image_available_semaphore : core::Semaphore,
    render_finish_semaphore : core::Semaphore,
    window_id : WindowId,
}

impl Vulkan {
    pub fn builder() -> VulkanBuilder{
        VulkanBuilder::default()
    }

    pub fn core(&self) -> Arc<core::Core> {
        self.core.clone()
    }

    pub fn surface_format(&self) -> vk::SurfaceFormatKHR { self.surface_format }

    pub fn surface_extent(&self) -> vk::Extent2D { self.surface_extent }

    pub fn swapchain_image_views(&self) -> &[core::ImageView] { &self.swapchain_image_views }

    pub fn render_commands(&self) -> &[vk::CommandBuffer] { &self.render_command_buffers }

    pub fn queue_render_command(&mut self,command_buffer : vk::CommandBuffer) {
        self.render_command_buffers.push(command_buffer)
    }

    pub fn create_render_pass(&mut self,builder : core::RenderPassBuilder) -> Result<core::RenderPass,vk::Result> {
        builder.build(self.core.clone())
    }

    pub fn create_descriptor_set_layout(&mut self,builder : core::DescriptorSetLayoutBuilder) -> Result<core::DescriptorSetLayout,vk::Result> {
        builder.build(self.core.clone())
    }

    pub fn allocate_descriptor_sets(&mut self,layouts : &[&core::DescriptorSetLayout]) -> Result<core::DescriptorSets,vk::Result>{
        let layouts = layouts.iter()
            .map(|layout|{
                layout.descriptor_layout
            }).collect::<Vec<_>>();
        core::DescriptorSets::allocate(self.core.clone(),&layouts)
    }

    pub fn create_pipeline_layout(&mut self,builder : core::PipelineLayoutBuilder) -> Result<core::PipelineLayout,vk::Result> {
        builder.build(self.core.clone())
    }

    pub fn create_pipeline(&mut self,builder : core::PipelineBuilder) -> Result<core::Pipeline,core::CreatePipelineError> {
        builder.build(self.core.clone())
    }

    pub fn create_framebuffer(&mut self,builder : core::FramebufferBuilder) -> Result<core::Framebuffer,vk::Result> {
        builder.build(self.core.clone())
    }

    pub fn create_image(&mut self,builder : core::ImageBuilder) -> Result<core::Image,vk::Result> {
        builder.build(self.core.clone())
    }

    pub fn create_sampler(&mut self,builder : vk::SamplerCreateInfo) -> Result<core::Sampler,vk::Result> {
        Sampler::build(self.core.clone(),builder)
    }

    pub fn create_semaphore(&mut self) -> Result<core::Semaphore,vk::Result> {
        core::Semaphore::new(self.core.clone())
    }

    pub fn create_buffer(&mut self,builder : core::BufferBuilder) -> Result<core::Buffer,vk::Result> {
        builder.build(self.core.clone())
    }

    pub fn allocate_memory(&mut self,reqs : vk::MemoryRequirements,flags : vk::MemoryPropertyFlags)
        -> Result<core::DeviceMemory,core::AllocateMemoryError>{
        core::DeviceMemory::allocate(self.core.clone(),reqs,flags)
    }

    pub fn allocate_command_buffers(&mut self,count : usize) -> Result<core::CommandBuffers,vk::Result> {
        core::CommandBuffers::allocate(self.core.clone(),count)
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
        unsafe { self.core.swapchain_manager.destroy_swapchain(self.swapchain,Option::None) }
        unsafe { self.core.surface_manager.destroy_surface(self.surface,Option::None) };
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum PresentMode {
    Immediate,
    Mailbox,
    FIFO,
    FIFORelaxed
}