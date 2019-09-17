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

use atsams70q21::{Peripherals, PMC};

#[entry]
fn main() -> ! {
    let peripherals = atsams70q21::Peripherals::take().unwrap();

    //enable PIOB in PMC
    let pmc = peripherals.PMC;
    pmc.pmc_pcer0.write( |w| { w.pid11().set_bit() });

    let piob = peripherals.PIOB;
    //set p5 to output pin

    //set led PB5 on
    piob.pio_oer.write( |w| { w.p5().set_bit() } );

    loop {
        // your code goes here
    }
}
