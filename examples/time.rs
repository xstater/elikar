use std::time::Duration;
use elikar::common::Spawner;
use futures::StreamExt;

fn main() {
    let mut game = elikar::init().unwrap();

    game.window_builder()
        .title("time")
        .build()
        .unwrap();

    let events = game.events();
    game.spawn_local(async move {
        let mut quit = events.on_quit();
        while let Some(_) = quit.next().await{
            events.elikar_world().quit();
        }
    });

    let handle = game.spawn(async {
        let mut timer = tokio::time::interval(Duration::from_secs(1));
        let mut count = 0;
        loop {
            timer.tick().await;
            count += 1;
            println!("{}s",count);
        }
    });

    let events = game.events();
    game.spawn(async move {
        let mut mouse_down = events.on_mouse_down();
        if let Some(_) = mouse_down.next().await {
            println!("Timer stop");
            handle.abort();
        }
    });

    let handle = game.spawn_local(async move {
        let timer = tokio::time::sleep(Duration::from_secs(2));
        timer.await;
    });

    game.spawn_local(async move {
        handle.await;
        println!("Time out!")
    });
    
    game.run();
}
