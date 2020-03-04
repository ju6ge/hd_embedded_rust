use core::fmt;
use core::marker::PhantomData;
use core::ptr;

use embedded_hal::prelude::*;
use embedded_hal::serial;
use nb::block;

use crate::target_device::{UART0, UART1, UART2, UART3, UART4};
use crate::target_device::{USART0, USART1, USART2};
use crate::target_device::PMC;

use crate::gpio::{PeripheralCntr, PeriphA, PeriphB, PeriphC, PeriphD};
use crate::gpio::pioa::{PA4, PA5, PA6, PA9, PA10, PA21, PA23};
use crate::gpio::piob::{PB0, PB1, PB4, PB13};
use crate::gpio::piod::{PD3, PD15, PD16, PD17, PD18, PD19, PD25, PD26, PD28, PD30, PD31};

use crate::time::Hertz;

/// Serial error
#[derive(Debug)]
pub enum Error {
    /// Framing error
    Framing,
    /// Noise error
    Noise,
    /// RX buffer overrun
    Overrun,
    /// Parity check error
    Parity,
    #[doc(hidden)]
    _Extensible,
}

/// Interrupt event
pub enum Event {
    /// New data has been received
    Rxne,
    /// New data can be sent
    Txe,
    /// Idle line state detected
    Idle,
}

pub mod config {
    use crate::time::Bps;
    use crate::time::U32Ext;

    pub enum WordLength {
        DataBits5,
        DataBits6,
        DataBits7,
        DataBits8,
        DataBits9,
    }

    pub enum Parity {
        ParityNone,
        ParityEven,
        ParityOdd,
    }

    pub enum StopBits {
        #[doc = "1 stop bit"]
        STOP1,
        #[doc = "1.5 stop bits"]
        STOP1P5,
        #[doc = "2 stop bits"]
        STOP2,
    }

    pub struct Config {
        pub baudrate: Bps,
        pub wordlength: WordLength,
        pub parity: Parity,
        pub stopbits: StopBits,
    }

    impl Config {
        pub fn baudrate(mut self, baudrate: Bps) -> Self {
            self.baudrate = baudrate;
            self
        }

        pub fn parity_none(mut self) -> Self {
            self.parity = Parity::ParityNone;
            self
        }

        pub fn parity_even(mut self) -> Self {
            self.parity = Parity::ParityEven;
            self
        }

        pub fn parity_odd(mut self) -> Self {
            self.parity = Parity::ParityOdd;
            self
        }

        pub fn wordlength_5(mut self) -> Self {
            self.wordlength = WordLength::DataBits5;
            self
        }

        pub fn wordlength_6(mut self) -> Self {
            self.wordlength = WordLength::DataBits6;
            self
        }

        pub fn wordlength_7(mut self) -> Self {
            self.wordlength = WordLength::DataBits7;
            self
        }

        pub fn wordlength_8(mut self) -> Self {
            self.wordlength = WordLength::DataBits8;
            self
        }

        pub fn wordlength_9(mut self) -> Self {
            self.wordlength = WordLength::DataBits9;
            self
        }

        pub fn stopbits(mut self, stopbits: StopBits) -> Self {
            self.stopbits = stopbits;
            self
        }
    }

    #[derive(Debug)]
    pub struct InvalidConfig;

    impl Default for Config {
        fn default() -> Config {
            let baudrate = 19_200_u32.bps();
            Config {
                baudrate,
                wordlength: WordLength::DataBits8,
                parity: Parity::ParityNone,
                stopbits: StopBits::STOP1,
            }
        }
    }
}


pub trait Pins<USART> {}
pub trait PinTx<USART> {}
pub trait PinRx<USART> {}
pub trait PinCk<USART> {}

impl<USART, TX, RX> Pins<USART> for (TX, RX)
where
    TX: PinTx<USART>,
    RX: PinRx<USART>,
{
}

/// A filler type for when the Tx pin is unnecessary
pub struct NoTx;
/// A filler type for when the Rx pin is unnecessary
pub struct NoRx;
/// A filler type for when the Ck pin is unnecessary
pub struct NoCk;

macro_rules! usart_pins {
    ($($USARTX:ty: TX: [$($TX:ty),*] RX: [$($RX:ty),*] CK: [$($CK:ty),*])+) => {
        $(
            $(
                impl PinTx<$USARTX> for $TX {}
            )*
            $(
                impl PinRx<$USARTX> for $RX {}
            )*
            $(
                impl PinCk<$USARTX> for $CK {}
            )*
        )+
    }
}

macro_rules! uart_pins {
    ($($UARTX:ty: TX: [$($TX:ty),*] RX: [$($RX:ty),*])+) => {
        $(
            $(
                impl PinTx<$UARTX> for $TX {}
            )*
            $(
                impl PinRx<$UARTX> for $RX {}
            )*
        )+
    }
}

usart_pins! {
	USART0:
		TX : [
			PB1<PeripheralCntr<PeriphC>>,
			NoTx
		]
		RX : [
			PB0<PeripheralCntr<PeriphC>>,
			NoRx
		]
		CK : [
			PB13<PeripheralCntr<PeriphC>>,
			NoCk
		]
	USART1:
		TX : [
			PB4<PeripheralCntr<PeriphD>>,
			NoTx
		]
		RX : [
			PA21<PeripheralCntr<PeriphA>>,
			NoRx
		]
		CK : [
			PA23<PeripheralCntr<PeriphA>>,
			NoCk
		]
	USART2:
		TX : [
			PD16<PeripheralCntr<PeriphB>>,
			NoTx
		]
		RX : [
			PD15<PeripheralCntr<PeriphB>>,
			NoRx
		]
		CK : [
			PD17<PeripheralCntr<PeriphB>>,
			NoCk
		]
}

uart_pins! {
	UART0:
		TX : [
			PA10<PeripheralCntr<PeriphA>>,
			NoTx
		]
		RX: [
			PA9<PeripheralCntr<PeriphA>>,
			NoRx
		]
	UART1:
		TX : [
			PA4<PeripheralCntr<PeriphC>>,
			PA6<PeripheralCntr<PeriphC>>,
			PD26<PeripheralCntr<PeriphD>>,
			NoTx
		]
		RX: [
			PA5<PeripheralCntr<PeriphC>>,
			NoRx
		]
	UART2:
		TX : [
			PD26<PeripheralCntr<PeriphC>>,
			NoTx
		]
		RX: [
			PD25<PeripheralCntr<PeriphC>>,
			NoRx
		]
	UART3:
		TX : [
			PD30<PeripheralCntr<PeriphA>>,
			PD31<PeripheralCntr<PeriphB>>,
			NoTx
		]
		RX: [
			PD28<PeripheralCntr<PeriphA>>,
			NoRx
		]
	UART4:
		TX : [
			PD3<PeripheralCntr<PeriphC>>,
			PD19<PeripheralCntr<PeriphC>>,
			NoTx
		]
		RX: [
			PD18<PeripheralCntr<PeriphC>>,
			NoRx
		]
}

/// Serial abstraction
pub struct Serial<USART, PINS> {
	usart: USART,
	pins: PINS,
}

/// Serial receiver
pub struct Rx<USART> {
	_usart: PhantomData<USART>,
}

/// Serial transmitter
pub struct Tx<USART> {
	_usart: PhantomData<USART>,
}

pub trait SerialExt<USART> {
	fn usart<PINS>(
		self,
		pins: PINS,
		config: config::Config,
		pmc: &mut PMC,
	) -> Result<Serial<USART, PINS>, config::InvalidConfig>
	where
		PINS: Pins<USART>;
}

macro_rules! usart {
	($( $USARTX:ident: (
			$usartX:ident,
			$en_reg:ident,
			$perid:ident,
			$usartXrst:ident,
			$pclkX:ident
		),
	)+) => {
		$(
			/// Configures a USART peripheral to provide serial
			/// communication
			impl<PINS> Serial<$USARTX, PINS> {
				pub fn $usartX(
					usart: $USARTX,
					pins: PINS,
					config: config::Config,
					pmc: &mut PMC,
				) -> Result<Self, config::InvalidConfig>
				where
					PINS: Pins<$USARTX>,
				{
				}
			}
		)+
	}
}
