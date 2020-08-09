#![no_std]
#![no_main]
#![feature(asm)]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::vec::*;
use core::alloc::Layout;
use cortex_m::asm;
use linked_list_allocator::LockedHeap;

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
	asm::bkpt();

	loop {}
}

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

extern crate panic_halt;
extern crate embedded_systems_board_uni_hd as board;

use cortex_m_rt::entry;
use atsamx7x_hal::target_device;
use atsamx7x_hal::gpio::*;
use atsamx7x_hal::clock_gen::{Clocks, MasterClockConfig, SlckConfig, MainckConfig, PllackConfig, UpllckConfig, SystemClockConfig, MasterDivider, MasterPrescale};
use atsamx7x_hal::serial::{config, Serial};
use atsamx7x_hal::time::{MegaHertz, *};
use atsamx7x_hal::delay::Delay;
use atsamx7x_hal::ebi::{ExternalBusInterface};
use embedded_hal::blocking::delay::{DelayMs};

use core::fmt::Write;

use board::mem::init_sdram;
use board::mem::{EbiPins};

#[entry]
fn main() -> ! {
	let cortex_p = cortex_m::Peripherals::take().unwrap();
	let peripherals = target_device::Peripherals::take().unwrap();

	let wdt = &peripherals.WDT;
	wdt.wdt_mr.write( |w| w.wddis().set_bit() );

	let mut pmc = peripherals.PMC;
	let mut supc = peripherals.SUPC;

	let mut scb = cortex_p.SCB;
	let mut cpuid = cortex_p.CPUID;
	scb.enable_icache();
	scb.disable_dcache(&mut cpuid);

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

	let mut delay = Delay::new(cortex_p.SYST, &clocks);

	let pins = EbiPins::default();
	let ebi = ExternalBusInterface::new(&pins);

	//setup sdram and write and read some values for testing
	let sdramc = peripherals.SDRAMC;
	let sdram = init_sdram(&mut pmc, sdramc, &clocks, &ebi);

	let data : u16 = 42 + 256;
	let addr = unsafe{ sdram.start_address().offset(300) } as *mut u16;

	writeln!(serial, "-----------------------------\r").ok();
	writeln!(serial, "{:?}\r", addr).ok();
	unsafe {core::ptr::write_volatile(addr, data)};
	unsafe {core::ptr::write_volatile(addr.offset(1), data+512)};

	delay.delay_ms(1 as u32);

	writeln!(serial, "{:?}\r", unsafe{ *(addr.offset(1)) }).ok();
	writeln!(serial, "{:?}\r", unsafe{ *((addr as *mut u8).offset(1) as *mut u16) }).ok();
	writeln!(serial, "{:?}\r", unsafe{ *addr }).ok();

	//zero all memory
	for i in 0..sdram.size()/2 {
		unsafe {
			let a = (sdram.start_address() as *const u16).offset(i as isize) as *mut u16;
			*a = 0;
		}
	}

	//setup allocator
	unsafe {
		HEAP_ALLOCATOR.lock().init(sdram.start_address() as usize, sdram.start_address() as usize + sdram.size() as usize);
	}

	writeln!(serial, "-----------------------------\r").ok();
	for i in 0..30 {
		unsafe {
			writeln!(serial, "{:?}\r", *(sdram.start_address() as *const u16).offset(i)).ok();
		}
	}
	writeln!(serial, "-----------------------------\r").ok();

	let mut xs = Vec::new();
	xs.reserve(10);
	xs.push(42);
	xs.push(13);
	xs.push(512);
	xs.push(1337);
	xs.push(96);
	xs.push(11);
	xs.push(-1);
	xs.push(69);
	//xs.push(420);
	//xs.push(2048);
	//xs.push(81);
	//xs.push(33);

	writeln!(serial, "{:?}\r", xs).ok();
	writeln!(serial, "{:?}\r", xs.as_ptr()).ok();

	let p = xs.as_ptr() as *const i32;
	for i in 0..30 {
		unsafe {
			writeln!(serial, "{:?}\r", *p.offset(i)).ok();
		}
	}
	writeln!(serial, "-----------------------------\r").ok();

	//enter infinite loop at end
	loop {
	}
}
