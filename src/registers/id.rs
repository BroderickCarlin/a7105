use super::*;
use defmt::Format;

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
pub struct IdData {
    pub id: u32,
}

impl Register for IdData {
    fn id() -> u8 {
        0x06
    }
}

impl ReadableRegister<4> for IdData {}
impl WritableRegister<4> for IdData {}

impl From<u32> for IdData {
    fn from(id: u32) -> Self {
        Self { id }
    }
}

impl From<IdData> for u32 {
    fn from(val: IdData) -> u32 {
        val.id
    }
}

#[cfg(test)]
mod test {
    use super::super::Register as _;
    use super::*;

    #[test]
    fn test_id() {
        assert_eq!(IdData::id(), 0x6);
    }
}
