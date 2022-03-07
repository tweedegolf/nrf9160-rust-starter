//! A simple program that connects to serial and relays the socket AT commands to it.
//! With this you can talk directly to the nrf modem from the PC

#![no_main]
#![no_std]

// links in a minimal version of libc
extern crate tinyrlibc;

use core::fmt::Write;

use defmt::unwrap;
use nrf9160_hal::{
    gpio,
    pac::{self, interrupt},
    uarte,
};
use nrf9160_rust_starter as _; // global logger + panicking-behavior + memory layout

const MILLISECOND_CYCLES: u32 = nrf9160_hal::Timer::<pac::TIMER0_NS>::TICKS_PER_SECOND / 1000;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut cp = unwrap!(cortex_m::Peripherals::take());
    let dp = unwrap!(nrf9160_hal::pac::Peripherals::take());

    // Enable the modem interrupts
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EGU1);
        pac::NVIC::unmask(pac::Interrupt::EGU2);
        pac::NVIC::unmask(pac::Interrupt::IPC);
        cp.NVIC.set_priority(pac::Interrupt::EGU1, 4 << 5);
        cp.NVIC.set_priority(pac::Interrupt::EGU2, 4 << 5);
        cp.NVIC.set_priority(pac::Interrupt::IPC, 0 << 5);
    }

    // Set up the serial
    let pins0 = gpio::p0::Parts::new(dp.P0_NS);
    let uart_pins = uarte::Pins {
        rxd: pins0.p0_05.into_floating_input().degrade(),
        txd: pins0
            .p0_06
            .into_push_pull_output(gpio::Level::High)
            .degrade(),
        cts: None,
        rts: None,
    };
    let mut serial = nrf9160_hal::Uarte::new(
        dp.UARTE0_NS,
        uart_pins,
        uarte::Parity::EXCLUDED,
        uarte::Baudrate::BAUD115200,
    );
    let mut serial_timer = nrf9160_hal::Timer::new(dp.TIMER0_NS);

    // Initialize the modem
    nrfxlib::init().unwrap();
    let at_socket = nrfxlib::at::AtSocket::new().unwrap();

    let mut buffer = [0; 1024];

    serial.write_str("Starting modem connection\r\n").unwrap();

    loop {
        if let Some(length) = at_socket.recv(&mut buffer).unwrap() {
            if length != 0 {
                defmt::info!("Sending to serial: {}", core::str::from_utf8(&buffer[..length]).unwrap());
                serial.write(&buffer[..length]).unwrap();
            }
        }

        if let Err(nrf9160_hal::uarte::Error::Timeout(length)) =
            serial.read_timeout(&mut buffer, &mut serial_timer, MILLISECOND_CYCLES * 100)
        {
            if length != 0 {
                defmt::info!("Sending to at: {}", core::str::from_utf8(&buffer[..length]).unwrap());
                at_socket.write(&buffer[..length]).unwrap();
            }
        }
    }
}

/// Interrupt Handler for LTE related hardware. Defer straight to the library.
#[interrupt]
fn EGU1() {
    nrfxlib::application_irq_handler();
    cortex_m::asm::sev();
}

/// Interrupt Handler for LTE related hardware. Defer straight to the library.
#[interrupt]
fn EGU2() {
    nrfxlib::trace_irq_handler();
    cortex_m::asm::sev();
}

/// Interrupt Handler for LTE related hardware. Defer straight to the library.
#[interrupt]
fn IPC() {
    nrfxlib::ipc_irq_handler();
    cortex_m::asm::sev();
}
