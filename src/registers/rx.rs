use super::*;
use defmt::Format;

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub enum Bandwidth {
    Khz250,
    Khz500,
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub struct Rx {
    pub freq_compensation_enable: bool,
    pub data_invert: bool,
    pub bandwidth: Bandwidth,
    pub lowside_band_select: bool,
}

impl Default for Rx {
    fn default() -> Self {
        Self {
            freq_compensation_enable: false,
            data_invert: false,
            bandwidth: Bandwidth::Khz500,
            lowside_band_select: false,
        }
    }
}

impl Register for Rx {
    fn id() -> u8 {
        0x18
    }
}

impl WritableRegister for Rx {}

impl Into<u8> for Rx {
    fn into(self) -> u8 {
        // The datasheet lists both 0b0100_0000 and 0b0110_0000 as the defaults we should use
        0b0100_0000
            | u8::from(self.freq_compensation_enable) << 4
            | u8::from(self.data_invert) << 3
            | match self.bandwidth {
                Bandwidth::Khz250 => 0b0,
                Bandwidth::Khz500 => 0b1,
            } << 1
            | u8::from(self.lowside_band_select)
    }
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub enum MixerGain {
    Db24,
    Db18,
    Db12,
    Db6,
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub enum LnaGain {
    Db24,
    Db18,
    Db12,
    Db6,
    Db0,
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub struct RxGain1 {
    pub manual_vga_calibration: bool,
    pub mixer_gain: MixerGain,
    pub lna_gain: LnaGain,
}

impl Default for RxGain1 {
    fn default() -> Self {
        Self {
            manual_vga_calibration: false,
            mixer_gain: MixerGain::Db24,
            lna_gain: LnaGain::Db24,
        }
    }
}

impl Register for RxGain1 {
    fn id() -> u8 {
        0x19
    }
}

impl WritableRegister for RxGain1 {}
impl ReadableRegister for RxGain1 {}

impl Into<u8> for RxGain1 {
    fn into(self) -> u8 {
        u8::from(self.manual_vga_calibration) << 7
            | match self.mixer_gain {
                MixerGain::Db24 => 0b00,
                MixerGain::Db18 => 0b01,
                MixerGain::Db12 => 0b10,
                MixerGain::Db6 => 0b11,
            } << 3
            | match self.lna_gain {
                LnaGain::Db24 => 0b000,
                LnaGain::Db18 => 0b001,
                LnaGain::Db12 => 0b010,
                LnaGain::Db6 => 0b011,
                LnaGain::Db0 => 0b100,
            }
    }
}

impl From<u8> for RxGain1 {
    fn from(val: u8) -> Self {
        Self {
            manual_vga_calibration: (val & 0b1000_0000) != 0,
            mixer_gain: match (val >> 3) & 0b11 {
                0b00 => MixerGain::Db24,
                0b01 => MixerGain::Db18,
                0b10 => MixerGain::Db12,
                _ => MixerGain::Db6,
            },
            lna_gain: match val & 0b111 {
                0b000 => LnaGain::Db24,
                0b001 => LnaGain::Db18,
                0b010 => LnaGain::Db12,
                0b011 => LnaGain::Db6,
                _ => LnaGain::Db0,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::Register as _;
    use super::*;

    #[test]
    fn test_rx_register() {
        let default: u8 = Rx::default().into();
        assert_eq!(default, 0b0100_0010);

        assert_eq!(Rx::id(), 0x18);
    }

    #[test]
    fn test_rx_gain1_register() {
        let default: u8 = RxGain1::default().into();
        assert_eq!(default, 0b0000_0000);

        assert_eq!(RxGain1::id(), 0x19);
    }
}
