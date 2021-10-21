use crate::imgui::ImGui;
use crate::offset_of;
use crate::render::vulkan::core;
use crate::render::vulkan::core::{
    AllocateMemoryError, AshRaw, Buffer, CommandBuffers, Framebuffer,
};
use crate::render::vulkan::Vulkan;
use ash::vk;
use imgui::DrawCmd;
use nalgebra_glm as glm;
use std::cell::{Ref, RefMut};
use std::fmt::{Display, Formatter};
use std::io;
use std::slice::from_raw_parts;
use xecs::system::Dependencies;
use xecs::System;

pub struct ImGuiRenderer<Dependency> {
    render_pass: Option<core::RenderPass>,
    pipeline_layout: Option<core::PipelineLayout>,
    descriptor_set_layout: Option<core::DescriptorSetLayout>,
    descriptor_sets: Option<core::DescriptorSets>,
    pipeline: Option<core::Pipeline>,
    sampler: Option<core::Sampler>,
    vertex_buffers: Vec<core::Buffer>,
    index_buffers: Vec<core::Buffer>,
    font_image: Option<core::Image>,
    font_image_view: Option<core::ImageView>,
    projection_matrix: glm::TMat4<f32>,
    framebuffers: Vec<core::Framebuffer>,
    command_buffers: Option<CommandBuffers>,
    _marker: std::marker::PhantomData<Dependency>,
}

impl<Dependency> ImGuiRenderer<Dependency> {
    pub fn new() -> Self {
        ImGuiRenderer {
            render_pass: Option::None,
            pipeline_layout: Option::None,
            descriptor_set_layout: Option::None,
            descriptor_sets: Option::None,
            pipeline: Option::None,
            sampler: Option::None,
            vertex_buffers: vec![],
            index_buffers: vec![],
            font_image: Option::None,
            font_image_view: Option::None,
            projection_matrix: Default::default(),
            framebuffers: vec![],
            command_buffers: Option::None,
            _marker: Default::default(),
        }
    }
}

impl<'a, Dependency: Dependencies> System<'a> for ImGuiRenderer<Dependency> {
    type InitResource = (&'a mut ImGui, &'a mut Vulkan);
    type Resource = (&'a ImGui, &'a mut Vulkan);
    // run after acquired image and handled events
    type Dependencies = (Vulkan, ImGui, Dependency);
    type Error = Error;

    fn init(
        &'a mut self,
        (mut imgui, vulkan): (RefMut<'a, ImGui>, RefMut<'a, Vulkan>),
    ) -> Result<(), Self::Error> {
        let mut font = imgui.context.fonts();
        let texture = font.build_rgba32_texture();

        let font_image_builder = core::Image::builder()
            .image_type(vk::ImageType::TYPE_2D)
            .format(vk::Format::R8G8B8A8_UNORM)
            .samples(vk::SampleCountFlags::TYPE_1)
            .extent(vk::Extent3D {
                width: texture.width,
                height: texture.height,
                depth: 1,
            })
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST)
            .tiling(vk::ImageTiling::OPTIMAL);
        let mut font_image = vulkan.core().create_image(font_image_builder)?;

        let mem_req = font_image.memory_requirements();
        let memory = vulkan.core().allocate_memory(mem_req, vk::MemoryPropertyFlags::DEVICE_LOCAL)?;

        font_image.bind_memory(memory)?;

        let buffer_builder = Buffer::builder()
            .size(texture.data.len())
            .usage(vk::BufferUsageFlags::TRANSFER_SRC);
        let mut stage_buffer = vulkan.core().create_buffer(buffer_builder)?;
        let mem_req = stage_buffer.memory_requirements();
        let memory = vulkan.core().allocate_memory(
            mem_req,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
        )?;
        stage_buffer.bind_memory(memory)?;

        stage_buffer
            .memory_mut()
            .unwrap()
            .copy_from_slice(texture.data)?;

        font_image.convert_layout_to(vk::ImageLayout::TRANSFER_DST_OPTIMAL)?;
        font_image.copy_from_buffer(&stage_buffer)?;
        font_image.convert_layout_to(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)?;

        let font_image_view = font_image.create_view().build()?;
        self.font_image_view = Some(font_image_view);
        self.font_image = Some(font_image);

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
        self.render_pass = Some(vulkan.core().create_render_pass(render_pass_builder)?);

        let descriptor_set_layout_builder = core::DescriptorSetLayout::builder().binding(
            0,
            1,
            vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            vk::ShaderStageFlags::FRAGMENT,
        );
        let descriptor_set_layout =
            vulkan.core().create_descriptor_set_layout(descriptor_set_layout_builder)?;
        self.descriptor_set_layout = Some(descriptor_set_layout);

        let pipeline_layout_builder = core::PipelineLayout::builder()
            .push_constant(
                vk::ShaderStageFlags::VERTEX,
                std::mem::size_of::<glm::TMat4<f32>>() as _,
                0,
            )
            .descriptor_set_layout(self.descriptor_set_layout.as_ref().unwrap());
        self.pipeline_layout = Some(vulkan.core().create_pipeline_layout(pipeline_layout_builder)?);

        let mut descriptor_sets =
            vulkan.core().allocate_descriptor_sets(&[self.descriptor_set_layout.as_ref().unwrap()])?;

        let sampler_info = vk::SamplerCreateInfo {
            mag_filter: vk::Filter::LINEAR,
            min_filter: vk::Filter::NEAREST,
            mipmap_mode: vk::SamplerMipmapMode::LINEAR,
            address_mode_u: vk::SamplerAddressMode::REPEAT,
            address_mode_v: vk::SamplerAddressMode::REPEAT,
            address_mode_w: vk::SamplerAddressMode::REPEAT,
            anisotropy_enable: vk::FALSE,
            // 16x anisotropy
            max_anisotropy: 16.0,
            compare_enable: vk::FALSE,
            compare_op: vk::CompareOp::ALWAYS,
            border_color: vk::BorderColor::FLOAT_OPAQUE_BLACK,
            unnormalized_coordinates: vk::FALSE,
            ..Default::default()
        };
        let sampler = vulkan.core().create_sampler(sampler_info)?;

        descriptor_sets.update_combined_image_sampler(
            0,
            0,
            &sampler,
            self.font_image_view.as_ref().unwrap(),
            self.font_image.as_ref().unwrap().layout(),
        );
        self.descriptor_sets = Some(descriptor_sets);
        self.sampler = Some(sampler);

        let pipeline_builder = core::Pipeline::builder()
            .shader_from_file(vk::ShaderStageFlags::VERTEX, "shaders/imgui/vert.spv")
            .shader_from_file(vk::ShaderStageFlags::FRAGMENT, "shaders/imgui/frag.spv")
            .input_binding::<imgui::DrawVert>(0, vk::VertexInputRate::VERTEX)
            .input_attribute(
                0,
                0,
                vk::Format::R32G32_SFLOAT,
                offset_of!(imgui::DrawVert, pos) as _,
            )
            .input_attribute(
                1,
                0,
                vk::Format::R32G32_SFLOAT,
                offset_of!(imgui::DrawVert, uv) as _,
            )
            .input_attribute(
                2,
                0,
                vk::Format::R32_UINT,
                offset_of!(imgui::DrawVert, col) as _,
            )
            .input_assembly(false, vk::PrimitiveTopology::TRIANGLE_LIST)
            .with_surface_area(&vulkan.surface_extent())
            .pipeline_layout(self.pipeline_layout.as_ref().unwrap())
            .dynamic_state(vk::DynamicState::SCISSOR)
            .dynamic_state(vk::DynamicState::VIEWPORT)
            .color_blend_attachment(vk::PipelineColorBlendAttachmentState {
                blend_enable: vk::TRUE,
                src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
                dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
                color_blend_op: vk::BlendOp::ADD,
                src_alpha_blend_factor: vk::BlendFactor::ONE,
                dst_alpha_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
                alpha_blend_op: vk::BlendOp::ADD,
                color_write_mask: vk::ColorComponentFlags::all(),
            })
            .render_pass(self.render_pass.as_ref().unwrap())
            .subpass(0);
        self.pipeline = Some(vulkan.core().create_pipeline(pipeline_builder)?);

        let views_len = vulkan.swapchain_image_views().len();
        self.command_buffers = Some(vulkan.core().allocate_command_buffers(views_len)?);

        for index in 0..views_len {
            let view = unsafe { vulkan.swapchain_image_views().get_unchecked(index) };
            let framebuffer_builder = Framebuffer::builder(self.render_pass.as_ref().unwrap())
                .view(view)
                .size(
                    vulkan.surface_extent().width,
                    vulkan.surface_extent().height,
                );
            let framebuffer = vulkan.core().create_framebuffer(framebuffer_builder)?;
            self.framebuffers.push(framebuffer);
        }

        Ok(())
    }

    fn update(
        &'a mut self,
        (imgui, mut vulkan): (
            Ref<'a, ImGui>,
            RefMut<'a, Vulkan>,
        ),
    ) -> Result<(), Self::Error> {
        let draw_data = imgui.draw_data();

        self.projection_matrix = glm::ortho(
            0.0,
            draw_data.framebuffer_scale[0] * draw_data.display_size[0],
            0.0,
            draw_data.framebuffer_scale[1] * draw_data.display_size[1],
            -1.0,
            1.0,
        );
        // vulkan render
        let image_index = vulkan.image_index();

        let command_buffer = self.command_buffers.as_ref().unwrap().raw()[image_index as usize];
        let render_pass = *self.render_pass.as_ref().unwrap().raw();
        let framebuffer = *self.framebuffers[image_index as usize].raw();
        let pipeline = *self.pipeline.as_ref().unwrap().raw();
        let pipeline_layout = *self.pipeline_layout.as_ref().unwrap().raw();

        unsafe {
            let core = vulkan.core();
            let device = core.device();
            device.reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::all())?;

            let begin_info = vk::CommandBufferBeginInfo::default();
            device.begin_command_buffer(command_buffer, &begin_info)?;

            let clear_color = [vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [1.0, 1.0, 1.0, 0.0],
                },
            }];
            let begin_render_pass_info = vk::RenderPassBeginInfo::builder()
                .render_pass(render_pass)
                .framebuffer(framebuffer)
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: vulkan.surface_extent(),
                })
                .clear_values(&clear_color);
            device.cmd_begin_render_pass(
                command_buffer,
                &begin_render_pass_info,
                vk::SubpassContents::INLINE,
            );
            device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
            device.cmd_push_constants(
                command_buffer,
                pipeline_layout,
                vk::ShaderStageFlags::VERTEX,
                0,
                to_u8_slice(self.projection_matrix.as_slice()),
            );
            device.cmd_bind_descriptor_sets(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                pipeline_layout,
                0,
                self.descriptor_sets.as_ref().unwrap().raw(),
                &[],
            );

            let viewport = [vk::Viewport {
                x: 0.0,
                y: 0.0,
                width: vulkan.surface_extent().width as _,
                height: vulkan.surface_extent().height as _,
                min_depth: 0.0,
                max_depth: 1.0,
            }];
            device.cmd_set_viewport(command_buffer, 0, &viewport);
        }

        self.vertex_buffers.clear();
        self.index_buffers.clear();

        for draw_list in draw_data.draw_lists() {
            let vertex_buffer_data = draw_list.vtx_buffer();
            let index_buffer_data = draw_list.idx_buffer();

            // vertex buffer
            let size = vertex_buffer_data.len() * std::mem::size_of::<imgui::DrawVert>();
            let mut stage_buffer = vulkan.core().create_buffer(
                core::Buffer::builder()
                    .size(size)
                    .usage(vk::BufferUsageFlags::TRANSFER_SRC),
            )?;
            let memory = vulkan.core().allocate_memory(
                stage_buffer.memory_requirements(),
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            )?;
            stage_buffer.bind_memory(memory)?;
            stage_buffer
                .memory_mut()
                .unwrap()
                .copy_from_slice(vertex_buffer_data)?;

            let mut vertex_buffer = vulkan.core().create_buffer(
                core::Buffer::builder()
                    .size(stage_buffer.size() as _)
                    .usage(
                        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
                    ),
            )?;
            let memory = vulkan.core().allocate_memory(
                vertex_buffer.memory_requirements(),
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            )?;
            vertex_buffer.bind_memory(memory)?;
            vertex_buffer.copy_from_buffer(&mut stage_buffer)?;

            // index buffer
            let size = index_buffer_data.len() * std::mem::size_of::<imgui::DrawIdx>();
            let mut stage_buffer = vulkan.core().create_buffer(
                core::Buffer::builder()
                    .size(size)
                    .usage(vk::BufferUsageFlags::TRANSFER_SRC),
            )?;
            let memory = vulkan.core().allocate_memory(
                stage_buffer.memory_requirements(),
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            )?;
            stage_buffer.bind_memory(memory)?;
            stage_buffer
                .memory_mut()
                .unwrap()
                .copy_from_slice(index_buffer_data)?;

            let mut index_buffer = vulkan.core().create_buffer(
                core::Buffer::builder()
                    .size(stage_buffer.size() as _)
                    .usage(vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER),
            )?;
            let memory = vulkan.core().allocate_memory(
                index_buffer.memory_requirements(),
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            )?;
            index_buffer.bind_memory(memory)?;
            index_buffer.copy_from_buffer(&mut stage_buffer)?;

            self.vertex_buffers.push(vertex_buffer);
            self.index_buffers.push(index_buffer);

            let clip_off = draw_data.display_pos;
            let clip_scale = draw_data.framebuffer_scale;

            unsafe {
                let core = vulkan.core();
                let device = core.device();
                device.cmd_bind_vertex_buffers(
                    command_buffer,
                    0,
                    &[*self.vertex_buffers.last().unwrap().raw()],
                    &[0],
                );
                device.cmd_bind_index_buffer(
                    command_buffer,
                    *self.index_buffers.last().unwrap().raw(),
                    0,
                    vk::IndexType::UINT16,
                );

                for draw_cmd in draw_list.commands() {
                    if let DrawCmd::Elements { count, cmd_params } = draw_cmd {
                        let mut clip_min = (
                            (cmd_params.clip_rect[0] - clip_off[0]) * clip_scale[0],
                            (cmd_params.clip_rect[1] - clip_off[1]) * clip_scale[1],
                        );
                        let mut clip_max = (
                            (cmd_params.clip_rect[2] - clip_off[0]) * clip_scale[0],
                            (cmd_params.clip_rect[3] - clip_off[1]) * clip_scale[1],
                        );

                        let extend = vulkan.surface_extent();

                        if clip_min.0 < 0.0 {
                            clip_min.0 = 0.0
                        };
                        if clip_min.1 < 0.0 {
                            clip_min.1 = 0.0
                        };
                        if clip_max.0 > extend.width as _ {
                            clip_max.0 = extend.width as _
                        };
                        if clip_max.1 > extend.height as _ {
                            clip_max.1 = extend.height as _
                        };
                        if clip_max.0 < clip_min.0 || clip_max.1 < clip_min.1 {
                            continue;
                        }

                        let scissor = [vk::Rect2D {
                            offset: vk::Offset2D {
                                x: clip_min.0 as _,
                                y: clip_min.1 as _,
                            },
                            extent: vk::Extent2D {
                                width: (clip_max.0 - clip_min.0) as _,
                                height: (clip_max.1 - clip_min.1) as _,
                            },
                        }];
                        device.cmd_set_scissor(command_buffer, 0, &scissor);
                        device.cmd_draw_indexed(
                            command_buffer,
                            count as _,
                            1,
                            cmd_params.idx_offset as _,
                            cmd_params.vtx_offset as _,
                            0,
                        );
                    }
                }
            }
        }
        unsafe {
            let core = vulkan.core();
            let device = core.device();
            device.cmd_end_render_pass(command_buffer);
            device.end_command_buffer(command_buffer)?;
        }
        vulkan.queue_render_command(command_buffer);

        Ok(())
    }
}

fn to_u8_slice<T: Sized>(slice: &[T]) -> &[u8] {
    let ptr = slice.as_ptr();
    let ptr = ptr as *const _;
    unsafe { from_raw_parts(ptr, slice.len() * std::mem::size_of::<T>()) }
}

#[derive(Debug)]
pub enum Error {
    VulkanError(vk::Result),
    IoError(io::Error),
    AllocateMemoryError,
}

impl From<vk::Result> for Error {
    fn from(vk_err: vk::Result) -> Self {
        Error::VulkanError(vk_err)
    }
}

impl From<io::Error> for Error {
    fn from(io_err: io::Error) -> Self {
        Error::IoError(io_err)
    }
}

impl From<AllocateMemoryError> for Error {
    fn from(alloc_err: AllocateMemoryError) -> Self {
        match alloc_err {
            AllocateMemoryError::VulkanError(vk_err) => Error::VulkanError(vk_err),
            AllocateMemoryError::FindMemoryFailed => Error::AllocateMemoryError,
        }
    }
}

impl From<core::CreatePipelineError> for Error {
    fn from(pipeline_err: core::CreatePipelineError) -> Self {
        match pipeline_err {
            core::CreatePipelineError::VulkanError(vk_err) => Error::VulkanError(vk_err),
            core::CreatePipelineError::IoError(io_err) => Error::IoError(io_err),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::VulkanError(vk_err) => {
                write!(f, "ImGui Render System Error! Vulkan error:{}", vk_err)
            }
            Error::IoError(io_err) => write!(f, "ImGui Render System Error! IO error:{}", io_err),
            Error::AllocateMemoryError => write!(
                f,
                "ImGui Render System Error! Allocate vulkan memory error!"
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::VulkanError(vk_err) => Some(vk_err),
            Error::IoError(io_err) => Some(io_err),
            _ => Option::None,
        }
    }
}