use cortex_m::asm;

pub fn delay20ns( ns20: i32 ) {
	for _i in 0..ns20 {
		asm::nop();
		asm::nop();
		asm::nop();
		asm::nop();
		asm::nop();
	}
}

pub fn delayus( us: i32 ) {
	for _i in 0..us {
		delay20ns(50);
	}
}

pub fn delayms( ms: i32 ) {
	for _i in 0..ms {
		delayus(1000);
	}
}
