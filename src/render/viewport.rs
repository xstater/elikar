
#[derive(Debug,Copy,Clone)]
pub enum Viewport {
    FixedSize(u32,u32,u32,u32),
    FollowWindow(f32,f32,f32,f32)
}
