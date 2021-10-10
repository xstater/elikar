use crate::render::vulkan::core::{AshRaw, Core, ImageView, RenderPass};
use ash::vk;
use std::sync::Arc;

pub struct Framebuffer {
    pub(in crate::render) core: Arc<Core>,
    pub(in crate::render) framebuffer: vk::Framebuffer,
}

impl Framebuffer {
    pub fn builder(render_pass: &RenderPass) -> FramebufferBuilder {
        FramebufferBuilder {
            views: vec![],
            render_pass: *render_pass.raw(),
            size: (0, 0),
            layers: 1,
        }
    }
}

impl AshRaw for Framebuffer {
    type Raw = vk::Framebuffer;

    fn raw(&self) -> &Self::Raw {
        &self.framebuffer
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            self.core
                .device
                .destroy_framebuffer(self.framebuffer, Option::None)
        }
    }
}

pub struct FramebufferBuilder {
    views: Vec<vk::ImageView>,
    render_pass: vk::RenderPass,
    size: (u32, u32),
    layers: u32,
}

impl FramebufferBuilder {
    pub fn view(mut self, view: &ImageView) -> Self {
        self.views.push(view.image_view);
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = (width, height);
        self
    }

    pub fn layers(mut self, layers: u32) -> Self {
        self.layers = layers;
        self
    }

    pub(in crate::render) fn build(self, core: Arc<Core>) -> Result<Framebuffer, vk::Result> {
        let info = vk::FramebufferCreateInfo::builder()
            .render_pass(self.render_pass)
            .attachments(&self.views)
            .height(self.size.1)
            .width(self.size.0)
            .layers(self.layers);
        let framebuffer = unsafe { core.device.create_framebuffer(&info, Option::None) }?;
        Ok(Framebuffer { core, framebuffer })
    }
}
