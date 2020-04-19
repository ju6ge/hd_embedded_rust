use atsame70q21::Peripherals as Same_p;
use cortex_m::Peripherals as Cortex_p;

use atsame70q21::{PMC, SDRAMC, PIOC};
use atsamx7x_hal::clock_gen::Clocks;
use atsamx7x_hal::gpio::*;
use atsamx7x_hal::delay;

pub fn init_sdram(pmc: &mut PMC, sdramc: &mut SDRAMC, clocks: &Clocks) {
	//Set Pins to correct mode
	let pioa = unsafe{Same_p::steal()}.PIOA.split(pmc);
	let pioc = unsafe{Same_p::steal()}.PIOC.split(pmc);
	let piod = unsafe{Same_p::steal()}.PIOD.split(pmc);
	let pioe = unsafe{Same_p::steal()}.PIOE.split(pmc);

	let _sdram_addr0 = pioc.p20.into_pmd0();
	let _sdram_addr1 = pioc.p21.into_pmd0();
	let _sdram_addr2 = pioc.p22.into_pmd0();
	let _sdram_addr3 = pioc.p23.into_pmd0();
	let _sdram_addr4 = pioc.p24.into_pmd0();
	let _sdram_addr5 = pioc.p25.into_pmd0();
	let _sdram_addr6 = pioc.p26.into_pmd0();
	let _sdram_addr7 = pioc.p27.into_pmd0();
	let _sdram_addr8 = pioc.p28.into_pmd0();
	let _sdram_addr9 = pioc.p29.into_pmd0();
	let _sdram_addr10 = piod.p13.into_pmd2();
	let _sdram_addr11 = pioc.p31.into_pmd0();
	let _sdram_addr12 = pioa.p18.into_pmd2();

	let _sdram_ba0 = pioa.p20.into_pmd2();
	let _sdram_ba1 = pioa.p0.into_pmd2();

	let _sdram_data0 = pioc.p0.into_pmd0();
	let _sdram_data1 = pioc.p1.into_pmd0();
	let _sdram_data2 = pioc.p2.into_pmd0();
	let _sdram_data3 = pioc.p3.into_pmd0();
	let _sdram_data4 = pioc.p4.into_pmd0();
	let _sdram_data5 = pioc.p5.into_pmd0();
	let _sdram_data6 = pioc.p6.into_pmd0();
	let _sdram_data7 = pioc.p7.into_pmd0();
	let _sdram_data8 = pioe.p0.into_pmd0();
	let _sdram_data9 = pioe.p1.into_pmd0();
	let _sdram_data10 = pioe.p2.into_pmd0();
	let _sdram_data11 = pioe.p3.into_pmd0();
	let _sdram_data12 = pioe.p4.into_pmd0();
	let _sdram_data13 = pioe.p5.into_pmd0();
	let _sdram_data14 = pioa.p15.into_pmd0();
	let _sdram_data15 = pioa.p16.into_pmd0();

	let _sdram_cke = piod.p14.into_pmd2();
	let _sdram_clk = piod.p23.into_pmd2();
	let _sdram_cas = piod.p17.into_pmd2();
	let _sdram_ras = piod.p16.into_pmd2();
	let _sdram_we = piod.p29.into_pmd2();
	let _sdram_dqml = pioc.p18.into_pmd0();
	let _sdram_dqmh = piod.p15.into_pmd2();

	//Setup SDRAM paramters
}
