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


fn system_clock_init( periph: &mut Peripherals ) {
	let pmc = &periph.PMC;

	let efc = &periph.EFC;
	efc.eefc_fmr.write( |w| unsafe{ w.fws().bits(5) } );

	//enable main crystal oscillator
	if pmc.ckgr_mor.read().moscsel().bit_is_clear() {
		pmc.ckgr_mor.write( |w| {
			w.key().passwd();
			unsafe {w.moscxtst().bits(255)};
			w.moscrcen().set_bit();
			w.moscxten().set_bit()
		});
		while pmc.pmc_sr.read().moscxts().bit_is_clear() {
		}
	}

	// switch to crystal oscillator
	pmc.ckgr_mor.write( |w| {
		w.key().passwd();
		unsafe {w.moscxtst().bits(255)};
		w.moscrcen().set_bit();
		w.moscxten().set_bit();
		w.moscsel().set_bit()
	});
	while pmc.pmc_sr.read().moscsels().bit_is_clear() {
	}

	pmc.pmc_mckr.write( |w| w.css().main_clk() );
	while pmc.pmc_sr.read().mckrdy().bit_is_clear() {
	}

	//enable PLLA
	pmc.ckgr_pllar.write( |w| {
		w.one().set_bit();
		unsafe { w.mula().bits(49) };
		unsafe { w.diva().bits(1) };
		unsafe { w.pllacount().bits(63) }
	});
	while pmc.pmc_sr.read().locka().bit_is_clear() {
	}

	pmc.pmc_mckr.write( |w| {
		w.mdiv().bits(1);
		w.css().plla_clk();
		w.pres().clk_2()
	});
	while pmc.pmc_sr.read().mckrdy().bit_is_clear() {
	}
	
	pmc.ckgr_uckr.write( |w| {
		w.upllen().set_bit();
		unsafe{ w.upllcount().bits(3) }
	});
	while pmc.pmc_sr.read().locku().bit_is_clear() {
	}

	pmc.pmc_usb.write( |w| {
		w.usbs().set_bit();
		unsafe { w.usbdiv().bits(0) }
	});

	pmc.pmc_scer.write( |w| w.usbclk().set_bit() );
}

#[entry]
fn main() -> ! {
	let mut peripherals = Peripherals::take().unwrap();

	system_clock_init(&mut peripherals);

	//enable PIOB in PMC
	let pmc = peripherals.PMC;
	pmc.pmc_pcer0.write( |w| {
		w.pid12().clear_bit()
		});

	let pioc = peripherals.PIOC;

	//set p5 to output pin
	pioc.pio_per.write( |w| { w.p10().set_bit() } );
	pioc.pio_oer.write( |w| { w.p10().set_bit() } );
	pioc.pio_per.write( |w| { w.p19().set_bit() } );
	pioc.pio_oer.write( |w| { w.p19().set_bit() } );
	
	//set led PB5 on

	loop {
		// your code goes here
		pioc.pio_codr.write( |w| { w.p10().set_bit() } );
		pioc.pio_sodr.write( |w| { w.p19().set_bit() } );
	}
}
