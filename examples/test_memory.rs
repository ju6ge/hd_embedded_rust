#![no_std]
#![no_main]
#![feature(asm)]

extern crate panic_halt;
extern crate embedded_systems_board_uni_hd as board;

use cortex_m_rt::entry;

use atsamx7x_hal::target_device;
use atsamx7x_hal::gpio::*;
use atsamx7x_hal::clock_gen::{Clocks, MasterClockConfig, SlckConfig, MainckConfig, PllackConfig, UpllckConfig, SystemClockConfig, MasterDivider, MasterPrescale};
use atsamx7x_hal::serial::{config, Serial};
use atsamx7x_hal::time::{MegaHertz, *};

use core::fmt::Write;

use board::mem::init_sdram;

#[entry]
fn main() -> ! {
	let _cortex_p = cortex_m::Peripherals::take().unwrap();
	let peripherals = target_device::Peripherals::take().unwrap();

	let wdt = &peripherals.WDT;
	wdt.wdt_mr.write( |w| w.wddis().set_bit() );

	let mut pmc = peripherals.PMC;
	let mut supc = peripherals.SUPC;

	peripherals.EFC.eefc_fmr.write( |w| {
		unsafe {w.fws().bits(5);}
		w.cloe().set_bit();
		w.scod().clear_bit()
	});

	let clocks:Clocks = SystemClockConfig{
		slck_conf : SlckConfig::default(),
		mainck_conf : MainckConfig::default().use_crystal(MegaHertz(12).into()).disable_rc(),
		plla_conf : PllackConfig::default().from_divider(1, 49).startup_cycles(100),
		upll_conf : UpllckConfig::default().enable(),
		mck_conf :MasterClockConfig::default().src_pllack().from_divider(MasterPrescale::Pres2, MasterDivider::Div2)
	}.freeze(&mut pmc, &mut supc);

	let pioa = peripherals.PIOA.split(&mut pmc);
	let tx = pioa.p10.into_pmd0();
	let rx = pioa.p9.into_pmd0();

	let mut serial = Serial::uart0(
		peripherals.UART0,
		(tx, rx),
		config::UartConfig::default().baudrate(115_200.bps()),
		&clocks,
		&mut pmc
	).unwrap();

	let sdramc = peripherals.SDRAMC;
	let sdram = init_sdram(&mut pmc, sdramc, &clocks);

	let data : u32 = 42;
	let addr = unsafe{ sdram.start_address().offset(1) } as *mut u32;

	writeln!(serial, "{:?}\r", addr).unwrap();

	//enter infinite loop at end
	loop {
	}
}
