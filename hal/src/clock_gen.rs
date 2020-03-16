use crate::time::*;

/// Enum to select SLCK source
///
/// SLK can be driven by two different clocks at 32MHz
/// one of the clocks can be bypassed using an external clock
/// this gives us three different modes of operation
pub enum SlckSrc{
	SlowRC,
	CrystalOscillator,
	Bypass
}

#[derive(Debug)]
pub struct InvalidConfig;

/// Holds the current SLCK config
pub struct SlckConfig {
	src : SlckSrc,
	freq : Hertz
}

impl SlckConfig {
	fn use_rc(mut self) -> Self {
		self.src = SlckSrc::SlowRC;
		self.freq = KiloHertz(32).into();

		self
	}
	fn use_crystal(mut self) -> Self {
		self.src = SlckSrc::CrystalOscillator;
		self.freq = Hertz(32768);

		self
	}

	/// bypass and use own clk src -> you need to provide the frequency of that clk src!
	fn bypass(mut self, freq:Hertz) -> Self {
		self.src = SlckSrc::Bypass;
		self.freq = freq;

		self
	}
}

impl Default for SlckConfig {
	fn default() -> SlckConfig {
		SlckConfig{
			src: SlckSrc::SlowRC,
			freq: KiloHertz(32).into()
		}
	}
}

/// Enum to select MAINCK source
///
/// There are 2 different sources for MAINCK one of them can
/// be bypassed which leaves 3 options to select from.
pub enum MainckSrc {
	MainRC,
	MainCrystalOscillator,
	Bypass
}

/// Holds the current MAINCK config
pub struct MainckConfig {
	freq : Hertz,
	src : MainckSrc,
	startup_cycles : u16
}

impl MainckConfig {
	fn use_rc_4mhz(mut self) -> Self {
		self.src = MainckSrc::MainRC;
		self.freq = MegaHertz(4).into();

		self
	}

	fn use_rc_8mhz(mut self) -> Self {
		self.src = MainckSrc::MainRC;
		self.freq = MegaHertz(8).into();

		self
	}

	fn use_rc_12mhz(mut self) -> Self {
		self.src = MainckSrc::MainRC;
		self.freq = MegaHertz(12).into();

		self
	}

	/// The frequency of the crystal oscillator depends on how the devices is connected
	/// so you need to provide the frequency you configured with the hardware
	fn use_crystal(mut self, freq:Hertz) -> Self {
		self.src = MainckSrc::MainCrystalOscillator;
		self.freq = freq;

		self
	}

	/// bypass and use own clk src -> you need to provide the frequency of that clk src!
	fn bypass(mut self, freq:Hertz) -> Self {
		self.src = MainckSrc::Bypass;
		self.freq = freq;

		self
	}

	fn startup_cycles(mut self, cycles:u16) -> Self {
		self.startup_cycles = cycles;

		self
	}
}

impl Default for MainckConfig {
	fn default() -> MainckConfig {
		MainckConfig {
			src : MainckSrc::MainRC,
			freq : MegaHertz(12).into(),
			startup_cycles : 100
		}
	}
}

/// Holds the current PLLA clock config
pub struct PllackConfig {
	freq : Hertz,
	diva : u16,
	mula : u16,
	from_freq : bool,
	startup_cycles : u16
}

impl PllackConfig {
	/// configure PLLA based on target frequency, tries to calculate reasonable values for the divider
	fn from_freq(mut self, freq:Hertz) -> Self{
		self.freq = freq;
		self.diva = 0;
		self.mula = 0;
		self.from_freq = true;

		self
	}

	/// configure PLLA based on divider values, will calculate the resulting frequency
	fn from_param(mut self, diva:u16, mula:u16 ) -> Self {
		self.freq = Hertz(0);
		self.diva = diva;
		self.mula = mula;
		self.from_freq = false;

		self
	}

	fn startup_cycles(mut self, cycles:u16) -> Self {
		self.startup_cycles = cycles;

		self
	}
}

impl Default for PllackConfig {
	/// default for this is in disabled state since no sensible value can be assumed
	fn default() -> PllackConfig {
		PllackConfig {
			freq : Hertz(0),
			diva : 0,
			mula : 0,
			from_freq : false,
			startup_cycles : 0
		}
	}
}

pub enum UpllckSrcFreq {
	SRC12MHz,
	SRC16MHz
}

/// Holds the current UPLL clock config
pub struct UpllckConfig {
	src : UpllckSrcFreq,
	freq : Hertz,
	startup_cycles : u16
}

impl UpllckConfig {
	fn src_freq(mut self, src:UpllckSrcFreq) -> Self {
		self.src = src;
		self.freq = MegaHertz(480).into();

		self
	}

	fn startup_cycles(mut self, cycles:u16) -> Self {
		self.startup_cycles = cycles;

		self
	}
}

impl Default for UpllckConfig {
	fn default() -> UpllckConfig {
		UpllckConfig {
			src : UpllckSrcFreq::SRC12MHz,
			freq : MegaHertz(480).into(),
			startup_cycles : 3
		}
	}
}

pub enum SystemClocksSrc {
	SLCK,
	MAINCK,
	PLLACK,
	UPLLCK
}

/// Frozen clock frequencies
///
/// The existance of this value indicates that the clock configuration should no longer be changed
pub struct Clocks {
	mck: Hertz,
}
