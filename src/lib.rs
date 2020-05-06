//! Board support crate for the 1Bitsy
//!
//! # Usage
//!

#![deny(warnings)]
#![no_std]

pub use stm32f4xx_hal as hal;
pub use stm32f4xx_hal::stm32;

use hal::gpio::{gpioa, gpiob, gpioc, gpiod};
use hal::gpio::{Floating, Input};

pub mod button;
pub mod clocks;
pub mod led;

pub use button::Button;
pub use clocks::configure as configure_clocks;
pub use led::Led;

pub struct Pins {
    pub pa0: gpioa::PA0<Input<Floating>>,
    pub pa1: gpioa::PA1<Input<Floating>>,
    pub pa2: gpioa::PA2<Input<Floating>>,
    pub pa3: gpioa::PA3<Input<Floating>>,
    pub pa4: gpioa::PA4<Input<Floating>>,
    pub pa5: gpioa::PA5<Input<Floating>>,
    pub pa6: gpioa::PA6<Input<Floating>>,
    pub pa7: gpioa::PA7<Input<Floating>>,
    pub pa8: gpioa::PA8<Input<Floating>>,
    pub pa9: gpioa::PA9<Input<Floating>>,
    pub pa10: gpioa::PA10<Input<Floating>>,
    pub pa11: gpioa::PA11<Input<Floating>>,
    pub pa12: gpioa::PA12<Input<Floating>>,
    pub pa13: gpioa::PA13<Input<Floating>>,
    pub pa14: gpioa::PA14<Input<Floating>>,
    pub pa15: gpioa::PA15<Input<Floating>>,

    pub pb0: gpiob::PB0<Input<Floating>>,
    pub pb1: gpiob::PB1<Input<Floating>>,
    pub pb2: gpiob::PB2<Input<Floating>>,
    pub pb3: gpiob::PB3<Input<Floating>>,
    pub pb4: gpiob::PB4<Input<Floating>>,
    pub pb5: gpiob::PB5<Input<Floating>>,
    pub pb6: gpiob::PB6<Input<Floating>>,
    pub pb7: gpiob::PB7<Input<Floating>>,
    pub pb8: gpiob::PB8<Input<Floating>>,
    pub pb9: gpiob::PB9<Input<Floating>>,
    pub pb10: gpiob::PB10<Input<Floating>>,
    pub pb11: gpiob::PB11<Input<Floating>>,
    pub pb12: gpiob::PB12<Input<Floating>>,
    pub pb13: gpiob::PB13<Input<Floating>>,
    pub pb14: gpiob::PB14<Input<Floating>>,
    pub pb15: gpiob::PB15<Input<Floating>>,

    pub pc1: gpioc::PC1<Input<Floating>>,
    pub pc2: gpioc::PC2<Input<Floating>>,
    pub pc3: gpioc::PC3<Input<Floating>>,
    pub pc4: gpioc::PC4<Input<Floating>>,
    pub pc5: gpioc::PC5<Input<Floating>>,
    pub pc6: gpioc::PC6<Input<Floating>>,
    pub pc7: gpioc::PC7<Input<Floating>>,
    pub pc8: gpioc::PC8<Input<Floating>>,
    pub pc9: gpioc::PC9<Input<Floating>>,
    pub pc10: gpioc::PC10<Input<Floating>>,
    pub pc11: gpioc::PC11<Input<Floating>>,
    pub pc12: gpioc::PC12<Input<Floating>>,
    pub pc13: gpioc::PC13<Input<Floating>>,
    pub pc14: gpioc::PC14<Input<Floating>>,
    pub pc15: gpioc::PC15<Input<Floating>>,

    pub pd2: gpiod::PD2<Input<Floating>>,
}

impl Pins {
    pub fn new(
        gpioa: gpioa::Parts,
        gpiob: gpiob::Parts,
        gpioc: gpioc::Parts,
        gpiod: gpiod::Parts,
    ) -> Self {
        Self {
            pa0: gpioa.pa0,
            pa1: gpioa.pa1,
            pa2: gpioa.pa2,
            pa3: gpioa.pa3,
            pa4: gpioa.pa4,
            pa5: gpioa.pa5,
            pa6: gpioa.pa6,
            pa7: gpioa.pa7,
            pa8: gpioa.pa8,
            pa9: gpioa.pa9,
            pa10: gpioa.pa10,
            pa11: gpioa.pa11,
            pa12: gpioa.pa12,
            pa13: gpioa.pa13,
            pa14: gpioa.pa14,
            pa15: gpioa.pa15,

            pb0: gpiob.pb0,
            pb1: gpiob.pb1,
            pb2: gpiob.pb2,
            pb3: gpiob.pb3,
            pb4: gpiob.pb4,
            pb5: gpiob.pb5,
            pb6: gpiob.pb6,
            pb7: gpiob.pb7,
            pb8: gpiob.pb8,
            pb9: gpiob.pb9,
            pb10: gpiob.pb10,
            pb11: gpiob.pb11,
            pb12: gpiob.pb12,
            pb13: gpiob.pb13,
            pb14: gpiob.pb14,
            pb15: gpiob.pb15,

            pc1: gpioc.pc1,
            pc2: gpioc.pc2,
            pc3: gpioc.pc3,
            pc4: gpioc.pc4,
            pc5: gpioc.pc5,
            pc6: gpioc.pc6,
            pc7: gpioc.pc7,
            pc8: gpioc.pc8,
            pc9: gpioc.pc9,
            pc10: gpioc.pc10,
            pc11: gpioc.pc11,
            pc12: gpioc.pc12,
            pc13: gpioc.pc13,
            pc14: gpioc.pc14,
            pc15: gpioc.pc15,

            pd2: gpiod.pd2,
        }
    }
}

pub fn led(pin: gpioa::PA8<Input<Floating>>) -> led::LED {
    pin.into_push_pull_output()
}

pub fn button(pin: gpioc::PC1<Input<Floating>>) -> button::BUTTON {
    pin
}
