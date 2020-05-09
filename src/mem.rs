use atsame70q21::Peripherals as Same_p;
use cortex_m::Peripherals as Cortex_p;

use atsame70q21::{PMC, SDRAMC};
use atsamx7x_hal::clock_gen::Clocks;
use atsamx7x_hal::sdram::*;
use atsamx7x_hal::gpio::*;
use atsamx7x_hal::time::{NanoSeconds, PicoSeconds};
use atsamx7x_hal::delay;

pub struct SdramPins {
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

		_b0 : pioa::PA20<PeripheralCntr<PeriphC>>,
		_b1 : pioa::PA0<PeripheralCntr<PeriphC>>,

		_msk0 : pioc::PC18<PeripheralCntr<PeriphA>>,
		_msk1 : piod::PD15<PeripheralCntr<PeriphC>>,

		_clk : piod::PD23<PeripheralCntr<PeriphC>>,
		_cke : piod::PD14<PeripheralCntr<PeriphC>>,
		_ras : piod::PD16<PeripheralCntr<PeriphC>>,
		_cas : piod::PD17<PeripheralCntr<PeriphC>>,
		_we : piod::PD29<PeripheralCntr<PeriphC>>
}

impl Default for SdramPins {
	fn default() -> SdramPins {
		let same = unsafe{Same_p::steal()};
		let mut pmc = same.PMC;
		//Set Pins to correct mode
		let pioa = same.PIOA.split(&mut pmc);
		let pioc = same.PIOC.split(&mut pmc);
		let piod = same.PIOD.split(&mut pmc);
		let pioe = same.PIOE.split(&mut pmc);

		SdramPins {
			_a0 : pioc.p20.into_pmd0(),
			_a1 : pioc.p21.into_pmd0(),
			_a2 : pioc.p22.into_pmd0(),
			_a3 : pioc.p23.into_pmd0(),
			_a4 : pioc.p24.into_pmd0(),
			_a5 : pioc.p25.into_pmd0(),
			_a6 : pioc.p26.into_pmd0(),
			_a7 : pioc.p27.into_pmd0(),
			_a8 : pioc.p28.into_pmd0(),
			_a9 : pioc.p29.into_pmd0(),
			_a10 : piod.p13.into_pmd2(),
			_a11 : pioc.p31.into_pmd0(),
			_a12 : pioa.p18.into_pmd2(),

			_b0 : pioa.p20.into_pmd2(),
			_b1 : pioa.p0.into_pmd2(),

			_d0 : pioc.p0.into_pmd0(),
			_d1 : pioc.p1.into_pmd0(),
			_d2 : pioc.p2.into_pmd0(),
			_d3 : pioc.p3.into_pmd0(),
			_d4 : pioc.p4.into_pmd0(),
			_d5 : pioc.p5.into_pmd0(),
			_d6 : pioc.p6.into_pmd0(),
			_d7 : pioc.p7.into_pmd0(),
			_d8 : pioe.p0.into_pmd0(),
			_d9 : pioe.p1.into_pmd0(),
			_d10 : pioe.p2.into_pmd0(),
			_d11 : pioe.p3.into_pmd0(),
			_d12 : pioe.p4.into_pmd0(),
			_d13 : pioe.p5.into_pmd0(),
			_d14 : pioa.p15.into_pmd0(),
			_d15 : pioa.p16.into_pmd0(),

			_cke : piod.p14.into_pmd2(),
			_clk : piod.p23.into_pmd2(),
			_cas : piod.p17.into_pmd2(),
			_ras : piod.p16.into_pmd2(),
			_we : piod.p29.into_pmd2(),
			_msk0 : pioc.p18.into_pmd0(),
			_msk1 : piod.p15.into_pmd2()
		}
	}
}

impl atsamx7x_hal::sdram::SdramPins for SdramPins {}

pub fn init_sdram(pmc: &mut PMC, sdramc: SDRAMC, clocks: &Clocks)
->Sdram<SdramPins>{
	let pins = SdramPins::default();

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

	let sdram = Sdram::setup(sdramc, pins, conf, clocks, pmc).unwrap();
	sdram
}
