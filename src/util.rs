use atsame70q21::{Peripherals};

use crate::system;

pub fn delayms(periph : &Peripherals, ms: u32 ) {
	system::start_rtt(periph, 0x20);
	while system::read_rtt(periph) < ms {
	};
}
