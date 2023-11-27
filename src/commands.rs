use defmt::Format;

/// A command to change influence the internal state of the A7105
#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub enum Command {
    /// Instructs the A7105 to perform a software reset, resulting in the A7105 returning
    /// to its power-on-reset default state.
    Reset,
    /// Resets the write pointer for the TX FIFO to the beginning of the FIFO
    ResetFifoWritePointer,
    /// Resets the read pointer for the RX FIFO to the beginning of the FIFO
    ResetFifoReadPointer,
}

/// An enum representing the various modes that a A7105 can be in.
///
/// Refer to the various variants for further documentation on each mode and the
/// sub-systems of the A7105 that are powered on in that mode.
#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub enum Mode {
    /// A low power mode that results in the A7105 powering off all internal circuity
    /// accept for that required to respond to SPI commands.
    ///
    /// In this mode the internal FIFO buffers can not be accessed
    ///
    /// ### Powered Sub-systems
    ///
    /// | On Chip Regulator | Crystal Osc | VCO | PLL | RX Circuits | TX Circuits |
    /// |-------------------|-------------|-----|-----|-------------|-------------|
    /// | Off               | Off         | Off | Off | Off         | Off         |
    Sleep,
    /// A low power mode that is not as aggressive as [`Sleep`](Mode::Sleep).
    ///
    /// In this mode the internal FIFO buffers can be accessed like normal, however
    /// the RX/TX circuity is powered down.
    ///
    /// ### Powered Sub-systems
    ///
    /// | On Chip Regulator | Crystal Osc | VCO | PLL | RX Circuits | TX Circuits |
    /// |-------------------|-------------|-----|-----|-------------|-------------|
    /// | On                | Off         | Off | Off | Off         | Off         |
    Idle,
    /// The default mode that the A7105 is in upon either a power-on-reset or a
    /// software reset
    ///
    /// ### Powered Sub-systems
    ///
    /// | On Chip Regulator | Crystal Osc | VCO | PLL | RX Circuits | TX Circuits |
    /// |-------------------|-------------|-----|-----|-------------|-------------|
    /// | On                | On          | Off | Off | Off         | Off         |
    Standby,
    /// A Mode that will power up both the internal VCO and PLL circuits but keep the
    /// RX/TX circuits powered down.
    ///
    /// PLL mode must be active to perform VCO and IF filter calibration.
    ///
    /// The specific behavior of the PLL system is controlled through the following registers:
    /// - [`Pll1`](crate::registers::Pll1)
    /// - [`Pll2`](crate::registers::Pll2)
    /// - [`Pll3`](crate::registers::Pll3)
    /// - [`Pll4`](crate::registers::Pll4)
    /// - [`Pll5`](crate::registers::Pll5)
    ///
    /// ### Powered Sub-systems
    ///
    /// | On Chip Regulator | Crystal Osc | VCO | PLL | RX Circuits | TX Circuits |
    /// |-------------------|-------------|-----|-----|-------------|-------------|
    /// | On                | On          | On  | On  | Off         | Off         |
    Pll,
    /// The A7105 enters a mode and begins to attempt to receive packets over the air
    ///
    /// If a FIFO is being used for RX, once a complete packet has been received the A7105
    /// will automatically fall back to the previous mode it was in.
    ///
    /// If direct mode is being used, the A7105 will stay in RX mode even after a
    /// complete packet has been received
    ///
    /// ### Powered Sub-systems
    ///
    /// | On Chip Regulator | Crystal Osc | VCO | PLL | RX Circuits | TX Circuits |
    /// |-------------------|-------------|-----|-----|-------------|-------------|
    /// | On                | On          | On  | On  | On          | Off         |
    Rx,
    /// The A7105 enters a mode and prepares to transmit bytes
    ///
    /// If a FIFO is being used for TX, once a complete packet has been transmitted
    /// the A7105 will automatically fall back to the previous mode it was in.
    ///
    /// If direct mode is being used, the A7105 will stay in TX mode even after a
    /// complete packet has been transmitted
    ///
    /// ### Powered Sub-systems
    ///
    /// | On Chip Regulator | Crystal Osc | VCO | PLL | RX Circuits | TX Circuits |
    /// |-------------------|-------------|-----|-----|-------------|-------------|
    /// | On                | On          | On  | On  | Off         | On          |
    Tx,
}

impl From<Mode> for u8 {
    fn from(val: Mode) -> Self {
        match val {
            Mode::Sleep => 0b1000_0000,
            Mode::Idle => 0b1001_0000,
            Mode::Standby => 0b1010_0000,
            Mode::Pll => 0b1011_0000,
            Mode::Rx => 0b1100_0000,
            Mode::Tx => 0b1101_0000,
        }
    }
}
