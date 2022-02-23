mod builder;
pub mod events;
mod window;
mod raw_handle;

pub use builder::Builder;
pub use window::Window;

use xecs::{entity::EntityId, query::WithId, world::World};

pub(in crate) fn find_window(world : &World,window_id : u32) -> Option<EntityId> {
    world.query::<&Window>().with_id()
        .find(|(_,window)|window.id() == window_id)
        .map(|(id,_)|id)
}
