#![no_std]

use commands::{Command, Mode};
use embedded_hal_async::spi::Operation;
use registers::{ReadableRegister, WritableRegister};

pub mod commands;
pub mod registers;

pub struct A7105<SPI> {
    spi: SPI,
}

impl<SPI> A7105<SPI> {
    /// Constructs a new instance of a [`A7105`] from the provided [`SpiDevice`](embedded_hal::spi::SpiDevice)
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }

    /// Destroys this instance of the [`A7105`], returning the inner [`SpiDevice`](embedded_hal::spi::SpiDevice)
    pub fn destroy(self) -> SPI {
        self.spi
    }
}

#[cfg(feature = "blocking")]
impl<SPI: embedded_hal::spi::SpiDevice> A7105<SPI> {
    /// Reads a value from a regsiter, determined by the specified return type.
    pub fn read_reg<const N: usize, R: ReadableRegister<N>>(&mut self) -> Result<R, SPI::Error> {
        let mut buf = [0u8; N];
        self.spi
            .transaction(&mut [Operation::Write(&[R::id()]), Operation::Read(&mut buf)])?;
        Ok(R::from_slice(buf))
    }

    /// Writes a value to a regsiter, determined by the specified register type.
    pub fn write_reg<const N: usize, R: WritableRegister<N>>(
        &mut self,
        reg: R,
    ) -> Result<(), SPI::Error> {
        self.spi.transaction(&mut [
            Operation::Write(&[R::id()]),
            Operation::Write(&reg.into_slice()),
        ])
    }

    /// Sets the A7105 into the specified mode
    pub fn set_mode(&mut self, mode: Mode) -> Result<(), SPI::Error> {
        self.spi.write(&[mode.into()])
    }

    /// Issues the given command to the A7105
    pub fn command(&mut self, command: Command) -> Result<(), SPI::Error> {
        let buf: &[u8] = match command {
            Command::Reset => &[0x00, 0x00],
            Command::ResetFifoReadPointer => &[0b1111_0000],
            Command::ResetFifoWritePointer => &[0b1110_0000],
        };
        self.spi.write(buf)
    }
}

#[cfg(feature = "async")]
impl<SPI: embedded_hal_async::spi::SpiDevice> A7105<SPI> {
    /// Reads a value from a regsiter, determined by the specified return type.
    pub async fn read_reg<const N: usize, R: ReadableRegister<N>>(
        &mut self,
    ) -> Result<R, SPI::Error> {
        let mut buf = [0u8; N];
        self.spi
            .transaction(&mut [Operation::Write(&[R::id()]), Operation::Read(&mut buf)])
            .await?;
        Ok(R::from_slice(buf))
    }

    /// Writes a value to a regsiter, determined by the specified register type.
    pub async fn write_reg<const N: usize, R: WritableRegister<N>>(
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
    pub async fn set_mode(&mut self, mode: Mode) -> Result<(), SPI::Error> {
        self.spi.write(&[mode.into()]).await
    }

    /// Issues the given command to the A7105
    pub async fn command(&mut self, command: Command) -> Result<(), SPI::Error> {
        let buf: &[u8] = match command {
            Command::Reset => &[0x00, 0x00],
            Command::ResetFifoReadPointer => &[0b1111_0000],
            Command::ResetFifoWritePointer => &[0b1110_0000],
        };
        self.spi.write(buf).await
    }
}
