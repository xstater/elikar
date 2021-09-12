use crate::render::renderer::Renderer;
use xecs::{System, World, EntityId};
use xecs::system::End;
use std::cell::Ref;
use crate::render::transform::Transform;

impl<'a> System<'a> for Renderer {
    type Resource = &'a World;
    type Dependencies = End;

    fn update(&'a mut self, world : Ref<'a,World>) {

        // render 2d scene
        // for (camera,camera_transform) in world.query::<(&Camera2d,&Transform)>() {
        //     // for IDE auto-complete
        //     let camera : &Camera2d = camera;
        //     let camera_transform : &Transform = camera_transform;
        //
        //
        // }
    }
}