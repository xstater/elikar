mod builder;
mod window;
mod manager;

pub use window::Window;
pub use builder::Builder;
pub use manager::Manager;

pub fn builder() -> Builder {
    Builder::new()
}