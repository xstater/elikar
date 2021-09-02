extern crate elikar;
extern crate elikar_gl;
extern crate sdl2_sys;

use sdl2_sys::*;
use elikar::{Elikar, window, ElikarStates};
use elikar_gl as gl;
use std::ffi::CString;
use xecs::{System, World};
use elikar::window::Window;
use std::cell::{RefMut, Ref};
use elikar::events::PollEvents;
use xecs::resource::Resource;
use xecs::system::End;

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
        if let Some(_motion) = &events.mouse_motion {
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
    type Resource = &'a mut World;
    type Dependencies = End;

    fn update(&'a mut self, world : RefMut<'a,World>) {
        for window in world.query::<&mut Window>() {
            window.gl_swap();
        }
    }
}

fn main(){
    let mut game = Elikar::new().unwrap();

    let window = window::Builder::default()
        .title("OpenGL test")
        .opengl()
        .build()
        .unwrap();

    let gl_context = unsafe {
        let wptr : *mut SDL_Window = window.window_ptr();
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

    {
        let mut world = game.current_stage_mut().world_mut();
        world.register::<Window>();
        world.create_entity()
            .attach(window);
    }

    game.current_stage_mut()
        .add_system(QuitSystem)
        .add_system(PollEvents::new())
        .add_system(PrintFpsSystem);
    game.current_stage_mut()
        .add_system(ClearScreen)
        .add_system(SwapWindow);

    game.run();

    unsafe {SDL_GL_DeleteContext(gl_context)};
}