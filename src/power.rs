extern crate sdl2_sys;

use std::os::raw::c_int;
use sdl2_sys::*;
use crate::common::unit::{Time, Second};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum PowerState {
    OnBattery,
    NoBattery,
    Charging,
    Charged
}

pub fn power_state() -> Option<PowerState>{
    let state = unsafe { sdl2_sys::SDL_GetPowerInfo(0 as *mut c_int,0 as *mut c_int) };
    match state {
        SDL_PowerState::SDL_POWERSTATE_UNKNOWN => Option::None,
        SDL_PowerState::SDL_POWERSTATE_ON_BATTERY => Some(PowerState::OnBattery),
        SDL_PowerState::SDL_POWERSTATE_NO_BATTERY => Some(PowerState::NoBattery),
        SDL_PowerState::SDL_POWERSTATE_CHARGING => Some(PowerState::Charging),
        SDL_PowerState::SDL_POWERSTATE_CHARGED => Some(PowerState::Charged),
    }
}

pub fn battery_time() -> Option<Time> {
    let mut sec : i32 = -1;
    unsafe {
        sdl2_sys::SDL_GetPowerInfo(&mut sec as *mut c_int,0 as *mut c_int);
    }
    if sec < 0 {
        Option::None
    } else {
        Some(sec.s())
    }
}

pub fn battery_percentage() -> Option<u32> {
    let mut psc : i32 = -1;
    unsafe {
        sdl2_sys::SDL_GetPowerInfo(0 as *mut c_int,&mut psc as *mut c_int);
    }
    if 0 <= psc && psc <= 100 {
        Some(psc as u32)
    } else {
        Option::None
    }
}