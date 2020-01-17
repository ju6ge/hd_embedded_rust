use atsame70q21::{Peripherals, RTT};

pub fn system_clock_init( periph: & Peripherals ) {
	let pmc = &periph.PMC;

	let efc = &periph.EFC;
	efc.eefc_fmr.write( |w| {
		unsafe{ w.fws().bits(5) };
		w.cloe().set_bit();
		w.scod().clear_bit()
	});

	//enable main crystal oscillator
	if pmc.ckgr_mor.read().moscsel().bit_is_clear() {
		pmc.ckgr_mor.write( |w| {
			w.key().passwd();
			unsafe {w.moscxtst().bits(100)};
			w.moscrcen().set_bit();
			w.moscxten().set_bit()
		});
		while pmc.pmc_sr.read().moscxts().bit_is_clear() {
		}
	}
	//measure
	pmc.ckgr_mcfr.write( |w| {
		w.ccss().set_bit();
		w.rcmeas().set_bit()
	});
	while pmc.ckgr_mcfr.read().mainfrdy().bit_is_clear() {
	}

	let mainf = pmc.ckgr_mcfr.read().mainf().bits();

	// switch to crystal oscillator
	if mainf > 0 {
		pmc.ckgr_mor.write( |w| {
			w.key().passwd();
			unsafe {w.moscxtst().bits(100)};
			w.moscrcen().set_bit();
			w.moscxten().set_bit();
			w.moscsel().set_bit()
		});
		while pmc.pmc_sr.read().moscsels().bit_is_clear() {
		}
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
		w.css().main_clk();
		w.pres().clk_2()
	});
	while pmc.pmc_sr.read().mckrdy().bit_is_clear() {
	}
	pmc.pmc_mckr.write( |w| {
		w.mdiv().pck_div2();
		w.pres().clk_2();
		w.css().plla_clk()
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

pub fn start_rtt(periph: & Peripherals, pres : u16) {
	let rtt = &periph.RTT;
	rtt.rtt_mr.write(|w| {
		unsafe {w.rtpres().bits(pres);}
		w.rttdis().clear_bit();
		w.rttrst().set_bit()
	});
}

pub fn read_rtt(periph: & Peripherals) -> u32 {
	let rtt = &periph.RTT;
	rtt.rtt_vr.read().crtv().bits()
}
