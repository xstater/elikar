mod sdl_error;
pub mod unit;
mod either;
mod signal;

pub use sdl_error::get_error;
pub use either::Either;
pub use signal::Signal;