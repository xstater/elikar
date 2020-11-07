

#[derive(Debug,PartialOrd,PartialEq)]
pub enum Button{
    Left,
    Middle,
    Right,
    X1,
    X2
}

#[derive(Debug,PartialOrd,PartialEq)]
pub enum Clicks{
    Single,
    Double
}

#[derive(Debug)]
pub struct ButtonInfo{
    pub timestamp : u32,
    pub window_id : u32,
    pub button : Button,
    pub click : Clicks,
    pub position : (i32,i32)
}
