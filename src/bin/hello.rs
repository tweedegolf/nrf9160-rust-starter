#![no_main]
#![no_std]

use nrf9160_rust_starter as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    loop {
        cortex_m::asm::wfi();
    }
}
