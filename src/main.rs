#![no_std]
#![no_main]
#![feature(asm)]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln};

use hal::target_device;
use hal::gpio::*;
use hal::serial::{config, Serial};
use hal::time::*;

use embedded_hal::digital::v2::ToggleableOutputPin;

use core::fmt::Write;

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

	let pioc = peripherals.PIOC.split(&mut pmc);
	let mut pin0 = pioc.p19.into_open_drain_output();
	let mut pin1 = pioc.p10.into_open_drain_output();

	let pioa = peripherals.PIOA.split(&mut pmc);
	let tx = pioa.p10.into_pmd0();
	let rx = pioa.p9.into_pmd0();

	let mut serial = Serial::uart0(
		peripherals.UART0,
		(tx, rx),
		config::UartConfig::default().baudrate(115_200.bps()),
		&mut pmc
	).unwrap();

	//blink
	loop {
		pin0.toggle().unwrap();
		pin1.toggle().unwrap();
		util::delayms(500);
		writeln!(serial, "Hello, world!\r").unwrap();
	}
}
