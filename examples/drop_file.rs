use elikar::{Elikar, States};
use futures::StreamExt;
use xecs::system::System;

fn main() {
    let mut game = Elikar::new().unwrap();

    game.window_builder().build().unwrap();

    let events = game.events();
    game.spawn(async move {
        let mut quit = events.on_quit();
        let world = quit.world();
        if let Some(_) = quit.next().await {
            world.read().unwrap()
                .resource_mut::<States>()
                .unwrap()
                .quit();
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
