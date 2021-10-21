use std::cell::{Ref, RefMut};

use ash::vk;
use xecs::System;
use crate::render::vulkan::{Vulkan, core::{self, AshRaw}};

pub struct FrameBegin{
    need_update: bool,
    background_color: [f32;4],
    render_pass: Option<core::RenderPass>,
    framebuffers: Vec<core::Framebuffer>,
    command_buffers: Option<core::CommandBuffers>
}

impl FrameBegin {
    pub fn new() -> FrameBegin {
        FrameBegin {
            need_update: false,
            background_color: [0.0,0.0,0.0,1.0],
            render_pass: Option::None,
            framebuffers: Vec::new(),
            command_buffers: Option::None,
        }
    }

    pub fn background_color(&self) -> &[f32;4] {
        &self.background_color
    }

    pub fn set_background_color(&mut self,color : (f32,f32,f32,f32)) {
        let (r,g,b,a) = color;
        self.need_update = true;
        self.background_color = [r,g,b,a];
    }

    fn update_commands(&mut self,) -> Result<(),vk::Result> {
        let image_count = self.framebuffers.len();
        let clear_color = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: self.background_color,
            },
        }];
        let render_pass = *self.render_pass.as_ref().unwrap().raw();
        for index in 0..image_count {
            let framebuffer = *self.framebuffers[index].raw();
            let extent = self.framebuffers[index].size_as_extent();
            self.command_buffers
                .as_mut()
                .unwrap()
                .reset(index)?
                .record(index,|device,command_buffer| -> Result<(),vk::Result>{
                    let begin_info = vk::RenderPassBeginInfo::builder()
                        .render_pass(render_pass)
                        .framebuffer(framebuffer)
                        .render_area(vk::Rect2D{
                            offset: vk::Offset2D{ x:0,y:0 },
                            extent,
                        })
                        .clear_values(&clear_color);
                    unsafe {
                        device.cmd_begin_render_pass(
                            command_buffer,
                            &begin_info,
                            vk::SubpassContents::INLINE);
                        device.cmd_end_render_pass(command_buffer);
                    }
                    Ok(())
                })?;
        }
        Ok(())
    }
}

impl<'a> System<'a> for FrameBegin {
    type InitResource = &'a Vulkan;
    type Resource = &'a mut Vulkan;
    type Dependencies = Vulkan;
    type Error = vk::Result;

    fn init(&'a mut self,vulkan : Ref<'a,Vulkan>) -> Result<(),Self::Error>{
        // just clear the screen
        let render_pass_builder = core::RenderPass::builder()
            // color attachment
            .attachment(vk::AttachmentDescription {
                format: vulkan.surface_format().format,
                samples: vk::SampleCountFlags::TYPE_1,
                load_op: vk::AttachmentLoadOp::CLEAR,
                store_op: vk::AttachmentStoreOp::STORE,
                stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
                stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
                initial_layout: vk::ImageLayout::UNDEFINED,
                final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                ..Default::default()
            })
            .subpass(
                vk::SubpassDependency {
                    src_subpass: vk::SUBPASS_EXTERNAL,
                    src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                    dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                    dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
                        | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                    ..Default::default()
                },
                vk::SubpassDescription::builder()
                    .color_attachments(&[vk::AttachmentReference {
                        attachment: 0,
                        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                    }])
                    .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                    .build(),
            );
        let render_pass = vulkan.core().create_render_pass(render_pass_builder)?;

        // framebuffers
        for view in vulkan.swapchain_image_views().iter() {
            let framebuffer = core::Framebuffer::builder(&render_pass)
                .view(view)
                .size(
                    vulkan.surface_extent.width,
                    vulkan.surface_extent.height);
            let framebuffer = vulkan.core().create_framebuffer(framebuffer)?;
            self.framebuffers.push(framebuffer);
        }

        // command buffers
        let image_count = vulkan.swapchain_image_views().len();
        let command_buffers = vulkan.core().allocate_command_buffers(image_count)?;

        self.render_pass = Some(render_pass);
        self.command_buffers = Some(command_buffers);
        
        self.update_commands()?;
        
        Ok(())
    }

    fn update(&'a mut self,mut vulkan : RefMut<'a,Vulkan>) -> Result<(),Self::Error>{
        if self.need_update {
            self.update_commands()?;
            self.need_update = false;
        }
        vulkan.queue_current_render_command(self.command_buffers.as_ref().unwrap());
        Ok(())
    }
}
