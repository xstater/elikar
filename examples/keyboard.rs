use elikar::{Elikar, common::Spawner, keyboard::Code};
use futures::StreamExt;

fn main() {
    let mut game = Elikar::new().unwrap();

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

    game.run();
}
