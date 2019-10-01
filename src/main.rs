#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln};

use atsams70q21::{Peripherals};


fn system_clock_init( periph: &mut Peripherals ) {
	let pmc = &periph.PMC;

	//enable main crystal oscillator
	if pmc.ckgr_mor.read().moscsel().bit_is_clear() {
		pmc.ckgr_mor.write( |w| {
			w.key().passwd();
			unsafe {w.moscxtst().bits(8)};
			w.moscrcen().set_bit();
			w.moscxten().set_bit()
		});
		while pmc.pmc_sr.read().moscxts().bit_is_clear() {
		}
		hprintln!("Enabled main crystal oscillator!");
	}

	// switch to crystal oscillator
	//pmc.ckgr_mor.write( |w| w.moscsel().set_bit() );
	//while pmc.pmc_sr.read().moscsels().bit_is_clear() {
	//}

	pmc.pmc_mckr.write( |w| w.css().main_clk() );
	while pmc.pmc_sr.read().mckrdy().bit_is_clear() {
	}
	hprintln!("Switched to main crystal oscillator!");

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
		w.css().main_clk();
		w.pres().clk_2()
	});
	while pmc.pmc_sr.read().mckrdy().bit_is_clear() {
	}
	hprintln!("Enabled plla!");

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
	hprintln!("Enabled usb!");
}

#[entry]
fn main() -> ! {
	let mut peripherals = Peripherals::take().unwrap();

	hprintln!("Start init clock!");
	system_clock_init(&mut peripherals);
	hprintln!("Clock init finished!");

	//enable PIOB in PMC
	//let pmc = peripherals.PMC;
	//pmc.pmc_pcer0.write( |w| { w.pid11().set_bit() });

	//let piob = peripherals.PIOB;
	////set p5 to output pin

	////set led PB5 on
	//piob.pio_oer.write( |w| { w.p5().set_bit() } );

	loop {
		// your code goes here
	}
}
