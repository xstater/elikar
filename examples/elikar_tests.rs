extern crate elikar;

use elikar::elikar::Elikar;
use elikar::window;
use elikar::system_event::Handlers;

fn main(){
    let game = Elikar::new().unwrap();

    let mut wm = window::Manager::new();
    let window = wm.builder()
        .title("asdasd")
        .position_centred()
        .opengl()
        .size(1280,800)
        .build()
        .unwrap();
    wm.add_windows(window);

    let mut event_handlers = Handlers::new();

    event_handlers.mouse_button_down.connect(|(x,y)|{
        println!("({},{})",x,y);
    });
    let mut game_closure = game.clone();
    let _cnt1 = event_handlers.mouse_button_down.connect(move |(x,y)|{
        if x < 100 && y < 100 {
            game_closure.quit();
        }
    });

    elikar::elikar::run(game,event_handlers);
}