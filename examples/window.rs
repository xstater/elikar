use elikar::common::Spawner;
use futures::StreamExt;

fn main() {
    let mut game = elikar::init().unwrap();

    let _window_id = game.window_builder()
        .resizable()
        .always_on_top()
        .skip_taskbar()
        .title("window event test")
        .build()
        .unwrap();

    let events = game.events();
    game.spawn(async move{
        let mut quit = events.on_quit();
        if let Some(_) = quit.next().await {
            println!("Quit");
            events.elikar_world().quit();
        }
    });

    let events = game.events();
    game.spawn(async move {
        let mut window_events = events.on_window_events();
        while let Some(event) = window_events.next().await {
            println!("{:?}",event);
        }
    });

    game.run();
}
