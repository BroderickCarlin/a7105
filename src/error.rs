use defmt::Format;

use crate::registers::Mode;

#[derive(Format, PartialEq, Debug, Clone)]
pub enum ReadPacketError<E> {
    SpiError(E),
    PacketError(PacketError),
}

#[derive(Format, PartialEq, Debug, Clone)]
pub struct PacketError {
    /// The Forward Error Correction failed on the received packet
    pub fec_failed: bool,
    /// The CRC check failed on the received packet
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
