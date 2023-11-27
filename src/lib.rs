#![no_std]

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
pub mod registers;

pub struct A7105<SPI> {
    spi: SPI,
}

impl<SPI> A7105<SPI> {
    const RX_BUFFER_ID: u8 = 0x05;
    const TX_BUFFER_ID: u8 = 0x05;
    const READ_FLAG: u8 = 0x40;

    /// Constructs a new instance of a [`A7105`] from the provided [`SpiDevice`](embedded_hal::spi::SpiDevice)
    pub const fn new(spi: SPI) -> Self {
        Self { spi }
    }

    /// Destroys this instance of the [`A7105`], returning the inner [`SpiDevice`](embedded_hal::spi::SpiDevice)
    pub fn destroy(self) -> SPI {
        self.spi
    }
}

impl<SPI: SpiDevice> A7105<SPI> {
    /// Reads a value from a regsiter, determined by the specified return type.
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

    /// Writes a value to a regsiter, determined by the specified register type.
    #[maybe_async::maybe_async]
    pub async fn write_reg<const N: usize, R: WritableRegister>(
        &mut self,
        reg: R,
    ) -> Result<(), SPI::Error> {
        self.spi
            .transaction(&mut [
                Operation::Write(&[R::id()]),
                Operation::Write(&reg.into_slice()),
            ])
            .await
    }

    /// Sets the A7105 into the specified mode
    #[maybe_async::maybe_async]
    pub async fn set_mode(&mut self, mode: Mode) -> Result<(), SPI::Error> {
        self.spi.write(&[mode.into()]).await
    }

    /// Issues the given command to the A7105
    #[maybe_async::maybe_async]
    pub async fn command(&mut self, command: Command) -> Result<(), SPI::Error> {
        let buf: &[u8] = match command {
            Command::Reset => &[0x00, 0x00],
            Command::ResetFifoReadPointer => &[0b1111_0000],
            Command::ResetFifoWritePointer => &[0b1110_0000],
        };
        self.spi.write(buf).await
    }

    /// Attempts to read a recieved packet from the A7105's internal RX buffer
    #[maybe_async::maybe_async]
    pub async fn rx_packet(&mut self, buf: &mut [u8]) -> Result<(), ReadPacketError<SPI::Error>> {
        // Start by verifying that the packet we received is actaully valid
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
    pub async fn tx_packet(&mut self, buf: &[u8]) -> Result<(), SPI::Error> {
        self.command(Command::ResetFifoWritePointer).await?;
        self.spi
            .transaction(&mut [
                Operation::Write(&[Self::TX_BUFFER_ID]),
                Operation::Write(buf),
            ])
            .await
    }
}
