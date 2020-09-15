extern crate sdl2;

use sdl2::keyboard::KeyboardUtil;
use sdl2::mouse::MouseUtil;

#[allow(dead_code)]
pub struct Elikar{
    app_name : String,
    app_version : (u32,u32,u32),
    keyboard : KeyboardUtil,
    mouse : MouseUtil,
    // audio_system : sdl2::AudioSubsystem,
    event_system : sdl2::EventSubsystem,
    // joystick_system : sdl2::JoystickSubsystem,
    // haptic_system : sdl2::HapticSubsystem,
    // game_controller_system : sdl2::GameControllerSubsystem,
    timer_system : sdl2::TimerSubsystem,
    video_system : sdl2::VideoSubsystem,
    event_pump : sdl2::EventPump,
    sdl_context : sdl2::Sdl
}

#[derive(Debug)]
pub enum SdlInitError{
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
    pub fn new(app_name : &str,app_version : (u32,u32,u32)) -> Result<Elikar,SdlInitError>{
        let sdl_context = sdl2::init()
            .map_err(|err| SdlInitError::InitSdlError(err)) ?;
        let event_system = sdl_context.event()
            .map_err(|err| SdlInitError::InitEventError(err)) ?;
        let timer_system = sdl_context.timer()
            .map_err(|err| SdlInitError::InitTimerError(err)) ?;
        let video_system = sdl_context.video()
            .map_err(|err| SdlInitError::InitVideoError(err)) ?;
        let event_pump = sdl_context.event_pump()
            .map_err(|err| SdlInitError::InitEventPumpError(err)) ?;

        Ok(Elikar{
            app_name : app_name.to_string(),
            app_version,
            keyboard : sdl_context.keyboard(),
            mouse : sdl_context.mouse(),
            event_system,
            timer_system,
            video_system,
            event_pump,
            sdl_context
        })
    }

    pub fn name(&self) -> &str{
        self.app_name.as_str()
    }

    pub fn version(&self) -> (u32,u32,u32) {
        self.app_version
    }

}
