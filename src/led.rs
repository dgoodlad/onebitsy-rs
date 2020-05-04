use crate::hal::gpio::gpioa::PA8;
use crate::hal::gpio::{Output, PushPull};
use crate::hal::prelude::*;

pub type LED = PA8<Output<PushPull>>;

pub trait Led {
    fn off(self);
    fn on(self);
}

impl Led for LED {
    fn off(self) {
        self.set_high().unwrap();
    }

    fn on(self) {
        self.set_low().unwrap();
    }
}
