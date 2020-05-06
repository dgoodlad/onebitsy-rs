use crate::hal::gpio::gpioa::PA8;
use crate::hal::gpio::{Output, PushPull};
use crate::hal::prelude::*;

pub type LED = PA8<Output<PushPull>>;

pub trait Led {
    fn off(&mut self);
    fn on(&mut self);
}

impl Led for LED {
    fn off(&mut self) {
        self.set_high().unwrap();
    }

    fn on(&mut self) {
        self.set_low().unwrap();
    }
}
