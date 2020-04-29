#![no_main]
#![no_std]

extern crate panic_halt;

use core::fmt::Write;

use cortex_m_rt::entry;
use stm32l0xx_hal::prelude::*;
use stm32l0xx_hal::{self as hal, serial, time};

#[entry]
#[allow(clippy::missing_safety_doc)]
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();
    let dp = hal::pac::Peripherals::take().unwrap();

    let syst = p.SYST;
    let mut rcc = dp.RCC.freeze(hal::rcc::Config::hsi16());
    let mut delay = hal::delay::Delay::new(syst, rcc.clocks);

    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    // Initialize serial port(s)
    let mut debug = serial::Serial::usart1(
        dp.USART1,
        gpiob.pb6.into_floating_input(),
        gpiob.pb7.into_floating_input(),
        serial::Config {
            baudrate: time::Bps(57_600),
            wordlength: serial::WordLength::DataBits8,
            parity: serial::Parity::ParityNone,
            stopbits: serial::StopBits::STOP1,
        },
        &mut rcc,
    )
    .unwrap();

    writeln!(debug, "Greetings, Rusty world, from Gfrörli v2!").unwrap();
    writeln!(debug, "Debug output initialized on USART1!").unwrap();

    // Initialize LEDs
    let mut led_r = gpiob.pb1.into_push_pull_output();
    let mut led_y = gpiob.pb0.into_push_pull_output();
    let mut led_g = gpioa.pa7.into_push_pull_output();

    writeln!(debug, "Starting loop").unwrap();
    loop {
        write!(debug, "a").unwrap();

        led_r.set_high().expect("Could not turn on LED");
        delay.delay(time::MicroSeconds(100_000));
        led_y.set_high().expect("Could not turn on LED");
        delay.delay(time::MicroSeconds(100_000));
        led_g.set_high().expect("Could not turn on LED");

        delay.delay(time::MicroSeconds(200_000));

        write!(debug, "b").unwrap();

        led_r.set_low().expect("Could not turn off LED");
        delay.delay(time::MicroSeconds(100_000));
        led_y.set_low().expect("Could not turn off LED");
        delay.delay(time::MicroSeconds(100_000));
        led_g.set_low().expect("Could not turn off LED");

        delay.delay(time::MicroSeconds(200_000));
    }
}
