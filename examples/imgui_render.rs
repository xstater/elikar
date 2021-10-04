use elikar::{Elikar, ElikarStates, window};
use xecs::System;
use elikar::events::PollEvents;
use elikar::render::vulkan::{PresentMode, Vulkan};
use std::cell::{Ref, RefMut};
use std::convert::Infallible;
use elikar::imgui::ImGui;
use elikar::imgui::systems::{ImGuiEventSystem, ImGuiRenderer};
use elikar::render::vulkan::systems::{AcquireNextImage, ExecuteRenderCommands};

struct Quit;
impl<'a> System<'a> for Quit {
    type InitResource = ();
    type Resource = (&'a PollEvents,&'a mut ElikarStates);
    type Dependencies = PollEvents;
    type Error = Infallible;

    fn update(&'a mut self, (events,mut states) : (Ref<'a,PollEvents>,RefMut<'a,ElikarStates>)) -> Result<(),Self::Error>{
        if let Some(_) = events.quit {
            states.quit();
        }
        Ok(())
    }
}
struct PrintFps;
impl<'a> System<'a> for PrintFps {
    type InitResource = ();
    type Resource = (&'a PollEvents,&'a ElikarStates);
    type Dependencies = PollEvents;
    type Error = Infallible;

    fn update(&'a mut self, (events,states) : (Ref<'a,PollEvents>,Ref<'a,ElikarStates>)) -> Result<(),Self::Error> {
        for _ in &events.mouse_motion {
            println!("fps:{}",states.fps());
        }
        Ok(())
    }
}

struct DrawGui;
impl<'a> System<'a> for DrawGui{
    type InitResource = ();
    type Resource = &'a mut ImGui;
    type Dependencies = ImGuiEventSystem;
    type Error = Infallible;

    fn update(&'a mut self, mut imgui : RefMut<'a,ImGui>) -> Result<(), Self::Error> {
        let mut ui = imgui.ui();
        ui.text("asd");
        ui.separator();
        ui.button("aaa");
        Ok(())
    }
}

struct RenderCrash;
impl<'a> System<'a> for RenderCrash {
    type InitResource = ();
    type Resource = &'a mut xecs::Errors;
    type Dependencies = ImGuiRenderer<DrawGui>;
    type Error = Infallible;

    fn update(&'a mut self,mut errors : RefMut<'a,xecs::Errors>) -> Result<(), Self::Error> {
        if let Some(error) = errors.fetch_error::<ImGuiRenderer<DrawGui>>() {
            panic!("Caught error:{}",error)
        }

        Ok(())
    }
}

fn main() {
    let mut game = Elikar::new().unwrap();

    let id = game.current_stage_ref()
        .system_data_mut::<window::Manager>()
        .create_window()
        .vulkan()
        .title("imgui render test")
        .build()
        .unwrap()
        .id();

    let vulkan = Vulkan::builder()
        .enable_debug()
        .present_mode(PresentMode::Immediate)
        .debug_error()
        .debug_warning()
        .app_name("imgui render")
        .build(game.current_stage_ref()
            .system_data_ref::<window::Manager>()
            .window_ref(id)
            .unwrap())
        .unwrap();

    game.current_stage_mut()
        .add_system(PrintFps)
        .add_system(Quit)
        .add_system(vulkan)
        .add_system(AcquireNextImage::new())
        .add_system(ExecuteRenderCommands::new())
        .add_system(ImGui::from_window_id(id))
        .add_system(ImGuiRenderer::<DrawGui>::new())
        .add_system(ImGuiEventSystem::new())
        .add_system(DrawGui);

    game.run()
}