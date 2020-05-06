use crate::hal;

use hal::prelude::*;
use hal::time::MegaHertz;

pub trait OneBitsyCrystal {
    const CRYSTAL_FREQ: MegaHertz = MegaHertz(25);

    fn use_one_bitsy_crystal(self) -> Self;
}

impl OneBitsyCrystal for hal::rcc::CFGR {
    fn use_one_bitsy_crystal(self) -> Self {
        self.use_hse(Self::CRYSTAL_FREQ)
    }
}

/// Configures the 25 MHz crystal and a 168 MHz system clock
///
/// The 1bitsy has a 25 MHz crystal wired to the MCU's high-speed external oscillator
/// pins. We enable that, and use it to drive the full 168 MHz system clock.
///
/// Usage:
///
/// ```
/// let p = stm32::Peripherals::take().unwrap();
/// let rcc = p.RCC.constrain();
/// let clocks = configure(rcc);
/// ```
pub fn configure(rcc: hal::rcc::Rcc) -> hal::rcc::Clocks {
    rcc.cfgr.use_one_bitsy_crystal().sysclk(168.mhz()).freeze()
}
