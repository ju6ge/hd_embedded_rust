//! Clock configuration

use crate::time::*;
use crate::target_device::PMC;
use crate::target_device::SUPC;

pub enum ClockCalcStrategy {
	FromFrequency,
	FromDivider
}

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
	strategy : ClockCalcStrategy,
	startup_cycles : u16
}

impl PllackConfig {
	/// configure PLLA based on target frequency, tries to calculate reasonable values for the divider
	fn from_freq(mut self, freq:Hertz) -> Self{
		self.freq = freq;
		self.diva = 0;
		self.mula = 0;
		self.strategy = ClockCalcStrategy::FromFrequency;

		self
	}

	/// configure PLLA based on divider values, will calculate the resulting frequency
	fn from_divider(mut self, diva:u16, mula:u16 ) -> Self {
		self.freq = Hertz(0);
		self.diva = diva;
		self.mula = mula;
		self.strategy = ClockCalcStrategy::FromDivider;

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
			strategy : ClockCalcStrategy::FromDivider,
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
	enable : bool
}

impl UpllckConfig {
	fn enable(mut self) -> Self {
		self.enable = true;

		self
	}

	fn disable(mut self) -> Self {
		self.enable = false;

		self
	}

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
	UPLLCKDIV,
	MCK
}

pub enum MasterPrescale {
	Pres1,
	Pres2,
	Pres3,
	Pres4,
	Pres8,
	Pres16,
	Pres32,
	Pres64
}

pub enum MasterDivider {
	Div1,
	Div2,
	Div3,
	Div4
}

pub enum UpllDiv{
	Div1,
	Div2
}

pub struct MasterClockConfig {
	src : SystemClocksSrc,
	pres : MasterPrescale,
	mdiv : MasterDivider,
	uplldiv : UpllDiv,
	freq_cpu : Option<Hertz>,
	freq_mck : Option<Hertz>,
	strategy : ClockCalcStrategy
}


impl MasterClockConfig {
	fn src_slck(mut self) -> Self {
		self.src = SystemClocksSrc::SLCK;

		self
	}

	fn src_mainck(mut self) -> Self {
		self.src = SystemClocksSrc::MAINCK;

		self

	}

	fn src_pllack(mut self) -> Self {
		self.src = SystemClocksSrc::PLLACK;

		self

	}

	fn src_upllckdiv(mut self) -> Self {
		self.src = SystemClocksSrc::UPLLCKDIV;

		self

	}

	fn set_uplldiv(mut self, div:UpllDiv) -> Self {
		self.uplldiv = div;

		self
	}

	fn from_divider(mut self, pres:MasterPrescale, mdiv:MasterDivider) -> Self {
		self.strategy = ClockCalcStrategy::FromDivider;
		self.pres = pres;
		self.mdiv = mdiv;

		self
	}

	fn from_freq(mut self, cpu_freq:Hertz, mck_freq:Hertz) -> Self {
		self.strategy = ClockCalcStrategy::FromFrequency;
		self.freq_cpu = Some(cpu_freq);
		self.freq_mck = Some(mck_freq);

		self
	}
}

impl Default for MasterClockConfig {
	fn default() -> MasterClockConfig {
		MasterClockConfig{
			src : SystemClocksSrc::MAINCK,
			pres : MasterPrescale::Pres1,
			mdiv : MasterDivider::Div1,
			uplldiv : UpllDiv::Div1,
			freq_cpu : None,
			freq_mck : None,
			strategy : ClockCalcStrategy::FromDivider
		}
	}
}

pub struct SystemClockConfig {
	slck_conf : SlckConfig,
	mainck_conf : MainckConfig,
	plla_conf : PllackConfig,
	upll_conf : UpllckConfig,
	mck_conf : MasterClockConfig
}

impl SystemClockConfig {
	/// Freezes the clock configuration by making it effective
	pub fn freeze(&self, pmc: &mut PMC, supc: &mut SUPC) -> Clocks {
		// Slow Clock configuration

		// Main Clock configuration

		// Plla configuration

		// Upll configuration

		// Master Clock configuration

		Clocks {

		}
	}
}

/// Frozen clock frequencies
///
/// The existance of this value indicates that the clock configuration should no longer be changed
pub struct Clocks {
	/// Processor Clock frequency
	hclk : Hertz,

	/// SysTick frequency
	sys_tick : Hertz,

	/// Free Running Processor Clock frequency
	fclk : Hertz,

	/// Master Clock frequency
	mck: Hertz,

	/// Slow Clock frequency
	slck : Hertz,

	/// Main Clock frequency
	mainck : Hertz,

	/// Plla Clock frequency
	plla : Hertz,

	/// Uplldiv Clock frequency
	uplldiv : Hertz,

	///UPLL usb clock frequency
	usb_480 : Hertz
}

impl Clocks {
	/// Returns Processor frequency
	fn hclk(&self) -> Hertz {
		self.hclk
	}

	/// Returns SysTick frequency
	fn sys_tick(&self) -> Hertz {
		self.sys_tick
	}

	/// Returns Free Running Processor Clock frequency
	fn fclk(&self) -> Hertz {
		self.fclk
	}

	/// Returns Master Clock frequency
	fn mck(&self) -> Hertz {
		self.mck
	}

	/// Returns Slow Clock frequency
	fn slck(&self) -> Hertz {
		self.slck
	}

	/// Returns Mainck frequency
	fn mainck(&self) -> Hertz {
		self.mainck
	}

	/// Returns Plla frequency
	fn plla(&self) -> Hertz {
		self.plla
	}

	/// Returns Uplldiv frequency
	fn uplldiv(&self) -> Hertz {
		self.uplldiv
	}

	/// Returns UPLL usb clock frequency
	fn usb_480(&self) -> Hertz {
		self.usb_480
	}
}
