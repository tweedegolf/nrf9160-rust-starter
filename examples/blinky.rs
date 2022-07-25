#![no_main]
#![no_std]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use nrf9160_hal::gpio::Level;
use nrf9160_hal::{gpio, pac::Peripherals, prelude::*};
use nrf9160_rust_starter as _; // global logger + panicking-behavior + memory layout

#[entry]
fn main() -> ! {
    defmt::println!("Entry into main");

    let p = Peripherals::take().unwrap();
    let port0 = gpio::p0::Parts::new(p.P0_NS);

    // Blue LED for the Conexio Stratus board
    let mut blue_led = port0.p0_03.into_push_pull_output(Level::Low);

    // Uncomment the line below if you are using the Icarus board. (po_10 = Red, p0_11 = Green, p0_12 = Blue)
    // let mut blue_led = port0.p0_12.into_push_pull_output(Level::Low);

    // loops forever toggling the blue_led
    loop {
        match blue_led.is_set_high().unwrap() {
            true => {
                blue_led.set_low().unwrap();
            }

            false => {
                blue_led.set_high().unwrap();
            }
        }

        delay(50_000_000);
    }
}
