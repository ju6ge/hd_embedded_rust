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

			use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin, InputPin};
			use embedded_hal::digital::v2::toggleable;
			use crate::target_device::{$PIOX};

			use crate::target_device::PMC;
			#[allow(unused_imports)]
			use super::{
				PMD0, PMD1, PMD2, PMD3, OpenDrain, Floating, PullUp, PullDown, Input, Output, GpioExt
			};



			/// PIO parts
			pub struct Parts {
				$(
					pub $pxi: $PXi<$MODE>,
				)+
			}

			impl GpioExt for $PIOX {
				type Parts = Parts;

				fn split(self, pmc: &mut PMC) -> Parts {
					pmc.pmc_pcer0.write( |w| w.$perid().set_bit());

					Parts {
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
					) -> $PXi<PMD0>{
						unsafe {
							&(*$PIOX::ptr()).pio_abcdsr[0].write(|w| { w.$pxi().clear_bit() });
							&(*$PIOX::ptr()).pio_abcdsr[1].write(|w| { w.$pxi().clear_bit() });
							&(*$PIOX::ptr()).pio_pdr.write(|w| { w.$pxi().set_bit() });
						}

						$PXi { _mode: PhantomData }
					}

					pub fn into_pmd1(
						self,
					) -> $PXi<PMD1>{
						unsafe {
							&(*$PIOX::ptr()).pio_abcdsr[0].write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_abcdsr[1].write(|w| { w.$pxi().clear_bit() });
							&(*$PIOX::ptr()).pio_pdr.write(|w| { w.$pxi().set_bit() });
						}

						$PXi { _mode: PhantomData }
					}


					pub fn into_pmd2(
						self,
					) -> $PXi<PMD2>{
						unsafe {
							&(*$PIOX::ptr()).pio_abcdsr[0].write(|w| { w.$pxi().clear_bit() });
							&(*$PIOX::ptr()).pio_abcdsr[1].write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_pdr.write(|w| { w.$pxi().set_bit() });
						}

						$PXi { _mode: PhantomData }
					}


					pub fn into_pmd3(
						self,
					) -> $PXi<PMD3>{
						unsafe {
							&(*$PIOX::ptr()).pio_abcdsr[0].write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_abcdsr[1].write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_pdr.write(|w| { w.$pxi().set_bit() });
						}

						$PXi { _mode: PhantomData }
					}

					pub fn into_open_drain_output(
						self,
					) -> $PXi<Output<OpenDrain>> {
						unsafe {
							&(*$PIOX::ptr()).pio_per.write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_oer.write(|w| { w.$pxi().set_bit() });
						}

						$PXi { _mode: PhantomData }
					}

					pub fn into_floating_input(
						self,
					) -> $PXi<Input<Floating>> {
						unsafe {
							&(*$PIOX::ptr()).pio_per.write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_odr.write(|w| { w.$pxi().set_bit() });

							//disable possible pull up/down
							&(*$PIOX::ptr()).pio_ppddr.write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_pudr.write(|w| { w.$pxi().set_bit() });
						}

						$PXi { _mode: PhantomData }
					}

					pub fn into_pull_down_input(
						self,
					) -> $PXi<Input<PullDown>> {
						unsafe {
							&(*$PIOX::ptr()).pio_per.write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_odr.write(|w| { w.$pxi().set_bit() });

							&(*$PIOX::ptr()).pio_ppder.write(|w| { w.$pxi().set_bit() });
						}

						$PXi { _mode: PhantomData }
					}

					pub fn into_pull_up_input(
						self,
					) -> $PXi<Input<PullDown>> {
						unsafe {
							&(*$PIOX::ptr()).pio_per.write(|w| { w.$pxi().set_bit() });
							&(*$PIOX::ptr()).pio_odr.write(|w| { w.$pxi().set_bit() });

							&(*$PIOX::ptr()).pio_puer.write(|w| { w.$pxi().set_bit() });
						}

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
