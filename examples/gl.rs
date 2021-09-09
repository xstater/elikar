extern crate elikar;
extern crate elikar_gl;
extern crate sdl2_sys;

use sdl2_sys::*;
use elikar::{Elikar, window, ElikarStates};
use elikar_gl as gl;
use std::ffi::CString;
use xecs::{System};
use std::cell::{RefMut, Ref};
use elikar::events::PollEvents;
use xecs::system::End;
use std::ptr::null_mut;

struct QuitSystem;
impl<'a> System<'a> for QuitSystem {
    type Resource = (&'a PollEvents,&'a mut ElikarStates);
    type Dependencies = PollEvents;

    fn update(&'a mut self, (events,mut states) : (Ref<'a,PollEvents>,RefMut<'a,ElikarStates>)) {
        if let Some(_) = events.quit {
            states.quit()
        }
    }
}

struct PrintFpsSystem;
impl<'a> System<'a> for PrintFpsSystem {
    type Resource = (&'a PollEvents,&'a ElikarStates);
    type Dependencies = PollEvents;

    fn update(&'a mut self,(events,states) : (Ref<'a,PollEvents>,Ref<'a,ElikarStates>)) {
        for _ in &events.mouse_motion {
            println!("fps:{}",states.fps());
        }
    }
}

struct ClearScreen;
impl<'a> System<'a> for ClearScreen {
    type Resource = ();
    type Dependencies = ();

    fn update(&'a mut self,_ : ()) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };
    }
}

struct SwapWindow;
impl<'a> System<'a> for SwapWindow {
    type Resource = &'a mut window::Manager;
    type Dependencies = End;

    fn update(&'a mut self,mut window_manager : RefMut<'a,window::Manager>) {
        for window in window_manager.iter_mut() {
            window.gl_swap();
        }
    }
}

fn main(){
    let mut game = Elikar::new().unwrap();

    let mut window_manager = game.create_window_manager();
    let mut gl_context = null_mut();
    {
        let window = window_manager.create_window()
            .title("OpenGL test")
            .opengl()
            .build()
            .unwrap();

        unsafe {
            let wptr: *mut SDL_Window = window.window_ptr();
            let gc = SDL_GL_CreateContext(wptr);
            if gc.is_null() {
                panic!("asd");
            }
            gl_context = gc;
        };

        gl::load_with(|s| unsafe {
            SDL_GL_GetProcAddress(CString::new(s).unwrap().as_ptr() as *const _)
        } as *const _);

        unsafe {
            if SDL_GL_SetSwapInterval(1) != 0 {
                panic!("SDL_GL_SetSwapInterval failed : VSYNC unsupported");
            }
        }
    }
    game.current_stage_mut()
        .add_system(window_manager)
        .add_system(QuitSystem)
        .add_system(PollEvents::new())
        .add_system(PrintFpsSystem)
        .add_system(ClearScreen)
        .add_system(SwapWindow);

    game.run();

    unsafe {SDL_GL_DeleteContext(gl_context)};
}