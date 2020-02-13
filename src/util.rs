use atsame70q21::{Peripherals};

use crate::system;

pub fn delayms(ms: u32 ) {
	system::start_rtt(0x20);
	while system::read_rtt() < ms {
	};
}
