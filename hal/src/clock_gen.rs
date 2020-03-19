//! Clock configuration

use crate::time::*;
use crate::target_device::PMC;
use crate::target_device::SUPC;
use crate::target_device::UTMI;

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
	// use the slow rc oscillator
	pub fn use_rc(mut self) -> Self {
		self.src = SlckSrc::SlowRC;
		self.freq = KiloHertz(32).into();

		self
	}
	// use the more precise crystal oscillator
	pub fn use_crystal(mut self) -> Self {
		self.src = SlckSrc::CrystalOscillator;
		self.freq = Hertz(32768);

		self
	}

	/// bypass and use own clk src -> you need to provide the frequency of that clk src!
	pub fn bypass(mut self, freq:Hertz) -> Self {
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

pub enum MainRcFreq {
	Freq4Mhz,
	Freq8Mhz,
	Freq12Mhz
}

/// Holds the current MAINCK config
pub struct MainckConfig {
	rc_on : bool,
	crystal_on : bool,
	rc_freq : MainRcFreq,
	freq : Hertz,
	src : MainckSrc,
	startup_cycles : u8
}

impl MainckConfig {
	/// use rc oscillator at 4MHz
	pub fn use_rc_4mhz(mut self) -> Self {
		self.src = MainckSrc::MainRC;
		self.rc_on = true;
		self.rc_freq = MainRcFreq::Freq4Mhz;

		self
	}

    /// use rc oscillator at 8MHz
	pub fn use_rc_8mhz(mut self) -> Self {
		self.src = MainckSrc::MainRC;
		self.rc_on = true;
		self.rc_freq = MainRcFreq::Freq8Mhz;

		self
	}

    /// use rc oscillator at 12MHz
	pub fn use_rc_12mhz(mut self) -> Self {
		self.src = MainckSrc::MainRC;
		self.rc_on = true;
		self.rc_freq = MainRcFreq::Freq12Mhz;

		self
	}

	/// disable rc oscillator
	pub fn disable_rc(mut self) -> Self {
		self.rc_on = false;

		self
	}

	/// The frequency of the crystal oscillator depends on how the devices is connected
	/// so you need to provide the frequency you configured with the hardware
	pub fn use_crystal(mut self, freq:Hertz) -> Self {
		self.src = MainckSrc::MainCrystalOscillator;
		self.freq = freq;
		self.crystal_on = true;

		self
	}

	/// disable crystal oscillator
	pub fn disable_crystal(mut self) -> Self {
		self.crystal_on = false;

		self
	}

	/// bypass and use own clk src -> you need to provide the frequency of that clk src!
	pub fn bypass(mut self, freq:Hertz) -> Self {
		self.src = MainckSrc::Bypass;
		self.freq = freq;
		self.crystal_on = false;

		self
	}


	/// procide slck cycle count to wait for crystal oscillator to stabilize
	pub fn startup_cycles(mut self, cycles:u8) -> Self {
		self.startup_cycles = cycles;

		self
	}
}

impl Default for MainckConfig {
	fn default() -> MainckConfig {
		MainckConfig {
			src : MainckSrc::MainRC,
			rc_freq : MainRcFreq::Freq12Mhz,
			rc_on : true,
			crystal_on : false,
			freq : MegaHertz(12).into(),
			startup_cycles : 100
		}
	}
}

/// Holds the current PLLA clock config
pub struct PllackConfig {
	freq : Hertz,
	diva : u8,
	mula : u16,
	strategy : ClockCalcStrategy,
	startup_cycles : u8
}

impl PllackConfig {
	/// configure PLLA based on target frequency, tries to calculate reasonable values for the divider
	pub fn from_freq(mut self, freq:Hertz) -> Self{
		self.freq = freq;
		self.diva = 0;
		self.mula = 0;
		self.strategy = ClockCalcStrategy::FromFrequency;

		self
	}

	/// configure PLLA based on divider values, will calculate the resulting frequency
	pub fn from_divider(mut self, diva:u8, mula:u16 ) -> Self {
		self.freq = Hertz(0);
		self.diva = diva;
		self.mula = mula;
		self.strategy = ClockCalcStrategy::FromDivider;

		self
	}

	pub fn startup_cycles(mut self, cycles:u8) -> Self {
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
	startup_cycles : u8,
	enable : bool
}

impl UpllckConfig {
	pub fn enable(mut self) -> Self {
		self.enable = true;

		self
	}

	pub fn disable(mut self) -> Self {
		self.enable = false;

		self
	}

	pub fn src_freq(mut self, src:UpllckSrcFreq) -> Self {
		self.src = src;

		self
	}

	pub fn startup_cycles(mut self, cycles:u8) -> Self {
		self.startup_cycles = cycles;

		self
	}
}

impl Default for UpllckConfig {
	fn default() -> UpllckConfig {
		UpllckConfig {
			src : UpllckSrcFreq::SRC12MHz,
			startup_cycles : 0,
			enable : false
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

/// Holds the configuration of the Master Clock driving the CPU and Peripherals
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
	/// select slow clock as source
	pub fn src_slck(mut self) -> Self {
		self.src = SystemClocksSrc::SLCK;

		self
	}

	/// select main clock as source
	pub fn src_mainck(mut self) -> Self {
		self.src = SystemClocksSrc::MAINCK;

		self

	}

	/// select plla clock as source
	pub fn src_pllack(mut self) -> Self {
		self.src = SystemClocksSrc::PLLACK;

		self

	}

	/// select uplldiv clock as source
	pub fn src_upllckdiv(mut self) -> Self {
		self.src = SystemClocksSrc::UPLLCKDIV;

		self

	}

	/// set divider for uplldiv clock signal
	pub fn set_uplldiv(mut self, div:UpllDiv) -> Self {
		self.uplldiv = div;

		self
	}

	/// set divider values for processor and perpheral clocks
	pub fn from_divider(mut self, pres:MasterPrescale, mdiv:MasterDivider) -> Self {
		self.strategy = ClockCalcStrategy::FromDivider;
		self.pres = pres;
		self.mdiv = mdiv;

		self
	}

	/// calculate divider values from target frequencies -> *Warning* only a best effort approximation is done
	/// since depending on the other clock signals it might not be possible to reach the exact value
	pub fn from_freq(mut self, cpu_freq:Hertz, mck_freq:Hertz) -> Self {
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

/// Holds the configuration of all main clock domains
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
		match self.slck_conf.src {
			SlckSrc::SlowRC => {
				// Dont do anything here, since this will be run at startup the SlowRC is already selected
				// It is not possible to switch back to the SlowRC!
			}
			SlckSrc::CrystalOscillator => {
				supc.supc_cr.write( |w| {
					w.key().passwd();
					w.xtalsel().set_bit()
				});
			}
			SlckSrc::Bypass => {
				supc.supc_mr.write( |w| {
					w.key().passwd();
					w.oscbypass().set_bit()
				});
				supc.supc_cr.write( |w| {
					w.key().passwd();
					w.xtalsel().set_bit()
				});
			}
		}

		// Main Clock configuration

		// if main rc should be enabled set it up
		if self.mainck_conf.rc_on {
			if pmc.ckgr_mor.read().moscrcen().bit_is_clear() {
				pmc.ckgr_mor.modify( |_, w| {
					w.key().passwd();
					w.moscrcen().set_bit()
				});
			}
			while pmc.pmc_sr.read().moscrcs().bit_is_clear(){
				//Wait until clock has stabilized
			}
			pmc.ckgr_mor.modify( |_,w| {
				w.key().passwd();
				match self.mainck_conf.rc_freq {
					MainRcFreq::Freq4Mhz => w.moscrcf()._4_mhz(),
					MainRcFreq::Freq8Mhz => w.moscrcf()._8_mhz(),
					MainRcFreq::Freq12Mhz => w.moscrcf()._12_mhz()
				}
			});
			while pmc.pmc_sr.read().moscrcs().bit_is_clear(){
				//Wait until clock has stabilized
			}
		}

		// if main crystal oscillator is enabled set it up
		if self.mainck_conf.crystal_on {
			if pmc.ckgr_mor.read().moscxten().bit_is_clear() {
				pmc.ckgr_mor.modify( |_, w| {
					w.key().passwd();
					unsafe{ w.moscxtst().bits(self.mainck_conf.startup_cycles); }
					w.moscxten().set_bit()
				});
			}
			while pmc.pmc_sr.read().moscxts().bit_is_clear(){
				//Wait until clock has stabilized
			}
		}

		// select clock
		let mainck_freq:Hertz;

		match self.mainck_conf.src {
			MainckSrc::MainRC => {
				if !self.mainck_conf.rc_on {
					panic!("Can not select Main RC Oscillator as MAINCK src if it is disabled!");
				}
				pmc.ckgr_mor.modify( |_,w| {
					w.key().passwd();
					w.moscsel().clear_bit()
				});
				while pmc.pmc_sr.read().moscsels().bit_is_clear(){
					//Wait for switch to be complete
				}
				mainck_freq = match self.mainck_conf.rc_freq {
					MainRcFreq::Freq4Mhz => MegaHertz(4).into(),
					MainRcFreq::Freq8Mhz => MegaHertz(8).into(),
					MainRcFreq::Freq12Mhz => MegaHertz(12).into()
				};
			}
			MainckSrc::MainCrystalOscillator => {
				if !self.mainck_conf.crystal_on {
					panic!("Can not select Crystal Oscillator as MAINCK src if it is disabled!");
				}
				pmc.ckgr_mor.modify( |_,w| {
					w.key().passwd();
					w.moscsel().set_bit()
				});
				while pmc.pmc_sr.read().moscsels().bit_is_clear(){
					//Wait for switch to be complete
				}
				mainck_freq = self.mainck_conf.freq;
			}
			MainckSrc::Bypass => {
				if self.mainck_conf.crystal_on {
					panic!("Can not bypass crystal oscillator when it is enabled!");
				}

				// if you use the bypass make sure your oscillator has stabilized before usage!

				pmc.ckgr_mor.modify( |_,w| {
					w.key().passwd();
					w.moscxtby().set_bit();
					w.moscxten().clear_bit()
				});
				pmc.ckgr_mor.modify( |_,w| {
					w.key().passwd();
					w.moscsel().set_bit()
				});
				while pmc.pmc_sr.read().moscsels().bit_is_clear(){
					//Wait for switch to be complete
				}
				mainck_freq = self.mainck_conf.freq;
			}
		}
		// shut down disabled oscillators
		if !self.mainck_conf.rc_on {
			pmc.ckgr_mor.modify( |_,w| {
				w.key().passwd();
				w.moscrcen().clear_bit()
			});
		}
		if !self.mainck_conf.crystal_on {
			pmc.ckgr_mor.modify( |_,w| {
				w.key().passwd();
				w.moscxten().clear_bit()
			});
		}

		// Plla configuration
		let plla_freq:Hertz;
		match self.plla_conf.strategy {
			ClockCalcStrategy::FromDivider => {
				pmc.ckgr_pllar.write( |w| {
					w.one().set_bit();
					unsafe { w.mula().bits(self.plla_conf.mula) };
					unsafe { w.diva().bits(self.plla_conf.diva) };
					unsafe { w.pllacount().bits(self.plla_conf.startup_cycles) }
				});
				while pmc.pmc_sr.read().locka().bit_is_clear() {
					//Wait until clock has stabilized
				}

				plla_freq = mainck_freq * (self.plla_conf.mula + 1) / (self.plla_conf.diva);
			}
			ClockCalcStrategy::FromFrequency => {
		    	//currently not implemented! leaves plla disabled
				plla_freq = Hertz(0);
			}
		}
		

		// Upll configuration
		let upll_freq:Hertz;
		if !self.upll_conf.enable {
			pmc.ckgr_uckr.write( |w| w.upllen().clear_bit() );
			upll_freq = Hertz(0);
		} else {
			match self.upll_conf.src {
				UpllckSrcFreq::SRC12MHz => unsafe{ (*UTMI::ptr()).utmi_cktrim.write(|w| w.freq().xtal12()); }
				UpllckSrcFreq::SRC16MHz => unsafe{ (*UTMI::ptr()).utmi_cktrim.write(|w| w.freq().xtal16()); }
			}
			pmc.ckgr_uckr.write( |w| {
				unsafe { w.upllcount().bits(self.upll_conf.startup_cycles); }
				w.upllen().set_bit()
			});
			while pmc.pmc_sr.read().locku().bit_is_clear() {
				//Wait until clock has stabilized
			}
			upll_freq = MegaHertz(480).into();
		}

		// Master Clock configuration

		Clocks {
			slck : self.slck_conf.freq,
			mainck : mainck_freq,
			plla : plla_freq,
			upll : upll_freq,

			uplldiv : Hertz(0),
			mck : Hertz(0),
			fclk : Hertz(0),
			sys_tick : Hertz(0),
			hclk : Hertz(0),

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
	upll : Hertz
}

impl Clocks {
	/// Returns Processor frequency
	pub fn hclk(&self) -> Hertz {
		self.hclk
	}

	/// Returns SysTick frequency
	pub fn sys_tick(&self) -> Hertz {
		self.sys_tick
	}

	/// Returns Free Running Processor Clock frequency
	pub fn fclk(&self) -> Hertz {
		self.fclk
	}

	/// Returns Master Clock frequency
	pub fn mck(&self) -> Hertz {
		self.mck
	}

	/// Returns Slow Clock frequency
	pub fn slck(&self) -> Hertz {
		self.slck
	}

	/// Returns Mainck frequency
	pub fn mainck(&self) -> Hertz {
		self.mainck
	}

	/// Returns Plla frequency
	pub fn plla(&self) -> Hertz {
		self.plla
	}

	/// Returns Uplldiv frequency
	pub fn uplldiv(&self) -> Hertz {
		self.uplldiv
	}

	/// Returns UPLL usb clock frequency
	pub fn upll(&self) -> Hertz {
		self.upll
	}
}
