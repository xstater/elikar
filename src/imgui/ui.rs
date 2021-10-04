use std::ffi::CString;
use imgui_sys::ImVec2;
use crate::imgui::ImGui;

pub struct Ui<'a> {
    pub (in crate::imgui) imgui : &'a mut ImGui,
    pub (in crate::imgui) texts : Vec<CString>
}

impl<'a> Ui<'a> {
    pub fn imgui(&'a self) -> &'a ImGui {
        self.imgui
    }

    pub fn imgui_mut(&'a mut self) -> &'a mut ImGui {
        self.imgui
    }

    pub fn text<T : AsRef<str>>(&mut self,text : T) {
        let s = text.as_ref();
        unsafe {
            let start = s.as_ptr();
            let end = start.add(s.len());
            imgui_sys::igTextUnformatted(start as _,end as _);
        }
    }

    pub fn button_with_size<T : AsRef<str>>(&mut self,text : T,width : f32,height : f32) -> bool {
        let text = CString::new(text.as_ref()).unwrap();
        self.texts.push(text);
        unsafe {
            imgui_sys::igButton(
                self.texts.last().unwrap().as_c_str().as_ptr(),
                ImVec2::new(width,height))
        }
    }

    pub fn button<T : AsRef<str>>(&mut self,text : T) -> bool {
        self.button_with_size(text,0.0,0.0)
    }

    pub fn separator(&mut self){
        unsafe { imgui_sys::igSeparator() }
    }
}