use super::*;
use defmt::Format;

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub enum WpllToTx {
    Us20,
    Us40,
    Us60,
    Us80,
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub enum PllToWpll {
    Us50,
    Us70,
    Us90,
    Us110,
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub struct Delay1 {
    pub wpl_to_tx: WpllToTx,
    pub pll_to_wpll: PllToWpll,
}

impl Default for Delay1 {
    fn default() -> Self {
        Self {
            wpl_to_tx: WpllToTx::Us60,
            pll_to_wpll: PllToWpll::Us70,
        }
    }
}

impl Register for Delay1 {
    fn id() -> u8 {
        0x16
    }
}

impl WritableRegister for Delay1 {}

impl Into<u8> for Delay1 {
    fn into(self) -> u8 {
        (match self.wpl_to_tx {
            WpllToTx::Us20 => 0b00,
            WpllToTx::Us40 => 0b01,
            WpllToTx::Us60 => 0b10,
            WpllToTx::Us80 => 0b11,
        } << 3)
            | match self.pll_to_wpll {
                PllToWpll::Us50 => 0b001,
                PllToWpll::Us70 => 0b010,
                PllToWpll::Us90 => 0b011,
                PllToWpll::Us110 => 0b100,
            }
    }
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub enum XtalSettlingDelay {
    Us200,
    Us400,
    Us600,
    Us800,
    Us1000,
    Us1500,
    Us2000,
    Us2500,
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub enum AgcDelaySettling {
    Us10,
    Us20,
    Us30,
    Us40,
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub enum RssiMeasurementDelay {
    Us10,
    Us20,
    Us30,
    Us40,
    Us50,
    Us60,
    Us70,
    Us80,
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub struct Delay2 {
    pub xtal_settling_delay: XtalSettlingDelay,
    pub agc_delay_settling: AgcDelaySettling,
    pub rssi_measurement_delay: RssiMeasurementDelay,
}

impl Default for Delay2 {
    fn default() -> Self {
        Self {
            xtal_settling_delay: XtalSettlingDelay::Us600,
            agc_delay_settling: AgcDelaySettling::Us10,
            // default value is 20uS, but datasheet suggets using 10uS
            rssi_measurement_delay: RssiMeasurementDelay::Us10,
        }
    }
}

impl Register for Delay2 {
    fn id() -> u8 {
        0x17
    }
}

impl WritableRegister for Delay2 {}

impl Into<u8> for Delay2 {
    fn into(self) -> u8 {
        (match self.xtal_settling_delay {
            XtalSettlingDelay::Us200 => 0b000,
            XtalSettlingDelay::Us400 => 0b001,
            XtalSettlingDelay::Us600 => 0b010,
            XtalSettlingDelay::Us800 => 0b011,
            XtalSettlingDelay::Us1000 => 0b100,
            XtalSettlingDelay::Us1500 => 0b101,
            XtalSettlingDelay::Us2000 => 0b110,
            XtalSettlingDelay::Us2500 => 0b111,
        } << 5)
            | match self.agc_delay_settling {
                AgcDelaySettling::Us10 => 0b00,
                AgcDelaySettling::Us20 => 0b01,
                AgcDelaySettling::Us30 => 0b10,
                AgcDelaySettling::Us40 => 0b11,
            } << 3
            | match self.rssi_measurement_delay {
                RssiMeasurementDelay::Us10 => 0b000,
                RssiMeasurementDelay::Us20 => 0b001,
                RssiMeasurementDelay::Us30 => 0b010,
                RssiMeasurementDelay::Us40 => 0b011,
                RssiMeasurementDelay::Us50 => 0b100,
                RssiMeasurementDelay::Us60 => 0b101,
                RssiMeasurementDelay::Us70 => 0b110,
                RssiMeasurementDelay::Us80 => 0b111,
            }
    }
}

#[cfg(test)]
mod test {
    use super::super::Register as _;
    use super::*;

    #[test]
    fn test_delay1() {
        let default: u8 = Delay1::default().into();
        assert_eq!(default, 0b0001_0010);

        assert_eq!(Delay1::id(), 0x16);
    }

    #[test]
    fn test_delay2() {
        let default: u8 = Delay2::default().into();
        assert_eq!(default, 0b0100_0000);

        assert_eq!(Delay2::id(), 0x17);
    }
}
