use elikar::{common::Spawner, keyboard::Code};
use futures::StreamExt;

fn main() {
    let mut game = elikar::init().unwrap();

    game.window_builder().build().unwrap();
    
    let events = game.events();
    game.spawn(async move {
        let mut key_down = events.on_key_down();
        while let Some(key) = key_down.next().await {
            println!("{:?}",key);
            if key.code == Code::Escape {
                let world = events.elikar_world();
                world.quit();
                break;
            }
        }
    });

    let events = game.events();
    game.spawn(async move{
        let mut update = events.on_update();
        while let Some(_) = update.next().await {
            let world = events.elikar_world();
            let keyboard = world.keyboard();
            if keyboard.pressed(Code::Return) {
                println!("Return Pressed!");
            }
            if keyboard.all_pressed(&[Code::Q,Code::E]) {
                println!("Q&E all pressed");
            }
        }
    });

    game.run();
}
