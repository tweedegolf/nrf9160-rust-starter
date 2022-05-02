#![no_main]
#![no_std]

use hal::Twim;
use nrf9160_rust_starter as _; // global logger + panicking-behavior + memory layout
use pcf85063a::{self, DateTime};

use nrf9160_hal as hal;
use nrf9160_hal::pac;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    let ahb_frequency = 32_768u32;

    let p = cortex_m::peripheral::Peripherals::take().unwrap();

    let mut delay = cortex_m::delay::Delay::new(p.SYST, ahb_frequency);

    let p = pac::Peripherals::take().unwrap();
    let pins0 = hal::gpio::p0::Parts::new(p.P0_NS);

    let sda = pins0.p0_26.into_floating_input().degrade();
    let scl = pins0.p0_27.into_floating_input().degrade();

    let twim = Twim::new(
        p.TWIM2_NS,
        hal::twim::Pins { scl, sda },
        hal::twim::Frequency::K100,
    );

    // set up the PCF8563 device
    let mut rtc = pcf85063a::PCF85063::new(twim);

    // prepare date and time to be set
    let now = DateTime {
        year: 21,   // 2021
        month: 4,   // April
        weekday: 0, // Sunday
        day: 4,
        hours: 16,
        minutes: 52,
        seconds: 00,
    };

    // set date and time in one go
    rtc.set_datetime(&now).unwrap();

    loop {
        delay.delay_ms(500 as u32);

        //get date and time in one go
        let time = rtc.get_datetime().unwrap();

        defmt::println!(
            "{:02}/{:02}/{:02} {:02}:{:02}:{:02} day {}\r",
            time.year,
            time.month,
            time.day,
            time.hours,
            time.minutes,
            time.seconds,
            time.weekday
        );

        delay.delay_ms(500 as u32);
    }

    // nrf9160_rust_starter::exit()
}
