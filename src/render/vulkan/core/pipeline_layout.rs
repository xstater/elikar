use crate::render::vulkan::core::{AshRaw, Core, DescriptorSetLayout};
use ash::vk;
use std::sync::Arc;

pub struct PipelineLayout {
    pub(in crate::render) core: Arc<Core>,
    pub(in crate::render) pipeline_layout: vk::PipelineLayout,
}

impl PipelineLayout {
    pub fn builder() -> PipelineLayoutBuilder {
        PipelineLayoutBuilder {
            push_constant_ranges: vec![],
            set_layouts: vec![],
        }
    }
}

impl AshRaw for PipelineLayout {
    type Raw = vk::PipelineLayout;

    fn raw(&self) -> &Self::Raw {
        &self.pipeline_layout
    }
}

impl Drop for PipelineLayout {
    fn drop(&mut self) {
        unsafe {
            self.core
                .device
                .destroy_pipeline_layout(self.pipeline_layout, Option::None);
        }
    }
}

pub struct PipelineLayoutBuilder {
    push_constant_ranges: Vec<vk::PushConstantRange>,
    set_layouts: Vec<vk::DescriptorSetLayout>,
}

impl PipelineLayoutBuilder {
    pub fn push_constant(
        mut self,
        shader_stage: vk::ShaderStageFlags,
        size: u32,
        offset: u32,
    ) -> Self {
        self.push_constant_ranges.push(vk::PushConstantRange {
            stage_flags: shader_stage,
            offset,
            size,
        });
        self
    }

    pub fn descriptor_set_layout(mut self, descriptor_set_layout: &DescriptorSetLayout) -> Self {
        self.set_layouts
            .push(descriptor_set_layout.descriptor_layout);
        self
    }

    pub(in crate::render) fn build(self, core: Arc<Core>) -> Result<PipelineLayout, vk::Result> {
        let info = vk::PipelineLayoutCreateInfo::builder()
            .push_constant_ranges(&self.push_constant_ranges)
            .set_layouts(&self.set_layouts);
        let pipeline_layout = unsafe { core.device.create_pipeline_layout(&info, Option::None) }?;

        Ok(PipelineLayout {
            core,
            pipeline_layout,
        })
    }
}
