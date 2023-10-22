use super::*;
use defmt::Format;

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub struct BatteryDetectConfig {
    /// VDD D voltage setting in Sleep mode
    pub sleep_voltage_setting: SleepModeVoltageSetting,
    /// VDD D and VDD A voltage setting in non-Sleep mode
    pub nonsleep_voltage_setting: NonSleepModeVoltageSetting,
    /// Battery voltage detect threshold
    pub detect_threshold: DetectThreshold,
    /// Battery detect threshold enabled
    pub detect_enabled: bool,
}

impl Register for BatteryDetectConfig {
    fn id() -> u8 {
        0x27
    }
}

impl WritableRegister for BatteryDetectConfig {}

impl From<BatteryDetectConfig> for u8 {
    fn from(val: BatteryDetectConfig) -> Self {
        u8::from(val.detect_enabled)
            | match val.detect_threshold {
                DetectThreshold::V20 => 0b000,
                DetectThreshold::V21 => 0b001,
                DetectThreshold::V22 => 0b010,
                DetectThreshold::V23 => 0b011,
                DetectThreshold::V24 => 0b100,
                DetectThreshold::V25 => 0b101,
                DetectThreshold::V26 => 0b110,
                DetectThreshold::V27 => 0b111,
            } << 1
            | match val.nonsleep_voltage_setting {
                NonSleepModeVoltageSetting::V18 => 0b11,
                NonSleepModeVoltageSetting::V19 => 0b10,
                NonSleepModeVoltageSetting::V20 => 0b01,
                NonSleepModeVoltageSetting::V21 => 0b00,
            } << 5
            | match val.sleep_voltage_setting {
                SleepModeVoltageSetting::ThreeFifth => 0b0,
                SleepModeVoltageSetting::ThreeForths => 0b1,
            } << 7
    }
}

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub enum SleepModeVoltageSetting {
    /// 3/5 * REGI
    #[default]
    ThreeFifth,
    /// 3/4 * REGI
    ThreeForths,
}

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub enum NonSleepModeVoltageSetting {
    /// 1.8V
    V18,
    /// 1.9V
    V19,
    /// 2.0V
    V20,
    /// 2.1V
    #[default]
    V21,
}

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub enum DetectThreshold {
    /// 2.0V
    V20,
    /// 2.1V
    V21,
    /// 2.2V
    V22,
    /// 2.3V
    #[default]
    V23,
    /// 2.4V
    V24,
    /// 2.5V
    V25,
    /// 2.6V
    V26,
    /// 2.7V
    V27,
}

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub struct BatteryDetectResult {
    /// Battery detection flag
    pub voltage_above_threshold: bool,
}

impl Register for BatteryDetectResult {
    fn id() -> u8 {
        0x27
    }
}

impl ReadableRegister for BatteryDetectResult {}

impl From<u8> for BatteryDetectResult {
    fn from(val: u8) -> Self {
        Self {
            voltage_above_threshold: (val & 0b0001_0000) != 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::Register as _;
    use super::*;

    #[test]
    fn test_battery_detect() {
        let default: u8 = BatteryDetectConfig::default().into();
        assert_eq!(default, 0b0000_0110);

        assert_eq!(BatteryDetectConfig::id(), 0x27);
        assert_eq!(BatteryDetectResult::id(), 0x27);
    }
}
