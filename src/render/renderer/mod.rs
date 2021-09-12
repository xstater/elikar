pub mod system;

use crate::render::vulkan::Vulkan;
use std::sync::Arc;

pub struct Renderer{
    vulkan : Arc<Vulkan>,
}

impl Renderer {
    pub fn new(vulkan : Arc<Vulkan>) -> Renderer {
        Renderer{
            vulkan
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
    }
}

