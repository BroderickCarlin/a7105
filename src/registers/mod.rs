pub use adc::*;
pub use battery::*;
pub use calibration::*;
pub use cko::*;
pub use clock::*;
pub use code::*;
pub use data_rate::*;
pub use delay::*;
pub use fifo::*;
pub use gpio::*;
pub use id::*;
pub use if_calibration::*;
pub use mode::*;
pub use pll::*;
pub use rc_osc::*;
pub use rssi::*;
pub use rx::*;
pub use tx::*;
pub use vco::*;

mod adc;
mod battery;
mod calibration;
mod cko;
mod clock;
mod code;
mod data_rate;
mod delay;
mod fifo;
mod gpio;
mod id;
mod if_calibration;
mod mode;
mod pll;
mod rc_osc;
mod rssi;
mod rx;
mod tx;
mod vco;

/// The generic top level trait for all register values
pub trait Register {
    fn id() -> u8;
}

/// A marker trait for registers that are readable
pub trait ReadableRegister<const N: usize = 1>: Register + FromSlice<N> {}

/// A marker trait for registers that are writable
pub trait WritableRegister<const N: usize = 1>: Register + IntoSlice<N> {}

/// A utility trait for representing types that can be created from a slice of bytes of a specific length
///
/// It is suggested to instead implement `From<u8>`, `From<u16>`, or `From<u32>` as this trait will be auto-derived
/// for types that do.
pub trait FromSlice<const N: usize> {
    fn from_slice(bytes: [u8; N]) -> Self;
}

impl<T> FromSlice<1> for T
where
    T: From<u8>,
{
    fn from_slice(bytes: [u8; 1]) -> Self {
        Self::from(bytes[0])
    }
}

impl<T> FromSlice<2> for T
where
    T: From<u16>,
{
    fn from_slice(bytes: [u8; 2]) -> Self {
        Self::from(u16::from_le_bytes(bytes))
    }
}

impl<T> FromSlice<4> for T
where
    T: From<u32>,
{
    fn from_slice(bytes: [u8; 4]) -> Self {
        Self::from(u32::from_le_bytes(bytes))
    }
}

/// A utility trait for representing types that can be converted into a slice of bytes of a specified length
///
/// It is suggested to instead implement `Into<u8>`, `Into<u16>`, or `Into<u32>` as this trait will be auto-derived
/// for types that do.
pub trait IntoSlice<const N: usize> {
    fn into_slice(self) -> [u8; N];
}

impl<T> IntoSlice<1> for T
where
    T: Into<u8>,
{
    fn into_slice(self) -> [u8; 1] {
        [self.into()]
    }
}

impl<T> IntoSlice<2> for T
where
    T: Into<u16>,
{
    fn into_slice(self) -> [u8; 2] {
        self.into().to_le_bytes()
    }
}

impl<T> IntoSlice<4> for T
where
    T: Into<u32>,
{
    fn into_slice(self) -> [u8; 4] {
        self.into().to_le_bytes()
    }
}
