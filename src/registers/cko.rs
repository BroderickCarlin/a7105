use super::*;
use defmt::Format;

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub enum PinOutputMode {
    DckInTxRckInRx,
    FifoPointerFlag,
    Fsync,
    FsyncDiv2,
    FsyncDiv4,
    #[default]
    FsyncDiv8,
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub struct CkoPinControl {
    pub external_clock_output_enabled: bool,
    pub pin_output_mode: PinOutputMode,
    pub invert_output: bool,
    pub output_enabled: bool,
    pub invert_spi_clk_input: bool,
}

impl Default for CkoPinControl {
    fn default() -> Self {
        Self {
            external_clock_output_enabled: true,
            pin_output_mode: Default::default(),
            invert_output: false,
            output_enabled: true,
            invert_spi_clk_input: false,
        }
    }
}

impl Register for CkoPinControl {
    fn id() -> u8 {
        0x0A
    }
}

impl WritableRegister for CkoPinControl {}

impl From<CkoPinControl> for u8 {
    fn from(val: CkoPinControl) -> Self {
        u8::from(val.external_clock_output_enabled) << 7
            | match val.pin_output_mode {
                PinOutputMode::DckInTxRckInRx => 0b0000_0000,
                PinOutputMode::FifoPointerFlag => 0b0001_0000,
                PinOutputMode::Fsync => 0b0010_0000,
                PinOutputMode::FsyncDiv2 => 0b0010_1000,
                PinOutputMode::FsyncDiv4 => 0b0011_0000,
                PinOutputMode::FsyncDiv8 => 0b0011_1000,
            }
            | u8::from(val.invert_output) << 2
            | u8::from(val.output_enabled) << 1
            | u8::from(val.invert_spi_clk_input)
    }
}

#[cfg(test)]
mod test {
    use super::super::Register as _;
    use super::*;

    #[test]
    fn test_cko_pin_control() {
        let default: u8 = CkoPinControl::default().into();
        assert_eq!(default, 0b1011_1010);

        assert_eq!(CkoPinControl::id(), 0xA);
    }
}
