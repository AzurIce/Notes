//! Light up the LED on PA6 using PAC

#![no_main]
#![no_std]

use nb::block;
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = pac::Peripherals::take().unwrap();

    // Take ownership of the flash and rcc peripherals and convert them into HAL structs
    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOA peripheral and split it into individual GPIO pins
    let mut gpioa = p.GPIOA.split();

    // Define the pa6 as a push-pull output
    let mut led = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(1.Hz()).unwrap();

    loop {
        // busy wait until the timer wraps around
        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
    }
}
