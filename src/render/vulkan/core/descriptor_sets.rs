use std::sync::Arc;
use crate::render::vulkan::core::{Core, AshRaw, ImageView};
use ash::vk;
use crate::render::vulkan::core::sampler::Sampler;

pub struct DescriptorSets {
    pub(in crate::render) core : Arc<Core>,
    pub(in crate::render) descriptor_sets : Vec<vk::DescriptorSet>
}

impl DescriptorSets {
    pub fn update_combined_image_sampler(&mut self,index : usize,binding : u32,sampler : &Sampler,view : &ImageView,layout : vk::ImageLayout){
        let image_info = [vk::DescriptorImageInfo{
            sampler: sampler.sampler,
            image_view: view.image_view,
            image_layout: layout
        }];
        let write = [vk::WriteDescriptorSet::builder()
            .dst_binding(binding)
            .dst_set(self.descriptor_sets[index])
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .image_info(&image_info)
            .build()];
        unsafe {
            self.core.device.update_descriptor_sets(&write,&[]);
        }
    }

    pub(in crate::render) fn allocate(core : Arc<Core>,layouts : &[vk::DescriptorSetLayout]) -> Result<DescriptorSets,vk::Result> {
        let info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(core.descriptor_pool)
            .set_layouts(&layouts);
        let descriptor_sets = unsafe {
            core.device.allocate_descriptor_sets(&info)
        }?;
        Ok(DescriptorSets{
            core,
            descriptor_sets
        })
    }
}

impl AshRaw for DescriptorSets {
    type Raw = [vk::DescriptorSet];

    fn raw(&self) -> &Self::Raw {
        &self.descriptor_sets
    }
}

impl Drop for DescriptorSets {
    fn drop(&mut self) {
        unsafe {
            self.core.device.free_descriptor_sets(self.core.descriptor_pool,&self.descriptor_sets).unwrap();
        }
    }
}
