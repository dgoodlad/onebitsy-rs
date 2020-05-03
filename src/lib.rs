//! Board support crate for the 1Bitsy
//!
//! # Usage
//!

#![deny(warnings)]
#![no_std]

pub use stm32f4xx_hal as hal;
pub use stm32f4xx_hal::stm32;

use embedded_hal::digital::v2::{InputPin, OutputPin};
use hal::gpio::gpiob::{self, PB8};
use hal::gpio::gpioc::{self, PC1};
use hal::gpio::{Floating, Input, Output, PushPull};
use hal::prelude::*;

/// The 1Bitsy's single on-board LED
///
/// # Usage
///
/// ```
/// let dp = stm32::Peripherals::take();
/// let gpiob = dp.GPIOB.split();
/// let mut led = Led::new(gpiob);
///
/// led.on();
/// led.off();
/// ```
pub struct Led {
    pin: PB8<Output<PushPull>>,
}

impl Led {
    pub fn new(pb8: PB8<Output<PushPull>>) -> Self {
        Led { pin: pb8 }
    }

    pub fn off(&mut self) {
        self.pin.set_high().unwrap();
    }

    pub fn on(&mut self) {
        self.pin.set_low().unwrap();
    }
}

/// The 1Bitsy's single on-board button
///
/// # Usage
///
/// ```
/// let dp = stm32::Peripherals::take();
/// let gpiob = dp.GPIOC.split();
/// let mut button = Button::new(gpioc);
///
/// if button.is_pressed() {
///   // ...
/// }
/// ```
pub struct Button {
    pin: PC1<Input<Floating>>,
}

impl Button {
    pub fn new(pin: PC1<Input<Floating>>) -> Self {
        Button { pin: pin }
    }

    pub fn is_pressed(&mut self) -> bool {
        self.pin.is_low().unwrap()
    }
}

pub struct Pins {
    pub pc10: hal::gpio::gpioc::PC10<Input<Floating>>,
    pub pc11: hal::gpio::gpioc::PC11<Input<Floating>>,
    pub pc12: hal::gpio::gpioc::PC12<Input<Floating>>,
}

pub struct OneBitsy {
    pub pins: Pins,
    pub led: Led,
    pub button: Button,
}

impl OneBitsy {
    pub fn new(dp: stm32::Peripherals) -> Self {
        //let gpioa: gpioa::Parts = dp.GPIOA.split();
        let gpiob: gpiob::Parts = dp.GPIOB.split();
        let gpioc: gpioc::Parts = dp.GPIOC.split();
        Self {
            pins: Pins {
                pc10: gpioc.pc10,
                pc11: gpioc.pc11,
                pc12: gpioc.pc12,
            },
            led: Led::new(gpiob.pb8.into_push_pull_output()),
            button: Button::new(gpioc.pc1),
        }
    }
}
