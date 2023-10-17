use super::*;
use defmt::Format;

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub struct CalibrationControl {
    /// VCO Current calibration enable (Auto clear when done)
    pub vco_current_calibration_enabled: bool,
    /// VCO Bank calibration enable (Auto clear when done)
    pub vco_bank_calibration_enabled: bool,
    /// IF Filter Bank calibration enable (Auto clear when done)
    pub if_filter_bank_calibration_enabled: bool,
}

impl Register for CalibrationControl {
    fn id() -> u8 {
        0x02
    }
}

impl ReadableRegister for CalibrationControl {}
impl WritableRegister for CalibrationControl {}

impl From<u8> for CalibrationControl {
    fn from(val: u8) -> Self {
        Self {
            vco_current_calibration_enabled: 0b0000_0100 & val != 0,
            vco_bank_calibration_enabled: 0b0000_0010 & val != 0,
            if_filter_bank_calibration_enabled: 0b0000_0001 & val != 0,
        }
    }
}

impl Into<u8> for CalibrationControl {
    fn into(self) -> u8 {
        u8::from(self.vco_current_calibration_enabled) << 2
            | u8::from(self.vco_bank_calibration_enabled) << 1
            | u8::from(self.if_filter_bank_calibration_enabled)
    }
}

#[cfg(test)]
mod test {
    use super::super::Register as _;
    use super::*;

    #[test]
    fn test_calibration_control() {
        let default: u8 = CalibrationControl::default().into();
        assert_eq!(default, 0);

        assert_eq!(CalibrationControl::id(), 0x2);
    }
}
