extern crate elikar;
extern crate elikar_gl;
extern crate sdl2_sys;

use sdl2_sys::*;
use elikar::{Elikar, window, system_event};
use elikar_gl as gl;
use std::ffi::CString;

fn main(){
    let mut game = Elikar::new().unwrap();

    let wm = window::Manager::new();
    let window = wm.builder()
        .title("OpenGL test")
        .opengl()
        .build()
        .unwrap();

    let gl_context = unsafe {
        let wptr : *mut SDL_Window = window.raw_window();
        let gc = SDL_GL_CreateContext(wptr);
        if gc.is_null() {
            panic!("asd");
        }
        gc
    };

    gl::load_with(|s| unsafe{
        SDL_GL_GetProcAddress(CString::new(s).unwrap().as_ptr()as *const _)
    } as *const _);

    unsafe{
        if SDL_GL_SetSwapInterval(1) != 0 {
            panic!("SDL_GL_SetSwapInterval failed : VSYNC unsupported");
        }
    }

    let mut sigs = system_event::Signals::new();
    let mut game_closure = game.clone();
    sigs.quit.connect(move |()|{
        game_closure.quit();
    });
    let game_closure = game.clone();
    sigs.key_down.connect(move |_|{
       println!("frame_duration:{}us,fps:{},fis:{}",
                game_closure.frame_duration().as_micros(),
                game_closure.fps(),
                game_closure.fis());
    });
    let mut window_closure = window.clone();
    sigs.enter_frame.connect(|()| {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };
    });
    sigs.leave_frame.connect(move |()|{
        window_closure.gl_swap().unwrap();
    });
    game.run(sigs);

    unsafe {SDL_GL_DeleteContext(gl_context)};
}