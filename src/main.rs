#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
//extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln};

use atsame70q21::{Peripherals, PMC, PIOB};

mod system_init;

fn delay20ns( ns20: i32 ) {
	for _i in 0..ns20 {
		asm::nop();
		asm::nop();
		asm::nop();
		asm::nop();
		asm::nop();
		asm::nop();
		asm::nop();
		asm::nop();
		asm::nop();
		asm::nop();
	}
}

fn delayus( us: i32 ) {
	for _i in 0..us {
		delay20ns(50);
	}
}

fn delayms( ms: i32 ) {
	for _i in 0..ms {
		delayus(1000);
	}
}

#[entry]
fn main() -> ! {
	let mut peripherals = Peripherals::take().unwrap();

	system_init::system_clock_init(&mut peripherals);

	//enable PIOC in PMC
	let pmc = peripherals.PMC;
	pmc.pmc_pcer0.write( |w| {
		w.pid12().set_bit()
		});

	let pioc = peripherals.PIOC;

	//enable pins to be controlled by pio and set to output mode
	pioc.pio_per.write( |w| { w.p9().set_bit() } );
	pioc.pio_oer.write( |w| { w.p9().set_bit() } );
	pioc.pio_per.write( |w| { w.p10().set_bit() } );
	pioc.pio_oer.write( |w| { w.p10().set_bit() } );
	pioc.pio_per.write( |w| { w.p19().set_bit() } );
	pioc.pio_oer.write( |w| { w.p19().set_bit() } );

	//blink
	loop {
		pioc.pio_sodr.write( |w| { w.p10().set_bit() } );
		pioc.pio_sodr.write( |w| { w.p19().set_bit() } );
		pioc.pio_codr.write( |w| { w.p9().set_bit() } );
    	delayms(1000);
		pioc.pio_codr.write( |w| { w.p10().set_bit() } );
		pioc.pio_codr.write( |w| { w.p19().set_bit() } );
		pioc.pio_sodr.write( |w| { w.p9().set_bit() } );
    	delayms(1000);
	}
}
