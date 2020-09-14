#[cfg(test)]
mod tests {
    #[test]
    fn sdl_test() {
        extern crate sdl2;

        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let mut window = video_subsystem
            .window("test",1280,768)
            .opengl()
            .build()
            .unwrap();
        window.show();

        let mut event_pump = sdl_context
            .event_pump()
            .unwrap();

        'mainloop: loop{
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode : Some(Keycode::Escape),..} => {
                        break 'mainloop;
                    }
                    _ => {}
                }
            }
        }

    }
}
