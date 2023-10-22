use defmt::Format;

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub enum Command {
    Reset,
    ResetFifoWritePointer,
    ResetFifoReadPointer,
}

#[derive(Format, PartialEq, Debug, Copy, Clone)]
pub enum Mode {
    Sleep,
    Idle,
    Standby,
    Pll,
    Rx,
    Tx,
}

impl From<Mode> for u8 {
    fn from(val: Mode) -> Self {
        match val {
            Mode::Sleep => 0b1000_0000,
            Mode::Idle => 0b1001_0000,
            Mode::Standby => 0b1010_0000,
            Mode::Pll => 0b1011_0000,
            Mode::Rx => 0b1100_0000,
            Mode::Tx => 0b1101_0000,
        }
    }
}
