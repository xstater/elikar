use crate::render::vulkan::core::{AshRaw, Core};
use ash::vk;
use std::ptr::null;
use std::sync::Arc;

pub struct DescriptorSetLayout {
    pub(in crate::render) core: Arc<Core>,
    pub(in crate::render) descriptor_layout: vk::DescriptorSetLayout,
}

impl DescriptorSetLayout {
    pub fn builder() -> DescriptorSetLayoutBuilder {
        DescriptorSetLayoutBuilder { bindings: vec![] }
    }
}

impl AshRaw for DescriptorSetLayout {
    type Raw = vk::DescriptorSetLayout;

    fn raw(&self) -> &Self::Raw {
        &self.descriptor_layout
    }
}

impl Drop for DescriptorSetLayout {
    fn drop(&mut self) {
        unsafe {
            self.core
                .device
                .destroy_descriptor_set_layout(self.descriptor_layout, Option::None)
        }
    }
}

pub struct DescriptorSetLayoutBuilder {
    bindings: Vec<vk::DescriptorSetLayoutBinding>,
}

impl DescriptorSetLayoutBuilder {
    pub fn binding(
        mut self,
        binding: u32,
        descriptor_count: u32,
        descriptor_type: vk::DescriptorType,
        stage: vk::ShaderStageFlags,
    ) -> Self {
        self.bindings.push(vk::DescriptorSetLayoutBinding {
            binding,
            descriptor_type,
            descriptor_count,
            stage_flags: stage,
            p_immutable_samplers: null(),
        });
        self
    }

    pub(in crate::render) fn build(
        self,
        core: Arc<Core>,
    ) -> Result<DescriptorSetLayout, vk::Result> {
        let info = vk::DescriptorSetLayoutCreateInfo::builder().bindings(&self.bindings);
        let layout = unsafe {
            core.device
                .create_descriptor_set_layout(&info, Option::None)
        }?;
        Ok(DescriptorSetLayout {
            core,
            descriptor_layout: layout,
        })
    }
}
