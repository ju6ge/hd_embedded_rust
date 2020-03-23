use atsame70q21::{Peripherals, RTT};

pub fn start_rtt(pres : u16) {
	let rtt = RTT::ptr();
	unsafe {
		(*rtt).rtt_mr.write(|w| {
			w.rtpres().bits(pres);
			w.rttdis().clear_bit();
			w.rttrst().set_bit()
		});
	}
}

pub fn read_rtt() -> u32 {
	let rtt = RTT::ptr();
	unsafe{(*rtt).rtt_vr.read().crtv().bits()}
}
