use elikar::common::SdlError;
use elikar::events::PollEvents;
use elikar::imgui::systems::{ImGuiEventSystem, ImGuiRenderer};
use elikar::imgui::ImGui;
use elikar::render::vulkan::systems::{FrameBegin, FrameEnd};
use elikar::render::vulkan::{PresentMode, Vulkan};
use elikar::window::WindowId;
use elikar::{window, Elikar, ElikarStates};
use std::cell::{Ref, RefMut};
use std::convert::Infallible;
use std::time::{Duration, Instant};
use xecs::{End, System};

struct Quit;
impl<'a> System<'a> for Quit {
    type InitResource = ();
    type Resource = (&'a PollEvents, &'a mut ElikarStates);
    type Dependencies = PollEvents;
    type Error = Infallible;

    fn update(
        &'a mut self,
        (events, mut states): (Ref<'a, PollEvents>, RefMut<'a, ElikarStates>),
    ) -> Result<(), Self::Error> {
        if let Some(_) = events.quit {
            states.quit();
        }
        Ok(())
    }
}

struct ShowFpsOnWindow{
    time : Instant,
    window : WindowId,
    title : String,
}
impl ShowFpsOnWindow {
    pub fn from_window_id(id : WindowId) -> ShowFpsOnWindow {
        ShowFpsOnWindow{
            time: Instant::now(),
            window: id,
            title : String::new()
        }
    }
}
impl<'a> System<'a> for ShowFpsOnWindow {
    type InitResource = &'a window::Manager;
    type Resource = (&'a mut window::Manager,&'a ElikarStates);
    type Dependencies = ();
    type Error = SdlError;

    fn init(&'a mut self,manager : Ref<'a,window::Manager>) -> Result<(),Self::Error>{
        self.title = manager.window_ref(self.window)
            .unwrap()
            .title();
        Ok(())
    }

    fn update(&'a mut self,(mut manager,states) : (RefMut<'a,window::Manager>,Ref<'a,ElikarStates>)) -> Result<(),Self::Error>{
        if self.time.elapsed() > Duration::from_secs(1) {
            let window = manager.window_mut(self.window).unwrap();
            let title = format!("{} fps:{}",self.title.as_str(),states.actual_fps());
            window.set_title(title.as_str());
            self.time = Instant::now();
        }
        Ok(())
    }
}

struct DrawGui;
impl<'a> System<'a> for DrawGui {
    type InitResource = ();
    type Resource = &'a mut ImGui;
    type Dependencies = ImGui;
    type Error = Infallible;

    fn update(&'a mut self, mut imgui: RefMut<'a, ImGui>) -> Result<(), Self::Error> {
        imgui.draw_frame(|ui| {
            let mut flag = false;
            ui.show_demo_window(&mut flag);
        });

        Ok(())
    }
}

struct RenderCrash;
impl<'a> System<'a> for RenderCrash {
    type InitResource = ();
    type Resource = &'a mut xecs::Errors;
    type Dependencies = End;
    type Error = Infallible;

    fn update(&'a mut self, mut errors: RefMut<'a, xecs::Errors>) -> Result<(), Self::Error> {
        for error in errors.fetch_all_errors() {
            panic!("Caught Error : {}",&error);
        }
        Ok(())
    }
}

fn main(){
    let mut game = Elikar::new().unwrap();

    let id = game
        .current_stage_ref()
        .system_data_mut::<window::Manager>()
        .create_window()
        .vulkan()
        .title("imgui render test")
        //.resizable()
        .build()
        .unwrap()
        .id();

    let vulkan = Vulkan::builder()
        .enable_debug()
        .present_mode(PresentMode::Immediate)
        .debug_error()
        .debug_warning()
        .app_name("imgui render")
        .build(
            game.current_stage_ref()
                .system_data_ref::<window::Manager>()
                .window_ref(id)
                .unwrap(),
        )
        .unwrap();

    let mut frame_begin = FrameBegin::new();
    frame_begin.set_background_color((1.0,1.0,1.0,1.0));

    game.current_stage_mut()
        .add_system(Quit)
        .add_system(vulkan)
        .add_system(frame_begin)
        .add_system(FrameEnd::new())
        .add_system(ShowFpsOnWindow::from_window_id(id))
        .add_system(ImGui::from_window_id(id))
        .add_system(ImGuiRenderer::<DrawGui>::new())
        .add_system(ImGuiEventSystem::new())
        .add_system(DrawGui)
        .add_system(RenderCrash);

    game.run()
}
