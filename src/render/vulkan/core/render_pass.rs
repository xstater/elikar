use std::sync::Arc;
use crate::render::vulkan::core::{Core, AshRaw};
use ash::vk;

pub struct RenderPass {
    pub(in crate::render) core : Arc<Core>,
    pub(in crate::render) render_pass : vk::RenderPass
}

impl RenderPass {
    pub fn builder() -> RenderPassBuilder {
        RenderPassBuilder{
            attachments: vec![],
            dependencies: vec![],
            subpasses: vec![]
        }
    }
}

impl AshRaw for RenderPass {
    type Raw = vk::RenderPass;

    fn raw(&self) -> &Self::Raw {
        &self.render_pass
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe {
            self.core.device.destroy_render_pass(self.render_pass,Option::None);
        }
    }
}

pub struct RenderPassBuilder {
    attachments : Vec<vk::AttachmentDescription>,
    dependencies : Vec<vk::SubpassDependency>,
    subpasses : Vec<vk::SubpassDescription>
}

impl RenderPassBuilder {
    pub fn attachment(mut self,desc : vk::AttachmentDescription) -> Self {
        self.attachments.push(desc);
        self
    }

    pub fn subpass(mut self,dep : vk::SubpassDependency,desc : vk::SubpassDescription) -> Self {
        self.dependencies.push(dep);
        self.subpasses.push(desc);
        self
    }

    pub(in crate::render) fn build(self,core : Arc<Core>) -> Result<RenderPass,vk::Result> {
        let info = vk::RenderPassCreateInfo::builder()
            .attachments(&self.attachments)
            .dependencies(&self.dependencies)
            .subpasses(&self.subpasses);
        let render_pass = unsafe {
            core.device.create_render_pass(&info,Option::None)
        }?;
        Ok(RenderPass{
            core,
            render_pass
        })
    }
}