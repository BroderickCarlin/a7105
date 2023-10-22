use super::*;
use defmt::Format;

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub enum MovingAverage {
    TwoBit,
    FourBit,
    EightBit,
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub struct Tx1 {
    // Moving average for non-filter select
    pub moving_average: Option<MovingAverage>,
    pub data_invert: bool,
    pub modulation_enable: bool,
    pub filter_enable: bool,
    pub fdp: u8,
}

impl Default for Tx1 {
    fn default() -> Self {
        Self {
            moving_average: None,
            data_invert: false,
            modulation_enable: true,
            filter_enable: false,
            fdp: 0b110,
        }
    }
}

impl Register for Tx1 {
    fn id() -> u8 {
        0x14
    }
}

impl WritableRegister for Tx1 {}

impl From<Tx1> for u8 {
    fn from(val: Tx1) -> u8 {
        u8::from(val.data_invert) << 5
            | u8::from(val.modulation_enable) << 4
            | u8::from(val.filter_enable) << 3
            | val.fdp.min(0b111)
            | match val.moving_average {
                None => 0b00,
                Some(MovingAverage::TwoBit) => 0b01,
                Some(MovingAverage::FourBit) => 0b10,
                Some(MovingAverage::EightBit) => 0b11,
            } << 6
    }
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub struct Tx2 {
    // Moving average for non-filter select
    pub fd: u8,
}

impl Default for Tx2 {
    fn default() -> Self {
        Self { fd: 0b01011 }
    }
}

impl Register for Tx2 {
    fn id() -> u8 {
        0x15
    }
}

impl WritableRegister for Tx2 {}

impl From<Tx2> for u8 {
    fn from(val: Tx2) -> u8 {
        val.fd.min(0b11111) | 0b0010_0000
    }
}

#[cfg(test)]
mod test {
    use super::super::Register as _;
    use super::*;

    #[test]
    fn test_tx1() {
        let default: u8 = Tx1::default().into();
        assert_eq!(default, 0b0001_0110);

        assert_eq!(Tx1::id(), 0x14);
    }

    #[test]
    fn test_tx2() {
        let default: u8 = Tx2::default().into();
        assert_eq!(default, 0b0010_1011);

        assert_eq!(Tx2::id(), 0x15);
    }
}
