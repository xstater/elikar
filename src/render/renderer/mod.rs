pub mod systems;
pub mod error;

use crate::render::vulkan::Vulkan;
use std::sync::Arc;
use ash::vk;
use std::fs::File;
use crate::render::renderer::error::BuildRendererError;
use std::ffi::CString;
use nalgebra_glm as glm;
use crate::render::sprite::Sprite;

pub struct Renderer{
    vulkan : Arc<Vulkan>,
    render_passes : Vec<vk::RenderPass>,
    pipeline_layouts : Vec<vk::PipelineLayout>,
    pipelines : Vec<vk::Pipeline>,
    framebuffers : Vec<vk::Framebuffer>,
    command_buffers : Vec<vk::CommandBuffer>,
    need_update_commands : Vec<bool>,
    image_available_semaphore : vk::Semaphore,
    render_finish_semaphore : vk::Semaphore,
    clear_color : glm::TVec3<f32>,
    sprite_vertices : Vec<Vertex>,
    sprite_vertex_buffer : vk::Buffer,
    sprite_memory : vk::DeviceMemory
}

macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let b: $base = std::mem::zeroed();
            (&b.$field as *const _ as isize) - (&b as *const _ as isize)
        }
    }};
}

#[derive(Debug,Copy,Clone)]
#[repr(C)]
struct Vertex{
    pos : [f32;3]
}

impl Renderer {
    pub fn new(vulkan : Arc<Vulkan>) -> Result<Renderer,BuildRendererError> {
        // build a 2d pipeline
        let attachments = [
            vk::AttachmentDescription{
                format: vulkan.surface_format.format,
                samples : vk::SampleCountFlags::TYPE_1,
                load_op: vk::AttachmentLoadOp::CLEAR,
                store_op: vk::AttachmentStoreOp::STORE,
                stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
                stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
                final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                .. Default::default()
            }
        ];

        let color_attachment_refs = [
            vk::AttachmentReference{
                attachment: 0,
                layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL
            }
        ];

        let dependencies = [
            vk::SubpassDependency{
                src_subpass: vk::SUBPASS_EXTERNAL,
                src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                dst_access_mask:
                    vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                .. Default::default()
            }
        ];

        let subpasses = [
            vk::SubpassDescription::builder()
                .color_attachments(&color_attachment_refs)
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .build()
        ];

        let render_pass_info = vk::RenderPassCreateInfo::builder()
            .attachments(&attachments)
            .dependencies(&dependencies)
            .subpasses(&subpasses);

        let render_pass = unsafe {
            vulkan.device.create_render_pass(&render_pass_info,Option::None)
        }?;

        let mut vs_file = File::open("./shaders/render2d/render2d.vert.spv")?;
        let mut fs_file = File::open("./shaders/render2d/render2d.frag.spv")?;
        let vs_spv = ash::util::read_spv(&mut vs_file)?;
        let fs_spv = ash::util::read_spv(&mut fs_file)?;

        let vs_info = vk::ShaderModuleCreateInfo::builder()
            .code(&vs_spv);
        let fs_info = vk::ShaderModuleCreateInfo::builder()
            .code(&fs_spv);

        let vs = unsafe {
            vulkan.device.create_shader_module(&vs_info,Option::None)
        }?;
        let fs = unsafe {
            vulkan.device.create_shader_module(&fs_info,Option::None)
        }?;

        let function_name = CString::new("main").unwrap();
        let shader_stages = [
            vk::PipelineShaderStageCreateInfo{
                stage: vk::ShaderStageFlags::VERTEX,
                module: vs,
                p_name: function_name.as_ptr(),
                .. Default::default()
            },
            vk::PipelineShaderStageCreateInfo{
                stage: vk::ShaderStageFlags::FRAGMENT,
                module: fs,
                p_name: function_name.as_ptr(),
                .. Default::default()
            }
        ];

        let vertex_input_bindings = [
            vk::VertexInputBindingDescription{
                binding: 0,
                stride: std::mem::size_of::<Vertex>() as _,
                input_rate: vk::VertexInputRate::VERTEX
            }
        ];

        let vertex_input_attributes = [
            // pos
            vk::VertexInputAttributeDescription{
                location: 0,
                binding: 0,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: offset_of!(Vertex,pos) as _
            }
        ];

        let vertex_input = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(&vertex_input_bindings)
            .vertex_attribute_descriptions(&vertex_input_attributes);

        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .primitive_restart_enable(false)
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST);

        let viewports = [
            vk::Viewport{
                x: 0.0,
                y: 0.0,
                width: vulkan.surface_extent.width as _,
                height: vulkan.surface_extent.height as _,
                min_depth: 0.0,
                max_depth: 1.0
            }
        ];

        let scissors = [
            vk::Rect2D{
                offset: vk::Offset2D{
                    x: 0,
                    y: 0
                },
                extent: vulkan.surface_extent
            }
        ];

        let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(&viewports)
            .scissors(&scissors);

        let rasterization = vk::PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_clamp(0.0)
            .depth_bias_constant_factor(0.0)
            .depth_bias_slope_factor(0.0);

        let multisample = vk::PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::TYPE_1)
            .min_sample_shading(1.0)
            .alpha_to_one_enable(false)
            .alpha_to_coverage_enable(false);

        let color_blend_attachments = [
            vk::PipelineColorBlendAttachmentState{
                blend_enable: vk::FALSE,
                src_color_blend_factor: vk::BlendFactor::ONE,
                dst_color_blend_factor: vk::BlendFactor::ZERO,
                color_blend_op: vk::BlendOp::ADD,
                src_alpha_blend_factor: vk::BlendFactor::ONE,
                dst_alpha_blend_factor: vk::BlendFactor::ZERO,
                alpha_blend_op: vk::BlendOp::ADD,
                color_write_mask: vk::ColorComponentFlags::all()
            }
        ];

        let color_blend = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::COPY)
            .attachments(&color_blend_attachments)
            .blend_constants([0.0,0.0,0.0,0.0]);

        // let dynamic_states =

        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::builder();
        let pipeline_layout = unsafe {
            vulkan.device.create_pipeline_layout(&pipeline_layout_info,Option::None)
        }?;

        let pipeline_infos = [vk::GraphicsPipelineCreateInfo::builder()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterization)
            .multisample_state(&multisample)
            .color_blend_state(&color_blend)
            // .dynamic_state()
            .layout(pipeline_layout)
            .render_pass(render_pass)
            .subpass(0)
            .build()
        ];
        let pipelines = unsafe {
            vulkan.device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                &pipeline_infos,
                Option::None
            )
        }.map_err(|(_,res)|res)?;

        unsafe { vulkan.device.destroy_shader_module(vs,Option::None) };
        unsafe { vulkan.device.destroy_shader_module(fs,Option::None) };

        let mut framebuffers = vec![];
        for view in vulkan.swapchain_image_views.iter() {
            let attachments = [*view];
            let framebuffers_info = vk::FramebufferCreateInfo::builder()
                .attachments(&attachments)
                .render_pass(render_pass)
                .width(vulkan.surface_extent.width)
                .height(vulkan.surface_extent.height)
                .layers(1);
            let framebuffer = unsafe {
                vulkan.device.create_framebuffer(&framebuffers_info,Option::None)
            }?;
            framebuffers.push(framebuffer);
        }

        let frame_count = framebuffers.len();
        let command_buffer_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(vulkan.command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(frame_count as _);
        let command_buffers = unsafe {
            vulkan.device.allocate_command_buffers(&command_buffer_info)
        }?;

        let semaphore_info = vk::SemaphoreCreateInfo::default();
        let image_available_semaphore = unsafe {
            vulkan.device.create_semaphore(&semaphore_info,Option::None)
        }?;
        let render_finish_semaphore = unsafe {
            vulkan.device.create_semaphore(&semaphore_info,Option::None)
        }?;


        Ok(Renderer{
            vulkan,
            render_passes: vec![render_pass],
            pipeline_layouts: vec![pipeline_layout],
            pipelines,
            framebuffers,
            command_buffers,
            need_update_commands : vec![true;frame_count],
            image_available_semaphore,
            render_finish_semaphore,
            clear_color : glm::make_vec3(&[0.0,0.0,0.0]),
            sprite_vertices: vec![],
            sprite_vertex_buffer: vk::Buffer::null(),
            sprite_memory: vk::DeviceMemory::null()
        })
    }

    pub fn vulkan_context(&self) -> Arc<Vulkan> {
        self.vulkan.clone()
    }

    pub fn clear_color(&mut self,r : f32,g : f32,b : f32) {
        self.clear_color = glm::make_vec3(&[r,g,b])
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe { self.vulkan.device.destroy_buffer(self.sprite_vertex_buffer,Option::None) }
        unsafe { self.vulkan.device.free_memory(self.sprite_memory,Option::None) }
        unsafe { self.vulkan.device.destroy_semaphore(self.render_finish_semaphore,Option::None) }
        unsafe { self.vulkan.device.destroy_semaphore(self.image_available_semaphore,Option::None) }
        unsafe {
            self.vulkan.device.free_command_buffers(
                self.vulkan.command_pool,
                self.command_buffers.as_slice()
            )
        }
        for framebuffer in &self.framebuffers {
            unsafe { self.vulkan.device.destroy_framebuffer(*framebuffer,Option::None) }
        }
        for pipeline in &self.pipelines {
            unsafe { self.vulkan.device.destroy_pipeline(*pipeline,Option::None) }
        }
        for pipeline_layout in &self.pipeline_layouts {
            unsafe { self.vulkan.device.destroy_pipeline_layout(*pipeline_layout,Option::None) }
        }
        for render_pass in &self.render_passes {
            unsafe { self.vulkan.device.destroy_render_pass(*render_pass, Option::None) }
        }
    }
}

