use core::marker::PhantomData;

use crate::target_device::PMC;

pub trait GpioExt {
	///The pins the PIO is split into
	type Parts;

	///Splits the PIO block into independant pins
	fn split(self, pmc: &mut PMC) -> Self::Parts;
}
/// Input mode (type state)
pub struct Input<MODE> {
	_mode : PhantomData<MODE>,
}

// Floating input (type state)
pub struct Floating;
// Pulled down input (type state)
pub struct PullDown;
//Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
	_mode : PhantomData<MODE>,
}

// OpenDrain output (type state)
pub struct OpenDrain;
// Peripheral multiplexer device 0 (type state)
pub struct PMD0;
// Peripheral multiplexer device 1 (type state)
pub struct PMD1;
// Peripheral multiplexer device 2 (type state)
pub struct PMD2;
// Peripheral multiplexer device 3 (type state)
pub struct PMD3;

/**
Macro to generate Traits for every PIO controller on the device
 PIOX  : PIO device
 piox  :
 perid : peripheral id
 PXx   :
*/
macro_rules! gpio {
	(
		$PIOX:ident, $piox:ident, $pioy:ident, $perid:ident, $PXx:ident, [
			$($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty, $PF:ident),)+
	]) => {
		/// PIO
		pub mod $piox {
			use core::convert::Infallible;
			use core::marker::PhantomData;

			use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin, ToggleableOutputPin, InputPin};
			use embedded_hal::digital::v2::toggleable;
			use crate::target_device::{$PIOX, $pioy};

			use crate::target_device::PMC;
			#[allow(unused_imports)]
			use super::{
				PMD0, PMD1, PMD2, PMD3, OpenDrain, Floating, PullUp, PullDown, Input, Output, GpioExt
			};

			//Opaque PER register
			pub struct PER {
				_0: (),
			}

			impl PER {
				pub(crate) fn per(&mut self) -> &$pioy::PIO_PER {
					unsafe { &(*$PIOX::ptr()).pio_per }
				}
			}

			//Opaque PDR register
			pub struct PDR {
				_0: (),
			}

			impl PDR {
				pub(crate) fn pdr(&mut self) -> &$pioy::PIO_PDR {
					unsafe { &(*$PIOX::ptr()).pio_pdr }
				}
			}

			//Opaque OER register
			pub struct OER {
				_0: (),
			}

			impl OER {
				pub(crate) fn oer(&mut self) -> &$pioy::PIO_OER {
					unsafe { &(*$PIOX::ptr()).pio_oer }
				}
			}

			//Opaque ODR register
			pub struct ODR {
				_0: (),
			}

			impl ODR {
				pub(crate) fn odr(&mut self) -> &$pioy::PIO_ODR {
					unsafe { &(*$PIOX::ptr()).pio_odr }
				}
			}


			//Opaque PUER register
			pub struct PUER {
				_0: (),
			}

			impl PUER {
				pub(crate) fn puer(&mut self) -> &$pioy::PIO_PUER {
					unsafe { &(*$PIOX::ptr()).pio_puer }
				}
			}

			//Opaque PUDR register
			pub struct PUDR {
				_0: (),
			}

			impl PUDR {
				pub(crate) fn pudr(&mut self) -> &$pioy::PIO_PUDR {
					unsafe { &(*$PIOX::ptr()).pio_pudr }
				}
			}

			//Opaque PPDER register
			pub struct PPDER {
				_0: (),
			}

			impl PPDER {
				pub(crate) fn ppder(&mut self) -> &$pioy::PIO_PPDER {
					unsafe { &(*$PIOX::ptr()).pio_ppder }
				}
			}

			//Opaque PPDDR register
			pub struct PPDDR {
				_0: (),
			}

			impl PPDDR {
				pub(crate) fn ppddr(&mut self) -> &$pioy::PIO_PPDDR {
					unsafe { &(*$PIOX::ptr()).pio_ppddr }
				}
			}
			//Opaque ABCDSR register
			pub struct ABCDSR {
				_0: (),
			}

			impl ABCDSR {
				pub(crate) fn abcdsr1(&mut self) -> &$pioy::PIO_ABCDSR {
					unsafe { &(*$PIOX::ptr()).pio_abcdsr[0] }
				}
				pub(crate) fn abcdsr2(&mut self) -> &$pioy::PIO_ABCDSR {
					unsafe { &(*$PIOX::ptr()).pio_abcdsr[1] }
				}
			}

			/// PIO parts
			pub struct Parts {
				/// Opaque Pio Enable Register
				pub per: PER,
				/// Opaque Pio Disable Register
				pub pdr: PDR,
				/// Opaque Output Enable Register
				pub oer: OER,
				/// Opaque Output Disable Register
				pub odr: ODR,
				/// Opaque Pull Up Enable Register
				pub puer : PUER,
				/// Opaque Pull Up Disable Register
				pub pudr : PUDR,
				/// Opaque Pad Pull Down Enable Register
				pub ppder : PPDER,
				/// Opaque Pad Pull Down Disable Register
				pub ppddr : PPDDR,
				/// Opaque Peripheral ABCD Select Register
				pub abcdsr : ABCDSR,
				$(
					pub $pxi: $PXi<$MODE>,
				)+
			}

			impl GpioExt for $PIOX {
				type Parts = Parts;

				fn split(self, pmc: &mut PMC) -> Parts {
					pmc.pmc_pcer0.write( |w| w.$perid().set_bit());

					Parts {
						per: PER{ _0 : () },
						pdr: PDR{ _0 : () },
						oer: OER{ _0 : () },
						odr: ODR{ _0 : () },
						puer: PUER{ _0 : () },
						pudr: PUDR{ _0 : () },
						ppder: PPDER{ _0 : () },
						ppddr: PPDDR{ _0 : () },
						abcdsr: ABCDSR{ _0: ()},

						$(
							$pxi: $PXi { _mode: PhantomData },
						)+
					}
				}
			}


			$(
				pub struct $PXi<MODE> {
					_mode: PhantomData<MODE>,
				}

				impl<MODE> $PXi<MODE> {
					pub fn into_pmd0(
						self,
						abcdsr: &mut ABCDSR,
						pdr: &mut PDR,
					) -> $PXi<PMD0>{
						abcdsr.abcdsr1().write(|w| {w.$pxi().clear_bit() });
						abcdsr.abcdsr2().write(|w| {w.$pxi().clear_bit() });
						pdr.pdr().write(|w| { w.$pxi().set_bit() });

						$PXi { _mode: PhantomData }
					}

					pub fn into_pmd1(
						self,
						abcdsr: &mut ABCDSR,
						pdr: &mut PDR,
					) -> $PXi<PMD1>{
						abcdsr.abcdsr1().write(|w| {w.$pxi().set_bit() });
						abcdsr.abcdsr2().write(|w| {w.$pxi().clear_bit() });
						pdr.pdr().write(|w| { w.$pxi().set_bit() });


						$PXi { _mode: PhantomData }
					}


					pub fn into_pmd2(
						self,
						abcdsr: &mut ABCDSR,
						pdr: &mut PDR,
					) -> $PXi<PMD2>{
						abcdsr.abcdsr1().write(|w| {w.$pxi().clear_bit() });
						abcdsr.abcdsr2().write(|w| {w.$pxi().set_bit() });
						pdr.pdr().write(|w| { w.$pxi().set_bit() });

						$PXi { _mode: PhantomData }
					}


					pub fn into_pmd3(
						self,
						abcdsr: &mut ABCDSR,
						pdr: &mut PDR,
					) -> $PXi<PMD3>{
						abcdsr.abcdsr1().write(|w| {w.$pxi().set_bit() });
						abcdsr.abcdsr2().write(|w| {w.$pxi().set_bit() });
						pdr.pdr().write(|w| { w.$pxi().set_bit() });

						$PXi { _mode: PhantomData }
					}

					pub fn into_open_drain_output(
						self,
						per: &mut PER,
						oer: &mut OER,
					) -> $PXi<Output<OpenDrain>> {
						per.per().write(|w| { w.$pxi().set_bit() });
						oer.oer().write(|w| { w.$pxi().set_bit() });

						$PXi { _mode: PhantomData }
					}
				}

				impl<MODE> OutputPin for $PXi<Output<MODE>> {
					type Error = Infallible;

					fn set_high(&mut self) -> Result<(), Self::Error> {
						//NOTE (unsafe) atomic write to a stateless register
						Ok(unsafe{ (*$PIOX::ptr()).pio_sodr.write(|w| w.bits(1 << $i ))})
					}
					fn set_low(&mut self) -> Result<(), Self::Error> {
						//NOTE (unsafe) atomic write to a stateless register
						Ok(unsafe{ (*$PIOX::ptr()).pio_codr.write(|w| w.bits(1 << $i ))})
					}
				}

				impl<MODE> StatefulOutputPin for $PXi<Output<MODE>> {
					fn is_set_high(&self) -> Result<bool, Self::Error> {
						//NOTE (unsafe) atomic read to a stateless register
						Ok(unsafe{ (*$PIOX::ptr()).pio_odsr.read().$pxi().bit_is_set() } )
					}
					fn is_set_low(&self) -> Result<bool, Self::Error> {
						//NOTE (unsafe) atomic write to a stateless register
						Ok(unsafe{ (*$PIOX::ptr()).pio_odsr.read().$pxi().bit_is_clear() })
					}
				}

				impl<MODE> toggleable::Default for $PXi<Output<MODE>> {}

				impl<MODE> InputPin for $PXi<Input<MODE>> {
					type Error = Infallible;

					fn is_high(&self) -> Result<bool, Self::Error> {
						// NOTE(unsafe) atomic read with no side effects
						Ok(unsafe { (*$PIOX::ptr()).pio_pdsr.read().$pxi().bit_is_set() })
					}

					fn is_low(&self) -> Result<bool, Self::Error> {
						// NOTE(unsafe) atomic read with no side effects
						Ok(unsafe { (*$PIOX::ptr()).pio_pdsr.read().$pxi().bit_is_clear() })
					}
				}
			)+
		}
	}
}
gpio!(
	PIOA, pioa, pioa, pid10, Px, [
		P0: (p0, 0, Input<Floating>, PMD0),
		P1: (p1, 1, Input<Floating>, PMD0),
		P2: (p2, 2, Input<Floating>, PMD0),
		P3: (p3, 3, Input<Floating>, PMD0),
		P4: (p4, 4, Input<Floating>, PMD0),
		P5: (p5, 5, Input<Floating>, PMD0),
		P6: (p6, 6, Input<Floating>, PMD0),
		P7: (p7, 7, Input<Floating>, PMD0),
		P8: (p8, 8, Input<Floating>, PMD0),
		P9: (p9, 9, Input<Floating>, PMD0),
		P10: (p10, 10, Input<Floating>, PMD0),
		P11: (p11, 11, Input<Floating>, PMD0),
		P12: (p12, 12, Input<Floating>, PMD0),
		P13: (p13, 13, Input<Floating>, PMD0),
		P14: (p14, 14, Input<Floating>, PMD0),
		P15: (p15, 15, Input<Floating>, PMD0),
		P16: (p16, 16, Input<Floating>, PMD0),
		P17: (p17, 17, Input<Floating>, PMD0),
		P18: (p18, 18, Input<Floating>, PMD0),
		P19: (p19, 19, Input<Floating>, PMD0),
		P20: (p20, 20, Input<Floating>, PMD0),
		P21: (p21, 21, Input<Floating>, PMD0),
		P22: (p22, 22, Input<Floating>, PMD0),
		P23: (p23, 23, Input<Floating>, PMD0),
		P24: (p24, 24, Input<Floating>, PMD0),
		P25: (p25, 25, Input<Floating>, PMD0),
		P26: (p26, 26, Input<Floating>, PMD0),
		P27: (p27, 27, Input<Floating>, PMD0),
		P28: (p28, 28, Input<Floating>, PMD0),
		P29: (p29, 29, Input<Floating>, PMD0),
		P30: (p30, 30, Input<Floating>, PMD0),
		P31: (p31, 31, Input<Floating>, PMD0),
	]);

gpio!(
	PIOB, piob, pioa, pid11, Px, [
		P0: (p0, 0, Input<Floating>, PMD0),
		P1: (p1, 1, Input<Floating>, PMD0),
		P2: (p2, 2, Input<Floating>, PMD0),
		P3: (p3, 3, Input<Floating>, PMD0),
		P4: (p4, 4, Input<Floating>, PMD0),
		P5: (p5, 5, Input<Floating>, PMD0),
		P6: (p6, 6, Input<Floating>, PMD0),
		P7: (p7, 7, Input<Floating>, PMD0),
		P8: (p8, 8, Input<Floating>, PMD0),
		P9: (p9, 9, Input<Floating>, PMD0),
		P10: (p10, 10, Input<Floating>, PMD0),
		P11: (p11, 11, Input<Floating>, PMD0),
		P12: (p12, 12, Input<Floating>, PMD0),
		P13: (p13, 13, Input<Floating>, PMD0),
		P14: (p14, 14, Input<Floating>, PMD0),
		P15: (p15, 15, Input<Floating>, PMD0),
		P16: (p16, 16, Input<Floating>, PMD0),
		P17: (p17, 17, Input<Floating>, PMD0),
		P18: (p18, 18, Input<Floating>, PMD0),
		P19: (p19, 19, Input<Floating>, PMD0),
		P20: (p20, 20, Input<Floating>, PMD0),
		P21: (p21, 21, Input<Floating>, PMD0),
		P22: (p22, 22, Input<Floating>, PMD0),
		P23: (p23, 23, Input<Floating>, PMD0),
		P24: (p24, 24, Input<Floating>, PMD0),
		P25: (p25, 25, Input<Floating>, PMD0),
		P26: (p26, 26, Input<Floating>, PMD0),
		P27: (p27, 27, Input<Floating>, PMD0),
		P28: (p28, 28, Input<Floating>, PMD0),
		P29: (p29, 29, Input<Floating>, PMD0),
		P30: (p30, 30, Input<Floating>, PMD0),
		P31: (p31, 31, Input<Floating>, PMD0),
	]);

gpio!(
	PIOC, pioc, pioa, pid12, Px, [
		P0: (p0, 0, Input<Floating>, PMD0),
		P1: (p1, 1, Input<Floating>, PMD0),
		P2: (p2, 2, Input<Floating>, PMD0),
		P3: (p3, 3, Input<Floating>, PMD0),
		P4: (p4, 4, Input<Floating>, PMD0),
		P5: (p5, 5, Input<Floating>, PMD0),
		P6: (p6, 6, Input<Floating>, PMD0),
		P7: (p7, 7, Input<Floating>, PMD0),
		P8: (p8, 8, Input<Floating>, PMD0),
		P9: (p9, 9, Input<Floating>, PMD0),
		P10: (p10, 10, Input<Floating>, PMD0),
		P11: (p11, 11, Input<Floating>, PMD0),
		P12: (p12, 12, Input<Floating>, PMD0),
		P13: (p13, 13, Input<Floating>, PMD0),
		P14: (p14, 14, Input<Floating>, PMD0),
		P15: (p15, 15, Input<Floating>, PMD0),
		P16: (p16, 16, Input<Floating>, PMD0),
		P17: (p17, 17, Input<Floating>, PMD0),
		P18: (p18, 18, Input<Floating>, PMD0),
		P19: (p19, 19, Input<Floating>, PMD0),
		P20: (p20, 20, Input<Floating>, PMD0),
		P21: (p21, 21, Input<Floating>, PMD0),
		P22: (p22, 22, Input<Floating>, PMD0),
		P23: (p23, 23, Input<Floating>, PMD0),
		P24: (p24, 24, Input<Floating>, PMD0),
		P25: (p25, 25, Input<Floating>, PMD0),
		P26: (p26, 26, Input<Floating>, PMD0),
		P27: (p27, 27, Input<Floating>, PMD0),
		P28: (p28, 28, Input<Floating>, PMD0),
		P29: (p29, 29, Input<Floating>, PMD0),
		P30: (p30, 30, Input<Floating>, PMD0),
		P31: (p31, 31, Input<Floating>, PMD0),
	]);

gpio!(
	PIOD, piod, pioa, pid16, Px, [
		P0: (p0, 0, Input<Floating>, PMD0),
		P1: (p1, 1, Input<Floating>, PMD0),
		P2: (p2, 2, Input<Floating>, PMD0),
		P3: (p3, 3, Input<Floating>, PMD0),
		P4: (p4, 4, Input<Floating>, PMD0),
		P5: (p5, 5, Input<Floating>, PMD0),
		P6: (p6, 6, Input<Floating>, PMD0),
		P7: (p7, 7, Input<Floating>, PMD0),
		P8: (p8, 8, Input<Floating>, PMD0),
		P9: (p9, 9, Input<Floating>, PMD0),
		P10: (p10, 10, Input<Floating>, PMD0),
		P11: (p11, 11, Input<Floating>, PMD0),
		P12: (p12, 12, Input<Floating>, PMD0),
		P13: (p13, 13, Input<Floating>, PMD0),
		P14: (p14, 14, Input<Floating>, PMD0),
		P15: (p15, 15, Input<Floating>, PMD0),
		P16: (p16, 16, Input<Floating>, PMD0),
		P17: (p17, 17, Input<Floating>, PMD0),
		P18: (p18, 18, Input<Floating>, PMD0),
		P19: (p19, 19, Input<Floating>, PMD0),
		P20: (p20, 20, Input<Floating>, PMD0),
		P21: (p21, 21, Input<Floating>, PMD0),
		P22: (p22, 22, Input<Floating>, PMD0),
		P23: (p23, 23, Input<Floating>, PMD0),
		P24: (p24, 24, Input<Floating>, PMD0),
		P25: (p25, 25, Input<Floating>, PMD0),
		P26: (p26, 26, Input<Floating>, PMD0),
		P27: (p27, 27, Input<Floating>, PMD0),
		P28: (p28, 28, Input<Floating>, PMD0),
		P29: (p29, 29, Input<Floating>, PMD0),
		P30: (p30, 30, Input<Floating>, PMD0),
		P31: (p31, 31, Input<Floating>, PMD0),
	]);
