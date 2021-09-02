extern crate sdl2_sys;

use sdl2_sys::*;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use crate::common::{ SdlError,Result };

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
    id : usize,
    text : String,
    function : Box<dyn Fn()>
}

pub struct MsgboxBuilder{
    box_type : Type,
    title : String,
    message : String,
    next_button_id : usize,
    buttons : Vec<ButtonInfo>,
}

impl MsgboxBuilder {
    pub fn error() -> MsgboxBuilder {
        MsgboxBuilder {
            box_type : Type::Error,
            title : String::new(),
            message : String::new(),
            next_button_id : 0,
            buttons : Vec::new()
        }
    }

    pub fn warning() -> MsgboxBuilder {
        MsgboxBuilder {
            box_type : Type::Warning,
            title : String::new(),
            message : String::new(),
            next_button_id : 0,
            buttons : Vec::new()
        }
    }

    pub fn information() -> MsgboxBuilder {
        MsgboxBuilder {
            box_type : Type::Information,
            title : String::new(),
            message : String::new(),
            next_button_id : 0,
            buttons : Vec::new()
        }
    }

    pub fn title(self,t : &str) -> Self{
        Self{
            title : t.to_owned(),
            ..self
        }
    }

    pub fn message(self, t : &str) -> Self {
        Self{
            message : t.to_owned(),
            ..self
        }
    }

    pub fn add_button(self, default_key : ButtonDefaultKey, text : &str,func : impl 'static + Fn()) -> Self{
        let mut t = self;
        t.buttons.push(ButtonInfo{
            default_key,
            id : t.next_button_id,
            text : text.to_owned(),
            function : Box::new(func)
        });
        t.next_button_id += 1;
        t
    }

    pub fn build(self) -> Result<()> {
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
            let tmp = CString::new(btn.text.clone()).unwrap();
            button_text_cstr.push(tmp);
            sdl_buttons.push(SDL_MessageBoxButtonData {
                flags,
                buttonid : btn.id as i32,
                text : button_text_cstr.last().unwrap().as_ptr() as *const c_char
            })
        }
        let title_cstr = CString::new(self.title.clone()).unwrap();
        let msg_cstr = CString::new(self.message.clone()).unwrap();
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
            Err(SdlError::get())
        } else {
            for btn in self.buttons{
                if btn.id == button_id as usize {
                    (*btn.function)();
                    break;
                }
            }
            Ok(())
        }
    }

}

pub fn alert(title : &str,message : &str){
    MsgboxBuilder::warning()
        .title(title)
        .message(message)
        .add_button(ButtonDefaultKey::Return,"Ok",||{})
        .build()
        .unwrap();
}

pub trait UnwrapErrorMsgbox {
    type Item;
    fn unwrap_error_msgbox(self) -> Self::Item;
}

impl<T> UnwrapErrorMsgbox for Option<T> {
    type Item = T;

    fn unwrap_error_msgbox(self) -> Self::Item {
        match self {
            Option::None => {
                MsgboxBuilder::error()
                    .title("Error")
                    .message("Unwrap")
                    .add_button(ButtonDefaultKey::Return,"Ok",||{})
                    .build().unwrap();
                panic!("unwrap");
            }
            Option::Some(t) => { t }
        }
    }
}

