use super::*;
use defmt::Format;

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub enum SystemClockDiv {
    Div1,
    Div2,
    Div4,
}

impl Into<u8> for SystemClockDiv {
    fn into(self) -> u8 {
        match self {
            SystemClockDiv::Div1 => 0b00,
            SystemClockDiv::Div2 => 0b01,
            SystemClockDiv::Div4 => 0b10,
        }
    }
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub struct Clock {
    pub clock_generated_enabled: bool,
    pub external_crystal_osc: bool,
    pub clock_generation_ref_cnt: u8,
    pub sys_clock_div: SystemClockDiv,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            clock_generated_enabled: false,
            external_crystal_osc: true,
            clock_generation_ref_cnt: 0b1111,
            sys_clock_div: SystemClockDiv::Div2,
        }
    }
}

impl Register for Clock {
    fn id() -> u8 {
        0x0D
    }
}

impl ReadableRegister for Clock {}
impl WritableRegister for Clock {}

impl From<u8> for Clock {
    fn from(val: u8) -> Self {
        Self {
            clock_generated_enabled: (val & 0b10) != 0,
            external_crystal_osc: (val & 0b1) != 0,
            clock_generation_ref_cnt: (val & 0b1111_0000) >> 4,
            sys_clock_div: match (val & 0b1100) >> 2 {
                0b00 => SystemClockDiv::Div1,
                0b01 | 0b10 => SystemClockDiv::Div2,
                _ => SystemClockDiv::Div4,
            },
        }
    }
}

impl Into<u8> for Clock {
    fn into(self) -> u8 {
        u8::from(self.clock_generated_enabled) << 1
            | u8::from(self.external_crystal_osc)
            | self.clock_generation_ref_cnt.min(0b1111) << 4
            | Into::<u8>::into(self.sys_clock_div) << 2
    }
}

#[cfg(test)]
mod test {
    use super::super::Register as _;
    use super::*;

    #[test]
    fn test_clock() {
        let default: u8 = Clock::default().into();
        assert_eq!(default, 0b1111_0101);

        assert_eq!(Clock::id(), 0xD);
    }
}
