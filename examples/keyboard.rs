extern crate elikar;

use elikar::{Elikar, window, system_event};
use elikar::keyboard::Code;

fn main(){
    let mut game = Elikar::new().unwrap();

    let wm = window::Manager::new();
    let window = wm.builder()
        .title("keyboard test")
        .opengl()
        .build()
        .unwrap();

    let mut signals = system_event::Signals::new();

    let mut game_closure = game.clone();
    signals.key_down.connect(move |info|{
        if info.code == Code::Escape{
            game_closure.quit()
        }
    });
    signals.key_down.connect(|info|{
        println!("info {:?}",info);
    });

    game.run(signals);
}