extern crate elikar;

use elikar::{Elikar, window, system_event};

fn main() {
    let mut game = Elikar::new().unwrap();

    let wm = window::Manager::new();
    let _window = wm.builder()
        .title("drop file here")
        .opengl()
        .build()
        .unwrap();

    let mut signals = system_event::Signals::new();

    let mut game_closure = game.clone();
    signals.quit.connect(move |()| {
        game_closure.quit();
    });
    signals.drop_files.connect(|info|{
        println!("File:{:?}",info);
    });

    game.run(signals);
}