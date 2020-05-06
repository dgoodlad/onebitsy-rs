use crate::hal::gpio::gpioc::PC1;
use crate::hal::gpio::{Floating, Input};
use crate::hal::prelude::*;

pub type BUTTON = PC1<Input<Floating>>;

pub trait Button {
    fn is_pressed(&self) -> bool;
}

impl Button for BUTTON {
    fn is_pressed(&self) -> bool {
        self.is_low().unwrap()
    }
}
