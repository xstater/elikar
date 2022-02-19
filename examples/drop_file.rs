use elikar::{Elikar, States, common::Spawner};
use futures::StreamExt;
use xecs::system::System;

fn main() {
    use simple_logger::SimpleLogger;
    SimpleLogger::new().init().unwrap();

    let mut game = Elikar::new().unwrap();

    game.window_builder().build().unwrap();

    let events = game.events();
    game.spawn(async move {
        let mut quit = events.on_quit();
        let world = quit.world();
        if let Some(_) = quit.next().await {
            let world = world.read();
            let mut states = world.resource_write::<States>() .unwrap();
            states.quit();
        }
    });

    let events = game.events();
    game.spawn(async move {
        let mut file = events.on_drop_file();
        while let Some(file) = file.next().await {
            println!("{:?}",file);
        }
    });

    game.run();
}
