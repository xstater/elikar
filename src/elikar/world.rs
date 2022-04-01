use parking_lot::RwLockReadGuard;
use xecs::{resource::{ResourceRead, ResourceWrite}, world::World};
use crate::{clipboard::Clipboard, ime::IME, keyboard::Keyboard, mouse::Mouse, quit::Quit, sysinfo::SystemInfo, time::Time};

/// fastly get resources of elikar
pub struct ElikarWorld<'a>{
    world : RwLockReadGuard<'a,World>
}

impl<'a> ElikarWorld<'a> {
    pub(in crate) fn new(world : RwLockReadGuard<'a,World>) -> Self {
        ElikarWorld {
            world
        }
    }

    pub fn time(&'a self) -> ResourceRead<'a,Time> {
        self.world.resource_read::<Time>()
            .expect("Time resource is removed unexpectly")
    }

    pub fn time_mut(&'a self) -> ResourceWrite<'a,Time> {
        self.world.resource_write::<Time>()
            .expect("Time resource is removed unexpectly")
    }

    pub fn need_quit(&'a self) -> bool {
        self.world.resource_read::<Quit>()
            .expect("Quit resource is removed unexpectly")
            .need_quit()
    }

    pub fn quit(&'a self) {
        self.world.resource_write::<Quit>()
            .expect("Quit resource is removed unexpectly")
            .quit()
    }

    pub fn mouse(&'a self) -> ResourceRead<'a,Mouse> {
        self.world.resource_read::<Mouse>()
            .expect("Mouse resource is removed unexpectly")
    }

    pub fn mouse_mut(&'a self) -> ResourceWrite<'a,Mouse> {
        self.world.resource_write::<Mouse>()
            .expect("Mouse resource is removed unexpectly")
    }

    pub fn keyboard(&'a self) -> ResourceRead<'a,Keyboard> {
        self.world.resource_read::<Keyboard>()
            .expect("Keyboard resource is removed unexpectly")
    }

    pub fn keyboard_mut(&'a self) -> ResourceWrite<'a,Keyboard> {
        self.world.resource_write::<Keyboard>()
            .expect("Keyboard resource is removed unexpectly")
    }

    pub fn clipboard(&'a self) -> ResourceRead<'a,Clipboard> {
        self.world.resource_read::<Clipboard>()
            .expect("Clipboard resource is removed unexpectly")
    }

    pub fn clipboard_mut(&'a self) -> ResourceWrite<'a,Clipboard> {
        self.world.resource_write::<Clipboard>()
            .expect("Clipboard resource is removed unexpectly")
    }

    pub fn ime(&'a self) -> ResourceRead<'a,IME> {
        self.world.resource_read::<IME>()
            .expect("IME resource is removed unexpectly")
    }

    pub fn ime_mut(&'a self) -> ResourceWrite<'a,IME> {
        self.world.resource_write::<IME>()
            .expect("IME resource is removed unexpectly")
    }

    pub fn system_info(&'a self) -> ResourceRead<'a,SystemInfo> {
        self.world.resource_read::<SystemInfo>()
            .expect("SystemInfo resource is removed unexpectly")
    }
}

