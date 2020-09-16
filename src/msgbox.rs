extern crate sdl2_sys;

use sdl2_sys::*;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use crate::elikar::get_error;

#[derive(Debug)]
enum Type{
    Error,
    Warning,
    Information
}

#[derive(Debug)]
pub enum ButtonDefaultKey{
    Nope,
    Return,
    Escape
}

pub struct ButtonInfo{
    default_key : ButtonDefaultKey,
    id : i32,
    text : String
}

pub struct MsgboxBuilder{
    box_type : Type,
    title : String,
    message : String,
    buttons : Vec<ButtonInfo>,
}

impl MsgboxBuilder {
    pub fn new() -> MsgboxBuilder {
        MsgboxBuilder {
            box_type : Type::Information,
            title : String::new(),
            message : String::new(),
            buttons : Vec::new()
        }
    }

    pub fn error(&mut self) -> &mut Self {
        self.box_type = Type::Error;
        self
    }

    pub fn warning(&mut self) -> &mut Self {
        self.box_type = Type::Warning;
        self
    }

    pub fn information(&mut self) -> &mut Self {
        self.box_type = Type::Information;
        self
    }

    pub fn title(&mut self,t : &str) -> &mut Self{
        self.title = t.to_owned();
        self
    }

    pub fn message(&mut self, t : &str) -> &mut Self {
        self.message = t.to_owned();
        self
    }

    pub fn add_button(&mut self,default_key : ButtonDefaultKey,id : i32,text : &str) -> &mut Self{
        self.buttons.push(ButtonInfo{
            default_key,
            id,
            text : text.to_owned()
        });
        self
    }

    pub fn build(&self) -> Result<i32,String> {
        let mut sdl_buttons : Vec<SDL_MessageBoxButtonData> = Vec::new();
        let mut button_text_cstr : Vec<CString> = Vec::new();
        for btn in self.buttons.iter().rev() {
            let flags : u32 = match &btn.default_key {
                ButtonDefaultKey::Nope => { 0 },
                ButtonDefaultKey::Return => {
                    SDL_MessageBoxButtonFlags::SDL_MESSAGEBOX_BUTTON_RETURNKEY_DEFAULT as u32
                },
                ButtonDefaultKey::Escape => {
                    SDL_MessageBoxButtonFlags::SDL_MESSAGEBOX_BUTTON_ESCAPEKEY_DEFAULT as u32
                },
            };
            let tmp = CString::new(btn.text.clone())
                .map_err(|_| "Invalid button text") ?;
            button_text_cstr.push(tmp);
            sdl_buttons.push(SDL_MessageBoxButtonData {
                flags,
                buttonid : btn.id,
                text : button_text_cstr.last().unwrap().as_ptr() as *const c_char
            })
        }
        let title_cstr = CString::new(self.title.clone())
            .map_err(|_| "Invalid title") ?;
        let msg_cstr = CString::new(self.message.clone())
            .map_err(|_| "Invalid message") ?;
        let sdl_msgbox = SDL_MessageBoxData {
            flags : match &self.box_type {
                Type::Error => {
                    SDL_MessageBoxFlags::SDL_MESSAGEBOX_ERROR as u32
                },
                Type::Warning => {
                    SDL_MessageBoxFlags::SDL_MESSAGEBOX_WARNING as u32
                },
                Type::Information => {
                    SDL_MessageBoxFlags::SDL_MESSAGEBOX_INFORMATION as u32
                },
            },
            window : 0 as *mut _,
            title : title_cstr.as_ptr() as *const c_char,
            message : msg_cstr.as_ptr() as *const c_char,
            numbuttons : sdl_buttons.len() as c_int,
            buttons : sdl_buttons.as_ptr(),
            colorScheme : 0 as *const _
        };
        let mut button_id : i32 = -1;
        let errcode = unsafe {
            SDL_ShowMessageBox(&sdl_msgbox as *const _,&mut button_id as *mut _)
        };
        if errcode == -1 || button_id == -1{
            Err(get_error())
        } else {
            Ok(button_id)
        }
    }

}

pub fn alert(title : &str,message : &str){
    MsgboxBuilder::new()
        .warning()
        .title(title)
        .message(message)
        .add_button(ButtonDefaultKey::Return,0,"Ok")
        .build()
        .unwrap();
}