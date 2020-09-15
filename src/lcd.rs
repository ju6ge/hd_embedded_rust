use atsamx7x_hal::smc::{Smc, *};
use atsamx7x_hal::time::{NanoSeconds};
use atsamx7x_hal::gpio::*;


pub struct LCD {
	backlight_pwm_pin: pioa::PA1<PeripheralCntr<PeriphB>>
}

impl LCD {

}

pub fn setup_lcd(smc: &mut Smc, lcd_pin: pioa::PA1<PeripheralCntr<PeriphB>>) -> LCD{
	let conf = SmcDeviceConfig{
		mode: SmcDeviceMode::default()
			.bus_width_16_bit()
			.read_mode_rd()
			.write_mode_we(),
		// setup time for signals is negligable, when setting to 0 at least 1 cycle is used
		setup: SmcDeviceSetupTimings {
			read: NanoSeconds(0).into(),
			read_cs: NanoSeconds(0).into(),
			write: NanoSeconds(5).into(),
			write_cs: NanoSeconds(5).into()
		},
		pulse: SmcDevicePulseTimings {
			read: NanoSeconds(500).into(),
			read_cs: NanoSeconds(500).into(),
			write: NanoSeconds(50).into(),
			write_cs: NanoSeconds(50).into()
		},
		cycle: SmcDeviceCycleTimings {
			read: NanoSeconds(1000).into(),
			write: NanoSeconds(100).into()
		}
	};

	smc.setup_device(SmcDeviceSelect::SmcDevice2, conf).ok();
	

	let lcd = LCD{
		backlight_pwm_pin: lcd_pin
	};

	lcd
}
