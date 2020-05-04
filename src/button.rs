use crate::hal::gpio::gpiob::PB1;
use crate::hal::gpio::{Floating, Input};
use crate::hal::prelude::*;

pub type BUTTON = PB1<Input<Floating>>;

pub trait Button {
    fn is_pressed(self) -> bool;
}

impl Button for BUTTON {
    fn is_pressed(self) -> bool {
        self.is_low().unwrap()
    }
}
