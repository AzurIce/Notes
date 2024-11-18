//! Implement a delay of 1 second using systick and while loop

#![no_main]
#![no_std]

use cortex_m::{peripheral::syst::SystClkSource, Peripherals};
use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    let peripherals = Peripherals::take().unwrap();
    let mut syst = peripherals.SYST;
    syst.set_clock_source(SystClkSource::Core);
    // 8 MHz according to 2.3.7 Clocks and startup of the datasheet
    syst.set_reload(8_000_000);
    syst.clear_current();
    syst.enable_counter();
    while !syst.has_wrapped() {}

    hprintln!("Hello, world! after 1 second").unwrap();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    // debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
