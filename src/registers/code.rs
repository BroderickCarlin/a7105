use super::*;
use defmt::Format;

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub enum IdLength {
    Two,
    #[default]
    Four,
}

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub enum PreambleLength {
    One,
    Two,
    Three,
    #[default]
    Four,
}

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub struct Code1 {
    pub data_whitening_enabled: bool,
    pub fec_enabled: bool,
    pub crc_enabled: bool,
    pub id_length: IdLength,
    pub preable_length: PreambleLength,
}

impl Register for Code1 {
    fn id() -> u8 {
        0x1F
    }
}

impl WritableRegister for Code1 {}

impl Into<u8> for Code1 {
    fn into(self) -> u8 {
        0 | if self.data_whitening_enabled {
            0b0010_0000
        } else {
            0
        } | if self.fec_enabled { 0b0001_0000 } else { 0 }
            | if self.crc_enabled { 0b0000_1000 } else { 0 }
            | if self.id_length == IdLength::Four {
                0b0000_0100
            } else {
                0
            }
            | match self.preable_length {
                PreambleLength::One => 0,
                PreambleLength::Two => 0b01,
                PreambleLength::Three => 0b10,
                PreambleLength::Four => 0b11,
            }
    }
}

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub enum IdErrorCodeTolerance {
    Bits0,
    #[default]
    Bits1,
    Bits2,
    Bits3,
}

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub enum PreabmelPatternDetectionLength {
    Bits0,
    Bits4,
    Bits8,
    #[default]
    Bits16,
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub struct Code2 {
    pub demodulator_dc_estimation_average_mode: u8,
    pub id_error_code_tolerance: IdErrorCodeTolerance,
    pub preamble_pattern_detection_length: PreabmelPatternDetectionLength,
}

impl Default for Code2 {
    fn default() -> Self {
        Self {
            demodulator_dc_estimation_average_mode: 0b001,
            id_error_code_tolerance: Default::default(),
            preamble_pattern_detection_length: Default::default(),
        }
    }
}

impl Register for Code2 {
    fn id() -> u8 {
        0x20
    }
}

impl WritableRegister for Code2 {}

impl Into<u8> for Code2 {
    fn into(self) -> u8 {
        (self.demodulator_dc_estimation_average_mode & 0b111) << 4
            | match self.id_error_code_tolerance {
                IdErrorCodeTolerance::Bits0 => 0,
                IdErrorCodeTolerance::Bits1 => 0b0100,
                IdErrorCodeTolerance::Bits2 => 0b1000,
                IdErrorCodeTolerance::Bits3 => 0b1100,
            }
            | match self.preamble_pattern_detection_length {
                PreabmelPatternDetectionLength::Bits0 => 0,
                PreabmelPatternDetectionLength::Bits4 => 0b01,
                PreabmelPatternDetectionLength::Bits8 => 0b10,
                PreabmelPatternDetectionLength::Bits16 => 0b11,
            }
    }
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub struct Code3 {
    pub encryption_key: u8,
}

impl Default for Code3 {
    fn default() -> Self {
        Self {
            encryption_key: 0b0010_1010,
        }
    }
}

impl WritableRegister for Code3 {}

impl Register for Code3 {
    fn id() -> u8 {
        0x21
    }
}

impl Into<u8> for Code3 {
    fn into(self) -> u8 {
        self.encryption_key & 0b0111_1111
    }
}

#[cfg(test)]
mod test {
    use super::super::Register as _;
    use super::*;

    #[test]
    fn test_code1() {
        let default: u8 = Code1::default().into();
        assert_eq!(default, 0b111);

        assert_eq!(Code1::id(), 0x1F);
    }

    #[test]
    fn test_code2() {
        let default: u8 = Code2::default().into();
        assert_eq!(default, 0b0001_0111);

        assert_eq!(Code2::id(), 0x20);
    }

    #[test]
    fn test_code3() {
        let default: u8 = Code3::default().into();
        assert_eq!(default, 0b0010_1010);

        assert_eq!(Code3::id(), 0x21);
    }
}
