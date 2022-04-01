use std::thread::current;
use elikar::{Elikar, common::Spawner};
use futures::stream::StreamExt;

fn main() {
    // use simple_logger::SimpleLogger;
    // SimpleLogger::new().init().unwrap();

    let mut game = Elikar::new().unwrap();

    println!("main thread:{:?}",current().id());

    game.window_builder()
        .build()
        .unwrap();

    let events = game.events(); 
    game.spawn_local(async move{
        let mut quit = events.on_quit();
        if let Some(_) = quit.next().await {
            let world = events.elikar_world();
            world.quit_mut().quit();
        }
    });

    let events = game.events(); 
    game.spawn_local(async move{
        let mut mouse_down = events.on_mouse_down();
        while let Some(button) = mouse_down.next().await {
            println!("button down:{:?}",button);
            let world = events.elikar_world();
            let time = world.time();
            println!("fps:{},actual_fps:{}",time.fps(),time.actual_fps());
        }
    });
    let events = game.events(); 
    game.spawn_local(async move{
        let mut motion = events.on_mouse_motion();
        while let Some(motion) = motion.next().await {
            println!("motion: {:?}",motion);
        }
    });
    let events = game.events();
    game.spawn_local(async move {
        let mut wheel = events.on_mouse_wheel();
        while let Some(wheel) = wheel.next().await {
            println!("wheel: {:?}",wheel);
        }
    });
    
    game.run();
}
