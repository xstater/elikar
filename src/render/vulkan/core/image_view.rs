use crate::render::vulkan::core::{AshRaw, Core, Image};
use ash::vk;
use std::sync::Arc;

pub struct ImageView {
    pub(in crate::render) core: Arc<Core>,
    pub(in crate::render) image_view: vk::ImageView,
}

impl ImageView {
    pub fn builder(image: &Image) -> ImageViewBuilder {
        ImageViewBuilder {
            core: image.core.clone(),
            info: vk::ImageViewCreateInfo {
                image: *image.raw(),
                view_type: vk::ImageViewType::TYPE_2D,
                format: image.format,
                components: vk::ComponentMapping {
                    r: vk::ComponentSwizzle::R,
                    g: vk::ComponentSwizzle::G,
                    b: vk::ComponentSwizzle::B,
                    a: vk::ComponentSwizzle::A,
                },
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                ..Default::default()
            },
        }
    }
}

impl AshRaw for ImageView {
    type Raw = vk::ImageView;

    fn raw(&self) -> &Self::Raw {
        &self.image_view
    }
}

impl Drop for ImageView {
    fn drop(&mut self) {
        unsafe {
            self.core
                .device
                .destroy_image_view(self.image_view, Option::None)
        }
    }
}

pub struct ImageViewBuilder {
    core: Arc<Core>,
    info: vk::ImageViewCreateInfo,
}

impl ImageViewBuilder {
    pub fn format(mut self, format: vk::Format) -> Self {
        self.info.format = format;
        self
    }

    pub fn components(mut self, swizzle: vk::ComponentMapping) -> Self {
        self.info.components = swizzle;
        self
    }

    pub fn subresource_range(mut self, range: vk::ImageSubresourceRange) -> Self {
        self.info.subresource_range = range;
        self
    }
    pub fn build(self) -> Result<ImageView, vk::Result> {
        let image_view = unsafe { self.core.device.create_image_view(&self.info, Option::None) }?;
        Ok(ImageView {
            core: self.core,
            image_view,
        })
    }
}
