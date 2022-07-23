#![no_main]
#![no_std]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use nrf9160_hal::{
    gpio,
    pac::Peripherals,
    prelude::*,
    saadc::{Saadc, SaadcConfig},
};
use nrf9160_rust_starter as _; // global logger + panicking-behavior + memory layout

#[entry]
fn main() -> ! {
    defmt::println!("Entry into main");

    let p = Peripherals::take().unwrap();
    let port0 = gpio::p0::Parts::new(p.P0_NS);

    // Configuration for reading analog voltage signal with ADC
    let adc_config = SaadcConfig::default();
    let mut adc = Saadc::new(p.SAADC_NS, adc_config);

    // Icarus uses the P0.13 pin internally for battery voltage measurement
    // P0.14 works on both Icarus and Stratus boards, or replace with a usable analog pin on your board
    let mut adc_pin = port0.p0_14.into_floating_input();

    // Blocking read
    defmt::println!("{}", adc.read(&mut adc_pin).unwrap());

    nrf9160_rust_starter::exit()
}
