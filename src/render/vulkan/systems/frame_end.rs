use std::cell::RefMut;

use ash::vk;
use xecs::{End, System};
use crate::render::vulkan::Vulkan;

pub struct FrameEnd{}

impl FrameEnd {
    pub fn new() -> FrameEnd {
        FrameEnd{

        }
    }
}

impl<'a> System<'a> for FrameEnd {
    type InitResource = ();
    type Resource = &'a mut Vulkan;
    type Dependencies = End;
    type Error = vk::Result;

    fn update(&'a mut self,mut vulkan : RefMut<'a,Vulkan>) -> Result<(),Self::Error>{
        vulkan.present_queue()?;
        Ok(())
    }

}
