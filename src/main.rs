#![no_std]
#![no_main]
#![feature(asm)]

// pick a panicking behavior
extern crate panic_halt;

//#[cfg(debug_assertions)]
//extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln};

use hal::target_device;
use hal::gpio::*;

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::ToggleableOutputPin;

//use hal::serial::Serial0;
//use embedded_hal::serial::Write;

mod system;
mod util;
mod debug;

#[entry]
fn main() -> ! {
	let mut peripherals = target_device::Peripherals::take().unwrap();

	system::system_clock_init(&mut peripherals);
	debug!("System clock initialized!");

	//enable PIOC in PMC
	let mut pmc = peripherals.PMC;
	pmc.pmc_pcer0.write( |w| {
		w.pid12().set_bit()
	});

	let wdt = &peripherals.WDT;
	wdt.wdt_mr.write( |w| w.wddis().set_bit() );

	let mut pioc = peripherals.PIOC.split(&mut pmc);
	let mut pin0 = pioc.p19.into_open_drain_output();
	let mut pin1 = pioc.p10.into_open_drain_output();

	//blink
	loop {
		pin0.toggle();
		pin1.toggle();
		util::delayms(500);
	}
}
