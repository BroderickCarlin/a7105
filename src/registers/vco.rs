use super::*;
use defmt::Format;

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub enum VcoCurrentCalibration {
    Automatic,
    Manual(u8),
}

impl Default for VcoCurrentCalibration {
    fn default() -> Self {
        Self::Manual(0b011)
    }
}

impl Register for VcoCurrentCalibration {
    fn id() -> u8 {
        0x24
    }
}

impl WritableRegister for VcoCurrentCalibration {}

impl Into<u8> for VcoCurrentCalibration {
    fn into(self) -> u8 {
        match self {
            Self::Automatic => 0,
            Self::Manual(val) => (val & 0b1111) | 0b1_0000,
        }
    }
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub struct VcoCurrentCalibrationResult {
    pub success: bool,
    pub value: u8,
}

impl Register for VcoCurrentCalibrationResult {
    fn id() -> u8 {
        0x24
    }
}

impl ReadableRegister for VcoCurrentCalibrationResult {}

impl From<u8> for VcoCurrentCalibrationResult {
    fn from(val: u8) -> Self {
        Self {
            success: (val & 0b1_0000) != 0,
            value: val & 0b1111,
        }
    }
}

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub enum VcoSingleBandCalibration1 {
    #[default]
    Automatic,
    Manual(u8),
}

impl Register for VcoSingleBandCalibration1 {
    fn id() -> u8 {
        0x25
    }
}

impl WritableRegister for VcoSingleBandCalibration1 {}

impl Into<u8> for VcoSingleBandCalibration1 {
    fn into(self) -> u8 {
        match self {
            Self::Automatic => 0,
            Self::Manual(val) => (val & 0b111) | 0b1000,
        }
    }
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub enum VcoVoltageOutput {
    /// VT<VTL<VTH
    VtMin,
    /// VTL<VT<VTH
    VtMid,
    /// VTL<VTH<VT
    VtMax,
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub struct VcoSingleBandCalibration1Result {
    pub voltage_output: VcoVoltageOutput,
    pub success: bool,
    pub value: u8,
}

impl Register for VcoSingleBandCalibration1Result {
    fn id() -> u8 {
        0x25
    }
}

impl ReadableRegister for VcoSingleBandCalibration1Result {}

impl From<u8> for VcoSingleBandCalibration1Result {
    fn from(val: u8) -> Self {
        Self {
            voltage_output: match (val & 0b11_0000) >> 4 {
                0b00 => VcoVoltageOutput::VtMin,
                0b01 => VcoVoltageOutput::VtMid,
                _ => VcoVoltageOutput::VtMax,
            },
            success: (val & 0b1000) != 0,
            value: val & 0b111,
        }
    }
}

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub struct VcoSingleBandCalibration2 {
    /// VCO tuning voltage upper threshold level setting.
    pub voltage_upper_threshold: TuningVoltageUpperThreshold,
    /// VCO tuning voltage lower threshold level setting.
    pub voltage_lower_threshold: TuningVoltageLowerThreshold,
}

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub enum TuningVoltageUpperThreshold {
    /// 0.6V
    V06,
    /// 0.7V
    V07,
    /// 0.8V
    V08,
    /// 0.9V
    V09,
    /// 1.0V
    V10,
    /// 1.1V
    V11,
    /// 1.2V
    V12,
    /// 1.3V
    #[default]
    V13,
}

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub enum TuningVoltageLowerThreshold {
    /// 0.1V
    V01,
    /// 0.2V
    V02,
    /// 0.3V
    V03,
    /// 0.4V
    #[default]
    V04,
    /// 0.5V
    V05,
    /// 0.6V
    V06,
    /// 0.7V
    V07,
    /// 0.8V
    V08,
}

impl Register for VcoSingleBandCalibration2 {
    fn id() -> u8 {
        0x26
    }
}

impl WritableRegister for VcoSingleBandCalibration2 {}

impl Into<u8> for VcoSingleBandCalibration2 {
    fn into(self) -> u8 {
        (match self.voltage_upper_threshold {
            TuningVoltageUpperThreshold::V06 => 0b000,
            TuningVoltageUpperThreshold::V07 => 0b001,
            TuningVoltageUpperThreshold::V08 => 0b010,
            TuningVoltageUpperThreshold::V09 => 0b011,
            TuningVoltageUpperThreshold::V10 => 0b100,
            TuningVoltageUpperThreshold::V11 => 0b101,
            TuningVoltageUpperThreshold::V12 => 0b110,
            TuningVoltageUpperThreshold::V13 => 0b111,
        } << 3)
            | match self.voltage_lower_threshold {
                TuningVoltageLowerThreshold::V01 => 0b000,
                TuningVoltageLowerThreshold::V02 => 0b001,
                TuningVoltageLowerThreshold::V03 => 0b010,
                TuningVoltageLowerThreshold::V04 => 0b011,
                TuningVoltageLowerThreshold::V05 => 0b100,
                TuningVoltageLowerThreshold::V06 => 0b101,
                TuningVoltageLowerThreshold::V07 => 0b110,
                TuningVoltageLowerThreshold::V08 => 0b111,
            }
    }
}

#[cfg(test)]
mod test {
    use super::super::Register as _;
    use super::*;

    #[test]
    fn test_vco_current() {
        let default: u8 = VcoCurrentCalibration::default().into();
        assert_eq!(default, 0b0001_0011);

        assert_eq!(VcoCurrentCalibration::id(), 0x24);
    }

    #[test]
    fn test_vco_single_band1() {
        let default: u8 = VcoSingleBandCalibration1::default().into();
        assert_eq!(default, 0);

        assert_eq!(VcoSingleBandCalibration1::id(), 0x25);
        assert_eq!(VcoSingleBandCalibration1Result::id(), 0x25);
    }

    #[test]
    fn test_vco_single_band2() {
        let default: u8 = VcoSingleBandCalibration2::default().into();
        assert_eq!(default, 0b0011_1011);

        assert_eq!(VcoSingleBandCalibration2::id(), 0x26);
    }
}
