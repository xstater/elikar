extern crate elikar;

use elikar::{Elikar, system_event, window, mouse};
use elikar::mouse::cursor::{Cursor, SystemCursor};

fn main(){
    let mut game = Elikar::new().unwrap();

    let mut event = system_event::Signals::new();

    let wm = window::Manager::new();
    let _window = wm.builder()
        .title("mouse test")
        .position_centred()
        .opengl()
        .build()
        .unwrap();

    let mut game_closure = game.clone();
    event.quit.connect(move|_|{
        game_closure.quit();
    });
    // event.mouse_button_down.connect(|_|{
    //     if mouse::cursor::is_visible() {
    //         mouse::cursor::hide()
    //     }else{
    //         mouse::cursor::show()
    //     }
    // });
    event.mouse_button_down.connect(|_|{
        let (x,y) = mouse::global_position();
        println!("global_position:({},{})",x,y);
        mouse::warp_global(100,100).unwrap();
    });
    event.mouse_wheel.connect(|info|{
        let (x,y) = info.scrolled;
        println!("wheel:({},{})",x,y);
    });

    let mut cursor = Cursor::system(SystemCursor::SizeAll).unwrap();
    cursor.set_as_cursor();

    game.run(event);
}