#![no_main]
#![no_std]

use nrf9160_rust_starter as _; // global logger + panicking-behavior + memory layout

use nrf9160_hal as hal;

use hal::gpio::{Level, Output, PushPull};
use hal::uarte;

use core::fmt::Write;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();
    let pins0 = hal::gpio::p0::Parts::new(p.P0_NS);
    let uart_pins = hal::uarte::Pins {
        rxd: pins0.p0_05.into_floating_input().degrade(),
        txd: pins0.p0_06.into_push_pull_output(Level::High).degrade(),
        cts: None,
        rts: None,
    };

    let mut uarte = hal::Uarte::new(
        p.UARTE0_NS,
        uart_pins,
        uarte::Parity::EXCLUDED,
        uarte::Baudrate::BAUD115200,
    );

    write!(uarte, "Hello, World!\r\n").unwrap();

    nrf9160_rust_starter::exit()
}
