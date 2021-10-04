use std::ffi::CString;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use ash::{util, vk};
use crate::render::vulkan::core::{AshRaw, Core, PipelineLayout, RenderPass, Shader};

pub struct Pipeline {
    pub(in crate::render) core : Arc<Core>,
    pub(in crate::render) pipeline : vk::Pipeline
}

impl Pipeline {
    pub fn builder() -> PipelineBuilder{
        PipelineBuilder{
            shaders: vec![],
            vertex_input_bindings: vec![],
            vertex_input_attributes: vec![],
            vertex_input_assembly: Default::default(),
            viewports: vec![],
            scissors: vec![],
            rasterization: vk::PipelineRasterizationStateCreateInfo{
                depth_clamp_enable: vk::FALSE,
                rasterizer_discard_enable: vk::FALSE,
                polygon_mode: vk::PolygonMode::FILL,
                cull_mode: vk::CullModeFlags::NONE,
                front_face: vk::FrontFace::CLOCKWISE,
                line_width: 1.0,
                .. Default::default()
            },
            multisample: vk::PipelineMultisampleStateCreateInfo{
                rasterization_samples: vk::SampleCountFlags::TYPE_1,
                sample_shading_enable: vk::FALSE,
                .. Default::default()
            },
            color_blend_attachments: vec![],
            color_blend_state: Default::default(),
            pipeline_layout: vk::PipelineLayout::null(),
            render_pass: vk::RenderPass::null(),
            subpass: 0,
        }
    }
}

impl AshRaw for Pipeline {
    type Raw = vk::Pipeline;

    fn raw(&self) -> &Self::Raw {
        &self.pipeline
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            self.core.device.destroy_pipeline(self.pipeline,Option::None)
        }
    }
}

pub struct PipelineBuilder {
    shaders : Vec<(vk::ShaderStageFlags,PathBuf)>,
    vertex_input_bindings : Vec<vk::VertexInputBindingDescription>,
    vertex_input_attributes : Vec<vk::VertexInputAttributeDescription>,
    vertex_input_assembly : vk::PipelineInputAssemblyStateCreateInfo,
    viewports : Vec<vk::Viewport>,
    scissors : Vec<vk::Rect2D>,
    rasterization : vk::PipelineRasterizationStateCreateInfo,
    multisample : vk::PipelineMultisampleStateCreateInfo,
    color_blend_attachments : Vec<vk::PipelineColorBlendAttachmentState>,
    color_blend_state : ColorBlendState,
    pipeline_layout : vk::PipelineLayout,
    render_pass : vk::RenderPass,
    subpass : u32,
}

#[derive(Debug,Copy,Clone,Default)]
struct ColorBlendState{
    enable_logic_op : bool,
    logic_op : vk::LogicOp,
    blend_constant : [f32;4]
}

impl PipelineBuilder {
    pub fn shader_from_file<P : AsRef<Path>>(mut self,shader_type : vk::ShaderStageFlags,path : P) -> Self {
        self.shaders.push((shader_type,path.as_ref().to_path_buf()));
        self
    }

    pub fn input_binding<V>(mut self,binding_index : u32,input_rate : vk::VertexInputRate) -> Self{
        self.vertex_input_bindings.push(vk::VertexInputBindingDescription{
            binding: binding_index,
            stride: std::mem::size_of::<V>() as _,
            input_rate
        });
        self
    }

    pub fn input_attribute(mut self,location : u32,binding : u32,format : vk::Format,offset : usize) -> Self {
        self.vertex_input_attributes.push(vk::VertexInputAttributeDescription{
            location,
            binding,
            format,
            offset : offset as _
        });
        self
    }

    pub fn input_assembly(mut self,enable_primitive_restart : bool,topology : vk::PrimitiveTopology) -> Self {
        self.vertex_input_assembly = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(topology)
            .primitive_restart_enable(enable_primitive_restart)
            .build();
        self
    }

    pub fn viewport(mut self,x : f32,y : f32,width : f32,height : f32,min_depth : f32,max_depth : f32) -> Self {
        self.viewports.push(vk::Viewport{
            x, y,
            width, height,
            min_depth, max_depth
        });
        self
    }

    pub fn scissor(mut self,x : i32,y : i32,width : u32,height : u32) -> Self {
        self.scissors.push(vk::Rect2D{
            offset: vk::Offset2D{ x, y },
            extent: vk::Extent2D{ width, height }
        });
        self
    }

    #[allow(unused_mut)]
    pub fn with_surface_area(mut self,area : &vk::Extent2D) -> Self {
        self.viewport(0.0,0.0,area.width as _,area.height as _,0.0,1.0)
            .scissor(0,0,area.width,area.height)
    }

    pub fn rasterization(mut self,info : vk::PipelineRasterizationStateCreateInfo) -> Self {
        self.rasterization = info;
        self
    }

    pub fn multisample(mut self,info : vk::PipelineMultisampleStateCreateInfo) -> Self {
        self.multisample = info;
        self
    }

    pub fn color_blend_attachment(mut self,attachment : vk::PipelineColorBlendAttachmentState) -> Self {
        self.color_blend_attachments.push(attachment);
        self
    }

    pub fn color_blend(mut self,enable_logic_op : bool,logic_op : vk::LogicOp,blend_constant : [f32;4]) -> Self{
        self.color_blend_state = ColorBlendState {
            enable_logic_op,
            logic_op,
            blend_constant
        };
        self
    }

    pub fn pipeline_layout(mut self,layout : &PipelineLayout) -> Self {
        self.pipeline_layout = layout.pipeline_layout;
        self
    }

    pub fn render_pass(mut self,render_pass : &RenderPass) -> Self {
        self.render_pass = render_pass.render_pass;
        self
    }

    pub fn subpass(mut self,subpass : u32) -> Self {
        self.subpass = subpass;
        self
    }

    pub(in crate::render) fn build(self,core : Arc<Core>) -> Result<Pipeline,CreatePipelineError>{
        let mut shaders = vec![];
        for (shader_type,path) in self.shaders.iter() {
            let mut file = File::open(path)?;
            let code = util::read_spv(&mut file)?;

            let info = vk::ShaderModuleCreateInfo::builder()
                .code(&code);
            let shader_module = unsafe {
                core.device.create_shader_module(&info,Option::None)?
            };
            shaders.push((*shader_type,Shader::new(core.clone(),shader_module)));
        }
        let function_name = CString::new("main").unwrap();
        let shader_stages = shaders.iter()
            .map(|(shader_type,shader)|{
                vk::PipelineShaderStageCreateInfo{
                    stage: *shader_type,
                    module: *shader.raw(),
                    p_name: function_name.as_ptr(),
                    .. Default::default()
                }
            }).collect::<Vec<_>>();

        let input_state = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(&self.vertex_input_bindings)
            .vertex_attribute_descriptions(&self.vertex_input_attributes);

        let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(&self.viewports)
            .scissors(&self.scissors);

        let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op(self.color_blend_state.logic_op)
            .logic_op_enable(self.color_blend_state.enable_logic_op)
            .blend_constants(self.color_blend_state.blend_constant)
            .attachments(&self.color_blend_attachments);

        let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .render_pass(self.render_pass)
            .layout(self.pipeline_layout)
            // .depth_stencil_state()
            .stages(&shader_stages)
            .vertex_input_state(&input_state)
            .input_assembly_state(&self.vertex_input_assembly)
            .viewport_state(&viewport_state)
            .color_blend_state(&color_blend_state)
            .rasterization_state(&self.rasterization)
            .multisample_state(&self.multisample)
            // .dynamic_state()
            .subpass(self.subpass);

        let pipelines = unsafe {
            core.device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                &[pipeline_info.build()],
                Option::None)
        }.map_err(|(_,err)|err)?;
        let pipeline = pipelines.into_iter().next().unwrap();

        Ok(Pipeline{
            core,
            pipeline
        })
    }
}

#[derive(Debug)]
pub enum CreatePipelineError {
    VulkanError(vk::Result),
    IoError(io::Error),
}

impl From<vk::Result> for CreatePipelineError {
    fn from(vk_err: vk::Result) -> Self {
        CreatePipelineError::VulkanError(vk_err)
    }
}

impl From<io::Error> for CreatePipelineError {
    fn from(io_err: io::Error) -> Self {
        CreatePipelineError::IoError(io_err)
    }
}

impl Display for CreatePipelineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CreatePipelineError::VulkanError(vk_err) =>
                write!(f,"Create pipeline failed! vulkan error:{}",vk_err),
            CreatePipelineError::IoError(io_err) =>
                write!(f,"Create pipeline failed! IO error:{}",io_err),
        }
    }
}

impl std::error::Error for CreatePipelineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CreatePipelineError::VulkanError(vk_err) => Some(vk_err),
            CreatePipelineError::IoError(io_err) => Some(io_err),
        }
    }
}

