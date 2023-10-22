use super::*;
use defmt::Format;

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub enum RssiMargin {
    Five,
    Ten,
    Fifteen,
    #[default]
    Twenty,
}

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub enum AdcClockSpeed {
    #[default]
    Mhz4,
    Mhz8,
}

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub enum AdcCaptureMode {
    Single,
    #[default]
    Continuous,
}

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub struct AdcControl {
    pub margin: RssiMargin,
    /// If `true`, RSSI measurement will end when carrier detected and ID code word received
    pub rssi_end_early: bool,
    pub adc_clock_speed: AdcClockSpeed,
    pub capture_mode: AdcCaptureMode,
}

impl Register for AdcControl {
    fn id() -> u8 {
        0x1E
    }
}

impl WritableRegister for AdcControl {}

impl From<AdcControl> for u8 {
    fn from(val: AdcControl) -> Self {
        0b0000_0010
            | match val.margin {
                RssiMargin::Five => 0b0000_0000,
                RssiMargin::Ten => 0b0100_0000,
                RssiMargin::Fifteen => 0b1000_0000,
                RssiMargin::Twenty => 0b1100_0000,
            }
            | if val.rssi_end_early { 0b0010_0000 } else { 0 }
            | match val.adc_clock_speed {
                AdcClockSpeed::Mhz4 => 0b0000_0000,
                AdcClockSpeed::Mhz8 => 0b0001_0000,
            }
            | match val.capture_mode {
                AdcCaptureMode::Single => 0b0000_0000,
                AdcCaptureMode::Continuous => 0b0000_0001,
            }
    }
}

#[cfg(test)]
mod test {
    use super::super::Register as _;
    use super::*;

    #[test]
    fn test_adc_control() {
        let default: u8 = AdcControl::default().into();
        assert_eq!(default, 0b1100_0011);

        assert_eq!(AdcControl::id(), 0x1E);
    }
}
