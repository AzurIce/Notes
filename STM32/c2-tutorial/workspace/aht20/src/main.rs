#![no_main]
#![no_std]

use core::{fmt::Write, time::Duration};

use cortex_m_semihosting::{hprint, hprintln};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::Point,
    text::{Baseline, Text},
    Drawable,
};
use panic_halt as _;

use cortex_m_rt::entry;
use nb::block;
use sh1106::{mode::GraphicsMode, Builder};
use shared_bus::BusManagerSimple;
use stm32f1xx_hal::{
    i2c, pac,
    prelude::*,
    time::MicroSeconds,
    timer::{Delay, Timer},
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = pac::Peripherals::take().unwrap();

    // Take ownership of the flash and rcc peripherals and convert them into HAL structs
    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .hclk(72.MHz())
        .freeze(&mut flash.acr);

    // Acquire the GPIOA peripheral and split it into individual GPIO pins
    let mut gpioa = p.GPIOA.split();
    let mut gpiob = p.GPIOB.split();

    let mut afio = p.AFIO.constrain();
    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let mut i2c = i2c::BlockingI2c::i2c1(
        p.I2C1,
        (scl, sda),
        &mut afio.mapr,
        i2c::Mode::Fast {
            frequency: 400.kHz(),
            duty_cycle: i2c::DutyCycle::Ratio2to1, // TODO: what is this?
        },
        clocks,
        50000,
        3,
        10000,
        50000,
    );
    let shared_i2c = BusManagerSimple::new(i2c);

    let mut disp: GraphicsMode<_> = Builder::new()
        .with_i2c_addr(0x3d)
        .connect_i2c(shared_i2c.acquire_i2c())
        .into();
    // hprint!("initializing display...");
    if let Err(err) = disp.init() {
        // hprintln!("error initializing display: {:?}", err);
    }
    // hprintln!("done!");
    disp.flush().unwrap();

    let mut delay = Timer::syst(cp.SYST, &clocks).delay();
    hprint!("initializing aht20...");
    let mut aht20_uninit =
        aht20_driver::AHT20::new(shared_i2c.acquire_i2c(), aht20_driver::SENSOR_ADDRESS);
    let mut aht20 = aht20_uninit.init(&mut delay).unwrap();
    // hprintln!("done!");

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    let mut str = heapless::String::<100>::new();

    loop {
        // busy wait until the timer wraps around
        delay.delay(MicroSeconds::millis(20));
        let aht20_measurement = aht20.measure(&mut delay).unwrap();

        disp.clear();
        str.clear();
        write!(
            &mut str,
            "temperature: {:.2}C",
            aht20_measurement.temperature
        )
        .unwrap();
        Text::new(str.as_str(), Point::new(0, 10), text_style)
            .draw(&mut disp)
            .unwrap();
        // hprintln!("temperature: {:.2}C", aht20_measurement.temperature);

        str.clear();
        write!(
            &mut str,
            "humidity: {:.2}%",
            aht20_measurement.humidity
        )
        .unwrap();
        Text::new(str.as_str(), Point::new(0, 24), text_style)
            .draw(&mut disp)
            .unwrap();
        disp.flush().unwrap();
        // hprintln!("humidity: {:.2}%", aht20_measurement.humidity);
    }
}
