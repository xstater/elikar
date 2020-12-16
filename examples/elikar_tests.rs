extern crate elikar;

use elikar::elikar::Elikar;
use elikar::event;
use elikar::window;

fn main(){
    let mut game = Elikar::new().unwrap();

    let mut wm = window::Manager::new();
    let window = wm.builder()
        .title("asdasd")
        .position_centred()
        .opengl()
        .size(1280,800)
        .build()
        .unwrap();
    wm.add_windows(window);

    game.event_handlers.mouse_button_down.connect(|(x,y)|println!("({},{})",x,y));
    game.event_handlers.mouse_button_down.connect(|(x,y)|{
        if x < 100 && y < 100 {
            println!("exit");
        }
    });

    game.run();
}