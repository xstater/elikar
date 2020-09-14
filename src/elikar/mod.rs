extern crate  sdl2;

use self::sdl2::keyboard::KeyboardUtil;
use self::sdl2::mouse::MouseUtil;

#[derive(Debug)]
pub struct Elikar{
    app_name : String,
    app_version : (u32,u32),
    sdl_context : sdl2::Sdl,
    keyboard : KeyboardUtil,
    mouse : MouseUtil,
    audio_system : Option<sdl2::AudioSubsystem>,
    event_system : Option<sdl2::EventSubsystem>,
    joystick_system : Option<sdl2::JoystickSubsystem>,
    haptic_system : Option<sdl2::HapticSubsystem>,
    game_controller_system : Option<sdl2::GameControllerSubsystem>,
    timer_system : Option<sdl2::TimerSubsystem>,
    video_system : Option<sdl2::VideoSubsystem>,
    event_pump : sdl2::EventPump
}

pub enum SdlError{
    InitSdlError(String),
    InitAudioError(String),
    InitEventError(String),
    InitJoystickError(String),
    InitHapticError(String),
    InitGameControllerError(String),
    InitTimerError(String),
    InitVideoError(String),
    InitEventPumpError(String)
}

impl Elikar{
    pub fn new(app_name : &str, app_version : (u32, u32)) -> Result<Elikar, SdlError> {
       unimplemented!()
    }
}


