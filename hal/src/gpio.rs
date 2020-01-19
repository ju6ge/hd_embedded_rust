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

// Controlled output (type state)
pub struct Controlled;
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
			use core::marker::PhantomData;

			use embedded_hal::digital::v2::OutputPin;
			use crate::target_device::pioa::{PIO_PER, PIO_PDR, PIO_OER, PIO_ODR, PIO_PUER, PIO_PUDR, PIO_PPDDR, PIO_PPDER};
			use crate::target_device::{$PIOX};

			use crate::target_device::PMC;
			use super::{
				PC0, PC1, PC2, PC3, Controlled, Floating, PullUp, PullDown, Input, Output, GpioExt
			};

			/// PIO parts
			pub struct Parts {
				/// Opaque Pio Enable Register
				pub per: PIO_PER,
				/// Opaque Pio Disable Register
				pub pdr: PIO_PDR,
				/// Opaque Output Enable Register
				pub oer: PIO_OER,
				/// Opaque Output Disable Register
				pub odr: PIO_ODR,
				/// Opaque Pull Up Enable Register
				pub puer : PIO_PUER,
				/// Opaque Pull Up Disable Register
				pub pudr : PIO_PUDR,
				/// Opaque Pad Pull Down Enable Register
				pub ppder : PIO_PPDER,
				/// Opaque Pad Pull Down Disable Register
				pub ppddr : PIO_PPDDR,
				$(
					pub $pxi: $PXi<$MODE>,
				)+
			}

			impl GpioExt for $PIOX {
				type Parts = Parts;

				fn split(self, pmc: &mut PMC) -> Parts {
					pmc.pmc_pcer0.write( |w| w.$perid().set_bit());

					Parts {
						per: PIO_PER,
						pdr: PIO_PDR,
						oer: PIO_OER,
						odr: PIO_ODR,
						puer: PIO_PUER,
						pudr: PIO_PUDR,
						ppder: PIO_PPDER,
						ppddr: PIO_PPDDR,

						$(
							$pxi: $PXi { _mode: PhantomData },
						)+
					}
				}
			}


			//Todo
			$(
				pub struct $PXi<MODE> {
					_mode: PhantomData<MODE>,
				}
			)+
		}
	}
}

gpio!(
	PIOC, pioc, pioa, pid12, PCx, [
		PC0: (pc0, 0, Input<Floating>, PMD0),
		PC1: (pc1, 1, Input<Floating>, PMD0),
		PC2: (pc2, 2, Input<Floating>, PMD0),
		PC3: (pc3, 3, Input<Floating>, PMD0),
		PC4: (pc4, 4, Input<Floating>, PMD0),
		PC5: (pc5, 5, Input<Floating>, PMD0),
		PC6: (pc6, 6, Input<Floating>, PMD0),
		PC7: (pc7, 7, Input<Floating>, PMD0),
		PC8: (pc8, 8, Input<Floating>, PMD0),
		PC9: (pc9, 9, Input<Floating>, PMD0),
		PC10: (pc10, 10, Input<Floating>, PMD0),
		PC11: (pc11, 11, Input<Floating>, PMD0),
		PC12: (pc12, 12, Input<Floating>, PMD0),
		PC13: (pc13, 13, Input<Floating>, PMD0),
		PC14: (pc14, 14, Input<Floating>, PMD0),
		PC15: (pc15, 15, Input<Floating>, PMD0),
		PC16: (pc16, 16, Input<Floating>, PMD0),
		PC17: (pc17, 17, Input<Floating>, PMD0),
		PC18: (pc18, 18, Input<Floating>, PMD0),
		PC19: (pc19, 19, Input<Floating>, PMD0),
		PC20: (pc20, 20, Input<Floating>, PMD0),
		PC21: (pc21, 21, Input<Floating>, PMD0),
		PC22: (pc22, 22, Input<Floating>, PMD0),
		PC23: (pc23, 23, Input<Floating>, PMD0),
		PC24: (pc24, 24, Input<Floating>, PMD0),
		PC25: (pc25, 25, Input<Floating>, PMD0),
		PC26: (pc26, 26, Input<Floating>, PMD0),
		PC27: (pc27, 27, Input<Floating>, PMD0),
		PC28: (pc28, 28, Input<Floating>, PMD0),
		PC29: (pc29, 29, Input<Floating>, PMD0),
		PC30: (pc30, 30, Input<Floating>, PMD0),
		PC31: (pc31, 31, Input<Floating>, PMD0),
	]);
