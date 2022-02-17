#![no_main]
#![no_std]

use nrf9160_rust_starter as _; // global logger + panicking-behavior + memory layout

use nrf9160_hal as hal;

use hal::gpio::Level;
use hal::uarte;

use core::fmt::Write;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();
    let pins0 = hal::gpio::p0::Parts::new(p.P0_NS);
    let pins = hal::uarte::Pins {
        rxd: pins0.p0_05.into_floating_input().degrade(),
        txd: pins0.p0_06.into_push_pull_output(Level::High).degrade(),
        cts: None,
        rts: None,
    };

    let device = p.UARTE0_NS;

    // init 1

    let mut uart = hal::Uarte::new(
        device,
        pins,
        uarte::Parity::EXCLUDED,
        uarte::Baudrate::BAUD115200,
    );

    write!(uart, "write 1\r\n").unwrap();
    write!(uart, "write 2\r\n").unwrap();

    // deinit 1
    let (device, pins) = uart.free();
    device.events_txstopped.reset();
    device.tasks_stoptx.write(|w| w.tasks_stoptx().trigger());

    while device.events_txstopped.read().bits() == 0 {
        cortex_m::asm::nop();
    }

    device.enable.write(|w| w.enable().disabled());

    // init 2
    let mut uart = hal::Uarte::new(
        device,
        pins,
        uarte::Parity::EXCLUDED,
        uarte::Baudrate::BAUD115200,
    );

    write!(uart, "write 3\r\n").unwrap();

    nrf9160_rust_starter::exit()
}
