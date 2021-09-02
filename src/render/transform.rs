use xrunits::angle::Radian;

#[derive(Debug,Copy, Clone,Hash,Eq, PartialEq)]
pub enum FlipType {
    None,
    Horizontal,
    Vertical
}


#[derive(Debug,Copy,Clone)]
pub struct Transform2D {
    position : (i32,i32),
    scale : (f32,f32),
    rotation : Radian,
    flip : FlipType
}

impl Transform2D{

}