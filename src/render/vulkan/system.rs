use xecs::System;
use xecs::resource::Resource;

pub struct RebuildSwapchain;

impl<'a> System<'a> for RebuildSwapchain {
    type Resource = ();
    type Dependencies = ();

    fn update(&'a mut self, resource: <Self::Resource as Resource<'a>>::Type) {
        unimplemented!()
    }
}