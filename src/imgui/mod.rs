pub mod systems;

use crate::imgui::systems::ImGuiEventSystem;
use crate::mouse::Mouse;
use crate::window::WindowId;
use crate::{window, ElikarStates};
use imgui::{Context, DrawData, Io, Ui};
use std::cell::{Ref, RefMut};
use std::convert::Infallible;
use xecs::System;

pub struct ImGui {
    window_id: WindowId,
    context: imgui::Context,
    draw_data: Option<*const DrawData>,
}

impl ImGui {
    pub fn from_window_id(window_id: WindowId) -> ImGui {
        ImGui {
            window_id,
            context: Context::create(),
            draw_data: Option::None,
        }
    }

    pub fn window_id(&self) -> WindowId {
        self.window_id
    }

    pub(in crate::imgui) fn io_mut(&mut self) -> &mut Io {
        self.context.io_mut()
    }

    pub(in crate::imgui) fn draw_data(&self) -> &DrawData {
        let ptr = self.draw_data.unwrap();
        unsafe { &*ptr }
    }

    pub fn draw_frame<F>(&mut self, f: F)
    where
        F: Fn(&Ui<'_>),
    {
        let ui = self.context.frame();
        f(&ui);
        let draw_data = ui.render();
        self.draw_data.replace(draw_data);
    }
}

impl<'a> System<'a> for ImGui {
    type InitResource = ();
    type Resource = (
        &'a window::Manager,
        &'a ElikarStates,
        &'a Mouse,
        &'a mut ImGuiEventSystem,
    );
    type Dependencies = ImGuiEventSystem;
    type Error = Infallible;

    fn init(&'a mut self, _: ()) -> Result<(), Self::Error> {
        let io = self.io_mut();

        io.backend_flags |= imgui::BackendFlags::HAS_SET_MOUSE_POS;
        io.backend_flags |= imgui::BackendFlags::HAS_MOUSE_CURSORS;

        Ok(())
    }

    fn update(
        &'a mut self,
        (window_manager, states, mouse, mut event): (
            Ref<'a, window::Manager>,
            Ref<'a, ElikarStates>,
            Ref<'a, Mouse>,
            RefMut<'a, ImGuiEventSystem>,
        ),
    ) -> Result<(), Self::Error> {
        let window_id = self.window_id;
        let io = self.io_mut();
        let window = window_manager.window_ref(window_id).unwrap();

        let (w, h) = window.size();
        let (draw_w, draw_h) = window.vk_drawable_size();

        io.display_size = [w as _, h as _];
        io.display_framebuffer_scale = [draw_w as f32 / w as f32, draw_h as f32 / h as f32];

        // io.delta_time = states.now_frame_time().as_secs_f32();
        io.delta_time = states.last_frame_time().as_secs_f32();

        let (x, y) = mouse.position();
        io.mouse_pos = [x as _, y as _];

        let button = mouse.button();
        io.mouse_down[0] = event.mouse_pressed[0] || button.left();
        io.mouse_down[1] = event.mouse_pressed[1] || button.right();
        io.mouse_down[2] = event.mouse_pressed[2] || button.middle();
        event.mouse_pressed = [false; 5];

        Ok(())
    }
}
