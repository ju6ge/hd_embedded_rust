#![no_std]

extern crate embedded_hal as hal;
extern crate nb;

pub use atsame70q21  as target_device;

pub mod serial;
pub mod gpio;
