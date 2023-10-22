use super::*;
use defmt::Format;

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub struct IfCalibration1Config {
    /// The calibration value to use. A value of `None` indicates an automatic calibration
    pub calibration_value: Option<u8>,
}

impl Register for IfCalibration1Config {
    fn id() -> u8 {
        0x22
    }
}

impl WritableRegister for IfCalibration1Config {}

impl From<IfCalibration1Config> for u8 {
    fn from(cfg: IfCalibration1Config) -> u8 {
        if let Some(val) = cfg.calibration_value {
            0b0001_0000 | (val & 0b1111)
        } else {
            0b0000_0000
        }
    }
}

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub struct IfCalibration1Result {
    pub calibration_successful: bool,
    pub calibration_value: u8,
}

impl Register for IfCalibration1Result {
    fn id() -> u8 {
        0x22
    }
}

impl ReadableRegister for IfCalibration1Result {}

impl From<u8> for IfCalibration1Result {
    fn from(val: u8) -> Self {
        Self {
            calibration_successful: (val & 0b0001_0000) != 0,
            calibration_value: val & 0b1111,
        }
    }
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub struct IfCalibration2 {
    pub deviation: u8,
}

impl Register for IfCalibration2 {
    fn id() -> u8 {
        0x23
    }
}

impl ReadableRegister for IfCalibration2 {}

impl From<u8> for IfCalibration2 {
    fn from(val: u8) -> Self {
        Self {
            deviation: val & 0b0001_1111,
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::Register as _;
    use super::*;

    #[test]
    fn test_if_calibration1_config() {
        let default: u8 = IfCalibration1Config::default().into();
        assert_eq!(default, 0b0);

        assert_eq!(IfCalibration1Config::id(), 0x22);
    }

    #[test]
    fn test_if_calibration2() {
        assert_eq!(IfCalibration2::id(), 0x23);
    }
}
