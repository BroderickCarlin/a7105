use super::*;
use defmt::Format;

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub enum ClockSelect {
    #[default]
    FSyncDiv8,
    FSyncDiv16,
    FSyncDiv32,
    FSyncDiv64,
}

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub struct RcOsc3 {
    /// Clock select for internal digital block Recommend [`ClockSelect::FSyncDiv8`]
    pub clock_select: ClockSelect,
    // There are other fields in here, but they are for internal use only so we don't expose them
}

impl Register for RcOsc3 {
    fn id() -> u8 {
        0x09
    }
}

impl WritableRegister for RcOsc3 {}

impl Into<u8> for RcOsc3 {
    fn into(self) -> u8 {
        match self.clock_select {
            ClockSelect::FSyncDiv8 => 0b0000_0101,
            ClockSelect::FSyncDiv16 => 0b0100_0101,
            ClockSelect::FSyncDiv32 => 0b1000_0101,
            ClockSelect::FSyncDiv64 => 0b1100_0101,
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::Register as _;
    use super::*;

    #[test]
    fn test_mode_control() {
        let default: u8 = RcOsc3::default().into();
        assert_eq!(default, 0b101);

        assert_eq!(RcOsc3::id(), 0x9);
    }
}
