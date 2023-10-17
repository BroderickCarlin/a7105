use super::*;
use defmt::Format;

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub struct DataRate {
    pub rate: u8,
}

impl Register for DataRate {
    fn id() -> u8 {
        0x0E
    }
}

impl ReadableRegister for DataRate {}
impl WritableRegister for DataRate {}

impl From<u8> for DataRate {
    fn from(rate: u8) -> Self {
        Self { rate }
    }
}

impl Into<u8> for DataRate {
    fn into(self) -> u8 {
        self.rate
    }
}

#[cfg(test)]
mod test {
    use super::super::Register as _;
    use super::*;

    #[test]
    fn test_data_rate() {
        let default: u8 = DataRate::default().into();
        assert_eq!(default, 0);

        assert_eq!(DataRate::id(), 0xE);
    }
}
