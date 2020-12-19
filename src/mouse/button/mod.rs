pub mod event;

extern crate sdl2_sys;

#[derive(Debug,Copy,Clone,PartialOrd,PartialEq)]
pub enum Button{
    Left,
    Middle,
    Right,
    X1,
    X2
}

#[derive(Debug,Copy,Clone,PartialOrd,PartialEq)]
pub enum Clicks{
    Single,
    Double
}

pub struct ButtonState(u32);

const LEFT_MASK : u32 = 0x01;
const MIDDLE_MASK : u32 = 0x02;
const RIGHT_MASK : u32 = 0x04;
const X1_MASK : u32 = 0x08;
const X2_MASK : u32 = 0x10;

impl ButtonState {
    pub(in crate::mouse) fn new(value : u32) -> ButtonState{
        ButtonState(value)
    }

    pub fn is_left(&self) -> bool{
        self.0 & LEFT_MASK == LEFT_MASK
    }
    pub fn is_right(&self) -> bool{
        self.0 & RIGHT_MASK == RIGHT_MASK
    }
    pub fn is_middle(&self) -> bool{
        self.0 & MIDDLE_MASK == MIDDLE_MASK
    }
    pub fn is_x1(&self) -> bool{
        self.0 & X1_MASK == X1_MASK
    }
    pub fn is_x2(&self) -> bool{
        self.0 & X2_MASK == X2_MASK
    }
}
