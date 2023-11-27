#![no_std]

//! `a7105` is a Rust crate that provides a high-level interface for interacting
//! with the A7105 2.4GHz FSK/GFSK Transceiver, built on top of
//! [`embedded-hal`](https://crates.io/crates/embedded-hal) traits. This crate supports
//! both synchronous (sync) and asynchronous (async) APIs, allowing users to choose the
//! mode that best fits their application requirements.
//!
//! This crate makes no assumptions for the protocol, if any, used on top of the a7105.
//! Instead, the responsibility of this crate end at configuring the radio and
//! reading/writing raw bytes over the air.
//!
//! Sync and Async support is through [`embedded-hal`](https://crates.io/crates/embedded-hal)
//! and [`embedded-hal-async`](https://crates.io/crates/embedded-hal-async), configurable
//! through the `async` and `blocking` features. By default the crate will use `async`
//! variants. If blocking APIs are preferred, the `blocking` feature can be specified and
//! default features disabled.

#[cfg(all(feature = "async", feature = "blocking"))]
compile_error!("Both the 'blocking' and 'async' features are enabled. Did you forget to disable default features?");

use commands::{Command, Mode};
pub use error::*;
use registers::{ReadableRegister, WritableRegister};

#[cfg(feature = "blocking")]
use embedded_hal::spi::{Operation, SpiDevice};
#[cfg(feature = "async")]
use embedded_hal_async::spi::{Operation, SpiDevice};

pub mod commands;
mod error;
pub mod prelude;
pub mod registers;

/// The `A7105` is the primary type for interfacing with the
/// radio hardware.
pub struct A7105<SPI> {
    spi: SPI,
}

impl<SPI> A7105<SPI> {
    const RX_BUFFER_ID: u8 = 0x05;
    const TX_BUFFER_ID: u8 = 0x05;
    const READ_FLAG: u8 = 0x40;

    /// Constructs a new instance of a [`A7105`] from the provided [`SpiDevice`]
    ///
    /// This method does not make any calls to the radio hardware and does nothing
    /// to configure the [`SpiDevice`]. This method assumes that the provided [`SpiDevice`]
    /// peripheral has been previously configured, and that all radio configuration
    /// will be explicitly handled through the returned [`A7105`] instance.
    pub const fn new(spi: SPI) -> Self {
        Self { spi }
    }

    /// Destroys this instance of the [`A7105`], returning the inner [`SpiDevice`]
    ///
    /// This method does not make any calls to the radio hardware. Any house keeping
    /// to shut down the radio must be explicitly done prior to calling `destroy()`
    pub fn destroy(self) -> SPI {
        self.spi
    }
}

impl<SPI: SpiDevice> A7105<SPI> {
    /// Reads a value from a register on the A7105, inferred by the specified return type.
    ///
    /// ```ignore
    /// use a7105::prelude::*;
    ///
    /// # let a7105_spi_peripheral = unimplemented!();
    /// let mut radio = A7105::new(a7105_spi_peripheral);
    ///
    /// // Read the DataRate register
    /// let data_rate: registers::DataRate = radio.read_reg().await.unwrap();
    ///
    /// // Read the IdData register
    /// let id_data: registers::IdData = radio.read_reg().await.unwrap();
    /// ````
    #[maybe_async::maybe_async]
    pub async fn read_reg<const N: usize, R: ReadableRegister<N>>(
        &mut self,
    ) -> Result<R, SPI::Error> {
        let mut buf = [0u8; N];
        self.spi
            .transaction(&mut [
                Operation::Write(&[R::id() | Self::READ_FLAG]),
                Operation::Read(&mut buf),
            ])
            .await?;
        Ok(R::from_slice(buf))
    }

    /// Writes a value to a register on the A7105, inferred by the specified type.
    ///
    /// All writable register types implement the `Default` trait, with the default
    /// values being the manufacturer recommended default values.
    ///
    /// ```ignore
    /// use a7105::prelude::*;
    ///
    /// # let a7105_spi_peripheral = unimplemented!();
    /// let mut radio = A7105::new(a7105_spi_peripheral);
    ///
    /// let id_data = registers::IdData {
    ///     id: 42
    /// };
    ///
    /// // Write the IdData register. The register to write to is inferred by the
    /// // provided type.
    /// radio.write_reg(id_data).await.unwrap();
    /// ````
    #[maybe_async::maybe_async]
    pub async fn write_reg<R: WritableRegister>(&mut self, reg: R) -> Result<(), SPI::Error> {
        self.spi
            .transaction(&mut [
                Operation::Write(&[R::id()]),
                Operation::Write(&reg.into_slice()),
            ])
            .await
    }

    /// Sets the A7105 into the specified [`Mode`]
    ///
    /// This method is used to change the operating mode of the A7105 chip. For
    /// information on the modes supported by the A7105 refer to the [`Mode`]
    /// documentation
    #[maybe_async::maybe_async]
    pub async fn set_mode(&mut self, mode: Mode) -> Result<(), SPI::Error> {
        self.spi.write(&[mode.into()]).await
    }

    /// Issues the given command to the A7105
    ///
    /// This method is used to change some aspect of the internal state of the A7105
    /// chip. For information on the commands supported by the A7105 refer to the
    /// [`Command`] documentation
    #[maybe_async::maybe_async]
    pub async fn command(&mut self, command: Command) -> Result<(), SPI::Error> {
        let buf: &[u8] = match command {
            Command::Reset => &[0x00, 0x00],
            Command::ResetFifoReadPointer => &[0b1111_0000],
            Command::ResetFifoWritePointer => &[0b1110_0000],
        };
        self.spi.write(buf).await
    }

    /// Attempts to read a received packet from the A7105's internal RX buffer, writing
    /// the results into the provided buffer
    ///
    /// This method will only attempt to read enough bytes to fill the provided buffer.
    /// As such, care should be taken to provide a buffer large enough to fit the largest
    /// possible packet that is expected to be received.
    #[maybe_async::maybe_async]
    pub async fn rx(&mut self, buf: &mut [u8]) -> Result<(), ReadPacketError<SPI::Error>> {
        // Start by verifying that the packet we received is actually valid
        let mode: registers::Mode = self.read_reg().await?;
        if !mode.crc_pass || !mode.fec_pass {
            // The packet we got was invalid, so there isn't anything to read
            return Err(ReadPacketError::PacketError(mode.into()));
        }

        // The packet was valid so reset the read pointer and do the actual read
        self.command(Command::ResetFifoReadPointer).await?;
        self.spi
            .transaction(&mut [
                Operation::Write(&[Self::RX_BUFFER_ID | Self::READ_FLAG]),
                Operation::Read(buf),
            ])
            .await?;
        Ok(())
    }

    /// Attempts to write a packet to the A7105's internal TX buffer
    #[maybe_async::maybe_async]
    pub async fn tx(&mut self, buf: &[u8]) -> Result<(), SPI::Error> {
        self.command(Command::ResetFifoWritePointer).await?;
        self.spi
            .transaction(&mut [
                Operation::Write(&[Self::TX_BUFFER_ID]),
                Operation::Write(buf),
            ])
            .await
    }
}
