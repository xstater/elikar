extern crate sdl2_sys;

use sdl2_sys::SDL_MouseButtonEvent;

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

#[derive(Debug,Copy,Clone)]
pub struct ButtonInfo{
    pub timestamp : u32,
    pub window_id : u32,
    pub button : Button,
    pub click : Clicks,
    pub position : (i32,i32)
}

const BUTTON_ID_LEFT : u8 = 1;
const BUTTON_ID_MIDDLE : u8 = 2;
const BUTTON_ID_RIGHT : u8 = 3;
const BUTTON_ID_X1 : u8 = 4;
const BUTTON_ID_X2 : u8 = 5;

impl From<SDL_MouseButtonEvent> for ButtonInfo{
    fn from(x: SDL_MouseButtonEvent) -> Self {
        ButtonInfo{
            timestamp : x.timestamp,
            window_id : x.windowID,
            button : match x.button {
                BUTTON_ID_LEFT => Button::Left,
                BUTTON_ID_MIDDLE => Button::Middle,
                BUTTON_ID_RIGHT => Button::Right,
                BUTTON_ID_X1 => Button::X1,
                BUTTON_ID_X2 => Button::X2,
                _ => Button::Left
            },
            click : if x.clicks == 1 { Clicks::Single } else { Clicks::Double },
            position : (x.x,x.y)
        }
    }
}