//! Prints "Hello, world!" on the host console using semihosting
//! 
//! Note that this example needs to disable stm32f1 and stm32f1xx-hal dependencies in Cargo.toml

#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!");

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    // debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
