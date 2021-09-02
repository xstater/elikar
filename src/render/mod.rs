pub mod transform;
pub mod gl;
pub mod renderer2d;

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum VSyncType {
    None,
    VSync,
    AdaptiveVSync
}

pub trait RendererContext : Sized{
    fn drawable_size(&self) -> (u32,u32);
    fn vsync(&self) -> VSyncType;
    fn set_vsync(&mut self,vsync_type : VSyncType);
}

