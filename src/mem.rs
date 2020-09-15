use atsame70q21::Peripherals as Same_p;
use atsame70q21::{PMC, SDRAMC};

use atsamx7x_hal::clock_gen::Clocks;
use atsamx7x_hal::sdram::*;
use atsamx7x_hal::ebi::*;
use atsamx7x_hal::gpio::*;
use atsamx7x_hal::time::{NanoSeconds, PicoSeconds};

pub struct EbiPins {
	    // address pins
		_a0 : pioc::PC20<PeripheralCntr<PeriphA>>,
		_a1 : pioc::PC21<PeripheralCntr<PeriphA>>,
		_a2 : pioc::PC22<PeripheralCntr<PeriphA>>,
		_a3 : pioc::PC23<PeripheralCntr<PeriphA>>,
		_a4 : pioc::PC24<PeripheralCntr<PeriphA>>,
		_a5 : pioc::PC25<PeripheralCntr<PeriphA>>,
		_a6 : pioc::PC26<PeripheralCntr<PeriphA>>,
		_a7 : pioc::PC27<PeripheralCntr<PeriphA>>,
		_a8 : pioc::PC28<PeripheralCntr<PeriphA>>,
		_a9 : pioc::PC29<PeripheralCntr<PeriphA>>,
		_a10 : piod::PD13<PeripheralCntr<PeriphC>>,
		_a11 : pioc::PC31<PeripheralCntr<PeriphA>>,
		_a12 : pioa::PA18<PeripheralCntr<PeriphC>>,

		//data pins
		_d0 : pioc::PC0<PeripheralCntr<PeriphA>>,
		_d1 : pioc::PC1<PeripheralCntr<PeriphA>>,
		_d2 : pioc::PC2<PeripheralCntr<PeriphA>>,
		_d3 : pioc::PC3<PeripheralCntr<PeriphA>>,
		_d4 : pioc::PC4<PeripheralCntr<PeriphA>>,
		_d5 : pioc::PC5<PeripheralCntr<PeriphA>>,
		_d6 : pioc::PC6<PeripheralCntr<PeriphA>>,
		_d7 : pioc::PC7<PeripheralCntr<PeriphA>>,
		_d8 : pioe::PE0<PeripheralCntr<PeriphA>>,
		_d9 : pioe::PE1<PeripheralCntr<PeriphA>>,
		_d10 : pioe::PE2<PeripheralCntr<PeriphA>>,
		_d11 : pioe::PE3<PeripheralCntr<PeriphA>>,
		_d12 : pioe::PE4<PeripheralCntr<PeriphA>>,
		_d13 : pioe::PE5<PeripheralCntr<PeriphA>>,
		_d14 : pioa::PA15<PeripheralCntr<PeriphA>>,
		_d15 : pioa::PA16<PeripheralCntr<PeriphA>>,

		//bank select
		_b0 : pioa::PA20<PeripheralCntr<PeriphC>>,
		_b1 : pioa::PA0<PeripheralCntr<PeriphC>>,

		// mask
		_msk0 : pioc::PC18<PeripheralCntr<PeriphA>>,
		_msk1 : piod::PD15<PeripheralCntr<PeriphC>>,

		//clk
		_clk : piod::PD23<PeripheralCntr<PeriphC>>,

		// sdram
		_cke : piod::PD14<PeripheralCntr<PeriphC>>,
		_ras : piod::PD16<PeripheralCntr<PeriphC>>,
		_cas : piod::PD17<PeripheralCntr<PeriphC>>,
		_we : piod::PD29<PeripheralCntr<PeriphC>>,

		// sram
		_ior : pioc::PC11<PeripheralCntr<PeriphA>>,
		_iow : pioc::PC8<PeripheralCntr<PeriphA>>,
		_cs_eth : pioc::PC14<PeripheralCntr<PeriphA>>,
		_cs_lcd : pioa::PA22<PeripheralCntr<PeriphC>>

		}

impl Default for EbiPins {
	fn default() -> EbiPins {
		let same = unsafe{Same_p::steal()};
		let mut pmc = same.PMC;
		//Set Pins to correct mode
		let pioa = same.PIOA.split(&mut pmc);
		let pioc = same.PIOC.split(&mut pmc);
		let piod = same.PIOD.split(&mut pmc);
		let pioe = same.PIOE.split(&mut pmc);

		EbiPins {
			_a0 : pioc.p20.into_peripheral_a(),
			_a1 : pioc.p21.into_peripheral_a(),
			_a2 : pioc.p22.into_peripheral_a(),
			_a3 : pioc.p23.into_peripheral_a(),
			_a4 : pioc.p24.into_peripheral_a(),
			_a5 : pioc.p25.into_peripheral_a(),
			_a6 : pioc.p26.into_peripheral_a(),
			_a7 : pioc.p27.into_peripheral_a(),
			_a8 : pioc.p28.into_peripheral_a(),
			_a9 : pioc.p29.into_peripheral_a(),
			_a10 : piod.p13.into_peripheral_c(),
			_a11 : pioc.p31.into_peripheral_a(),
			_a12 : pioa.p18.into_peripheral_c(),

			_b0 : pioa.p20.into_peripheral_c(),
			_b1 : pioa.p0.into_peripheral_c(),

			_d0 : pioc.p0.into_peripheral_a(),
			_d1 : pioc.p1.into_peripheral_a(),
			_d2 : pioc.p2.into_peripheral_a(),
			_d3 : pioc.p3.into_peripheral_a(),
			_d4 : pioc.p4.into_peripheral_a(),
			_d5 : pioc.p5.into_peripheral_a(),
			_d6 : pioc.p6.into_peripheral_a(),
			_d7 : pioc.p7.into_peripheral_a(),
			_d8 : pioe.p0.into_peripheral_a(),
			_d9 : pioe.p1.into_peripheral_a(),
			_d10 : pioe.p2.into_peripheral_a(),
			_d11 : pioe.p3.into_peripheral_a(),
			_d12 : pioe.p4.into_peripheral_a(),
			_d13 : pioe.p5.into_peripheral_a(),
			_d14 : pioa.p15.into_peripheral_a(),
			_d15 : pioa.p16.into_peripheral_a(),

			_cke : piod.p14.into_peripheral_c(),
			_clk : piod.p23.into_peripheral_c(),
			_cas : piod.p17.into_peripheral_c(),
			_ras : piod.p16.into_peripheral_c(),
			_we : piod.p29.into_peripheral_c(),
			_msk0 : pioc.p18.into_peripheral_a(),
			_msk1 : piod.p15.into_peripheral_c(),

			_ior : pioc.p11.into_peripheral_a(),
			_iow : pioc.p8.into_peripheral_a(),
			_cs_eth : pioc.p14.into_peripheral_a(),
			_cs_lcd : pioa.p22.into_peripheral_c()
		}
	}
}

impl atsamx7x_hal::ebi::EBIPins for EbiPins{}

pub fn init_sdram(pmc: &mut PMC, sdramc: SDRAMC, clocks: &Clocks, ebi: &ExternalBusInterface) -> Sdram{

	let conf = SdramConfig {
		banks : SdramBanks::Bank4,
		rows : SdramRows::Rows8K,
		columns : SdramColumns::Columns512,
		alignment: SdramAlignment::Unaligned,
		latency: SdramCasLatency::Latency3,
		timing : SdramTiming {
			twr : NanoSeconds(12).into(),
			trc : NanoSeconds(60).into(),
			trp : NanoSeconds(18).into(),
			trcd: NanoSeconds(18).into(),
			tras: NanoSeconds(42).into(),
			txsr: PicoSeconds(61500),
			refresh : NanoSeconds(7812).into()
		},
	};

	let sdram = Sdram::setup(sdramc, &ebi, conf, clocks, pmc).unwrap();
	sdram
}
