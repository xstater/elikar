extern crate elikar;

use elikar::elikar::Elikar;
use elikar::window;
use elikar::system_event::Signals;

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

    let mut event_handlers = Signals::new();

    let mut game_closure = game.clone();
    event_handlers.quit.connect(move |()|{
        game_closure.quit();
    });
    let game_closure = game.clone();
    event_handlers.mouse_button_down.connect(move |(x,y)|{
        println!("Down:({},{})",x,y);
        println!("frame_duration:{}us",game_closure.frame_duration().as_micros());
        println!("fps:{}",game_closure.fps());
        println!("fis:{}",game_closure.fis());
    });
    event_handlers.mouse_button_up.connect(move |(x,y)|{
        println!("Up:({},{})",x,y);
    });
    event_handlers.mouse_motion.connect(move |(x,y)|{
        println!("Motion:({},{})",x,y);
    });
    let mut game_closure = game.clone();
    event_handlers.mouse_button_down.connect(move |(x,y)|{
        if x < 100 && y < 100 {
            game_closure.quit();
        }
    });

    game.run(event_handlers);
}