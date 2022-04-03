use elikar::Spawner;
use futures::StreamExt;

fn main() {
    use simple_logger::SimpleLogger;
    SimpleLogger::new().init().unwrap();

    let mut game = elikar::init().unwrap();

    game.window_builder().build().unwrap();

    let events = game.events();
    game.spawn(async move {
        let mut quit = events.on_quit();
        if let Some(_) = quit.next().await {
            let world = events.elikar_world();
            world.quit();
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
