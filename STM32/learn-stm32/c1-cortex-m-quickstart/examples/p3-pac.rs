//! Light up the LED on PA6 using PAC

#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use stm32f1::stm32f103;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = stm32f103::Peripherals::take().unwrap();

    let mut syst = cp.SYST;
    // configure the system timer to wrap around every second
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // 1s
    syst.enable_counter();

    // enable the GPIOA clocks
    p.RCC
        .apb2enr
        .modify(|_, w| w.iopaen().set_bit());

    // configure PA6 to output mode using push-pull
    p.GPIOA
        .crl
        .modify(|_, w| 
            // set PA6 to output mode
            w.mode6().output()
            // set PA6 to push-pull output
            .cnf6().push_pull()
        );

    loop {
        // busy wait until the timer wraps around
        while !syst.has_wrapped() {}
        syst.clear_current();
        // set PA6 to high
        p.GPIOA.bsrr.write(|w| w.bs6().set_bit());

        while !syst.has_wrapped() {}
        syst.clear_current();
        // set PA6 to low
        p.GPIOA.bsrr.write(|w| w.br6().set_bit());
    }
}
