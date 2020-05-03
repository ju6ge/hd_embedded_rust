use atsame70q21::Peripherals as Same_p;
use cortex_m::Peripherals as Cortex_p;

use atsame70q21::{PMC, SDRAMC};
use atsamx7x_hal::clock_gen::Clocks;
use atsamx7x_hal::sdram::*;
use atsamx7x_hal::gpio::*;
use atsamx7x_hal::time::{NanoSeconds, PicoSeconds};
use atsamx7x_hal::delay;

pub struct NoCs;
impl CS for NoCs {}

pub fn init_sdram(pmc: &mut PMC, sdramc: SDRAMC, clocks: &Clocks)
->Sdram<(
		pioc::PC20<PeripheralCntr<PeriphA>>,
		pioc::PC21<PeripheralCntr<PeriphA>>,
		pioc::PC22<PeripheralCntr<PeriphA>>,
		pioc::PC23<PeripheralCntr<PeriphA>>,
		pioc::PC24<PeripheralCntr<PeriphA>>,
		pioc::PC25<PeripheralCntr<PeriphA>>,
		pioc::PC26<PeripheralCntr<PeriphA>>,
		pioc::PC27<PeripheralCntr<PeriphA>>,
		pioc::PC28<PeripheralCntr<PeriphA>>,
		pioc::PC29<PeripheralCntr<PeriphA>>,
		piod::PD13<PeripheralCntr<PeriphC>>,
		pioc::PC31<PeripheralCntr<PeriphA>>,
		pioa::PA18<PeripheralCntr<PeriphC>>,
		pioc::PC0<PeripheralCntr<PeriphA>>,
		pioc::PC1<PeripheralCntr<PeriphA>>,
		pioc::PC2<PeripheralCntr<PeriphA>>,
		pioc::PC3<PeripheralCntr<PeriphA>>,
		pioc::PC4<PeripheralCntr<PeriphA>>,
		pioc::PC5<PeripheralCntr<PeriphA>>,
		pioc::PC6<PeripheralCntr<PeriphA>>,
		pioc::PC7<PeripheralCntr<PeriphA>>,
		pioe::PE0<PeripheralCntr<PeriphA>>,
		pioe::PE1<PeripheralCntr<PeriphA>>,
		pioe::PE2<PeripheralCntr<PeriphA>>,
		pioe::PE3<PeripheralCntr<PeriphA>>,
		pioe::PE4<PeripheralCntr<PeriphA>>,
		pioe::PE5<PeripheralCntr<PeriphA>>,
		pioa::PA15<PeripheralCntr<PeriphA>>,
		pioa::PA16<PeripheralCntr<PeriphA>>,
		pioa::PA20<PeripheralCntr<PeriphC>>,
		pioa::PA0<PeripheralCntr<PeriphC>>,
		pioc::PC18<PeripheralCntr<PeriphA>>,
		piod::PD15<PeripheralCntr<PeriphC>>,
		piod::PD23<PeripheralCntr<PeriphC>>,
		piod::PD14<PeripheralCntr<PeriphC>>,
		NoCs,
		piod::PD16<PeripheralCntr<PeriphC>>,
		piod::PD17<PeripheralCntr<PeriphC>>,
		piod::PD29<PeripheralCntr<PeriphC>>
	)>
	{
	let same = unsafe{Same_p::steal()};
	//Set Pins to correct mode
	let pioa = same.PIOA.split(pmc);
	let pioc = same.PIOC.split(pmc);
	let piod = same.PIOD.split(pmc);
	let pioe = same.PIOE.split(pmc);

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

	let _sdram_cs = NoCs{};

	let pins = (
		_sdram_addr0,
		_sdram_addr1,
		_sdram_addr2,
		_sdram_addr3,
		_sdram_addr4,
		_sdram_addr5,
		_sdram_addr6,
		_sdram_addr7,
		_sdram_addr8,
		_sdram_addr9,
		_sdram_addr10,
		_sdram_addr11,
		_sdram_addr12,
		_sdram_data0,
		_sdram_data1,
		_sdram_data2,
		_sdram_data3,
		_sdram_data4,
		_sdram_data5,
		_sdram_data6,
		_sdram_data7,
		_sdram_data8,
		_sdram_data9,
		_sdram_data10,
		_sdram_data11,
		_sdram_data12,
		_sdram_data13,
		_sdram_data14,
		_sdram_data15,
		_sdram_ba0,
		_sdram_ba1,
		_sdram_dqml,
		_sdram_dqmh,
		_sdram_clk,
		_sdram_cke,
		_sdram_cs,
		_sdram_ras,
		_sdram_cas,
		_sdram_we
		);


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
