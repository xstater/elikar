use xrsignal::Signal;

pub struct Handlers{
    pub mouse_button_down : Signal<(i32,i32),()>,
    pub mouse_button_up : Signal<(i32,i32),()>,
    pub mouse_motion : Signal<(i32,i32),()>,

}

impl Handlers{
    pub fn new() -> Handlers{
        Handlers{
            mouse_button_down : Signal::new(),
            mouse_button_up : Signal::new(),
            mouse_motion : Signal::new(),
        }
    }
}