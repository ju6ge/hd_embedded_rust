#![no_std]
#![no_main]
#![feature(asm)]

// pick a panicking behavior
extern crate panic_halt;

//#[cfg(debug_assertions)]
//extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln};

use atsame70q21::{Peripherals};

use hal::serial::Serial0;
use embedded_hal::serial::Write;

mod system;
mod util;
mod debug;

fn leds_on(peripherals : &Peripherals) {
	let pioc = &peripherals.PIOC;

	pioc.pio_sodr.write( |w| {
		w.p10().set_bit();
		w.p19().set_bit()
	});
	pioc.pio_codr.write( |w| { w.p9().set_bit() } );
}

fn leds_off(peripherals : &Peripherals) {
	let pioc = &peripherals.PIOC;

	pioc.pio_codr.write( |w| {
		w.p10().set_bit();
		w.p19().set_bit()
	});
	pioc.pio_sodr.write( |w| { w.p9().set_bit() } );
}

#[entry]
fn main() -> ! {
	let mut peripherals = Peripherals::take().unwrap();

	system::system_clock_init(&mut peripherals);
	debug!("System clock initialized!");

	//enable PIOC in PMC
	let pmc = &peripherals.PMC;
	pmc.pmc_pcer0.write( |w| {
		w.pid12().set_bit()
	});

	let wdt = &peripherals.WDT;
	wdt.wdt_mr.write( |w| w.wddis().set_bit() );

	let pioc = &peripherals.PIOC;
	//enable pins to be controlled by pio and set to output mode
	pioc.pio_per.write( |w| {
		w.p9().set_bit();
		w.p10().set_bit();
		w.p19().set_bit()
	});
	pioc.pio_oer.write( |w| {
		w.p9().set_bit();
		w.p10().set_bit();
		w.p19().set_bit()
	});


	//blink
	loop {
		leds_on(&peripherals);
		util::delayms(&peripherals, 500);
    	leds_off(&peripherals);
		util::delayms(&peripherals, 500);
	}
}
