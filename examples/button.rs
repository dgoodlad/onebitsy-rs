#![no_std]
#![no_main]

extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use hal::prelude::*;
use onebitsy::{self, hal, stm32, Button, Led};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(_cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let _clocks = onebitsy::configure_clocks(dp.RCC.constrain());

        let pins = onebitsy::Pins::new(
            dp.GPIOA.split(),
            dp.GPIOB.split(),
            dp.GPIOC.split(),
            dp.GPIOD.split(),
        );
        let mut led = onebitsy::led(pins.pa8);
        let button = onebitsy::button(pins.pc1);

        loop {
            if button.is_pressed() {
                led.on();
            } else {
                led.off();
            }
        }
    }

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
