use super::*;
use defmt::Format;

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub struct Fifo1 {
    /// FIFO End Pointer for TX FIFO and Rx FIFO
    pub end_pointer: u8,
}

impl Default for Fifo1 {
    fn default() -> Self {
        Self {
            end_pointer: 0b0011_1111,
        }
    }
}

impl Register for Fifo1 {
    fn id() -> u8 {
        0x03
    }
}

impl WritableRegister for Fifo1 {}

impl From<Fifo1> for u8 {
    fn from(val: Fifo1) -> u8 {
        val.end_pointer
    }
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub struct Fifo2 {
    /// FIFO Pointer Margin
    pub margin: u8,
    /// Used for Segment FIFO, Refer to chapter 16 of the datasheet for details
    pub segment: u8,
}

impl Default for Fifo2 {
    fn default() -> Self {
        Self {
            margin: 1,
            segment: 0,
        }
    }
}

impl Register for Fifo2 {
    fn id() -> u8 {
        0x04
    }
}

impl WritableRegister for Fifo2 {}

impl From<Fifo2> for u8 {
    fn from(val: Fifo2) -> u8 {
        (val.segment & 0b0011_1111) | (val.margin << 6)
    }
}

#[cfg(test)]
mod test {
    use super::super::Register as _;
    use super::*;

    #[test]
    fn test_fifo1() {
        assert_eq!(Fifo1::id(), 0x3);
    }

    #[test]
    fn test_fifo2() {
        assert_eq!(Fifo2::id(), 0x4);
    }
}
