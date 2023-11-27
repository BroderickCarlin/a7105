use defmt::Format;

use crate::registers::Mode;

/// An error that can result from the attempt to receive a packet with
/// the A7105
#[derive(Format, PartialEq, Debug, Clone)]
pub enum ReadPacketError<E> {
    /// A SPI error was encountered
    SpiError(E),
    /// An error was encountered with the recieved packet
    PacketError(PacketError),
}

/// A type that represents the errors that were encountered with a
/// received packet
#[derive(Format, PartialEq, Debug, Clone)]
pub struct PacketError {
    /// The Forward Error Correction (FEC) failed on the received packet
    pub fec_failed: bool,
    /// The CRC failed on the received packet
    pub crc_failed: bool,
}

impl<E> From<E> for ReadPacketError<E> {
    fn from(value: E) -> Self {
        Self::SpiError(value)
    }
}

impl From<Mode> for PacketError {
    fn from(mode: Mode) -> Self {
        Self {
            fec_failed: !mode.fec_pass,
            crc_failed: !mode.crc_pass,
        }
    }
}
