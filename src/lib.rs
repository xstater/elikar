pub mod clipboard;
pub mod drop_file;
pub mod events;
pub mod ime;
pub mod keyboard;
pub mod mouse;
pub mod msgbox;
pub mod power;
pub mod sysinfo;
pub mod window;
pub mod time;
mod quit;
mod elikar;
mod common;
pub use elikar::{
    Elikar,
    ElikarWorld,
    SdlInitError
};
pub use common::{
    Handle,
    SdlError,
    Spawner,
    Result
};

pub fn init() -> std::result::Result<Elikar,SdlInitError> {
    Elikar::new()
}
