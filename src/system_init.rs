use atsame70q21::{Peripherals, PMC, PIOB};

pub fn system_clock_init( periph: &mut Peripherals ) {
	let pmc = &periph.PMC;

	let efc = &periph.EFC;
	efc.eefc_fmr.write( |w| unsafe{ w.fws().bits(5) } );

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
	}

	// switch to crystal oscillator
	pmc.ckgr_mor.write( |w| {
		w.key().passwd();
		unsafe {w.moscxtst().bits(8)};
		w.moscrcen().clear_bit();
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
		w.mdiv().pck_div2();
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
