use super::*;
use defmt::Format;

// #[derive(Format, PartialEq, Debug, Copy, Clone)]
// pub enum TrxStatus {
//     Rx,
//     Tx,
// }

// #[derive(Format, PartialEq, Debug, Copy, Clone)]
// pub struct Mode {
//     /// FEC is read only, it is updated internally while receiving every packet.
//     pub fec_pass: bool,
//     /// CRCF is read only, it is updated internally while receiving every packet.
//     pub crc_pass: bool,
//     /// RF Radio is enabled
//     pub rf_enabled: bool,
//     /// Internal Crystal Oscillator is enabled
//     pub internal_crystal_enabled: bool,
//     /// PLL is enabled
//     pub pll_enabled: bool,
//     /// TRX is enabled
//     pub trx_enabled: bool,
//     /// TX/RX state
//     pub trx_status: TrxStatus,
// }

// impl Register for Mode {
//     fn id() -> u8 {
//         0x00
//     }
// }

// impl ReadableRegister for Mode {}

// impl From<u8> for Mode {
//     fn from(val: u8) -> Self {
//         Self {
//             fec_pass: 0b0100_0000 & val != 0,
//             crc_pass: 0b0010_0000 & val != 0,
//             rf_enabled: 0b0001_0000 & val != 0,
//             internal_crystal_enabled: 0b0000_1000 & val != 0,
//             pll_enabled: 0b0000_0100 & val != 0,
//             trx_enabled: 0b0000_0010 & val != 0,
//             trx_status: if 0b0000_0001 & val != 0 {
//                 TrxStatus::Tx
//             } else {
//                 TrxStatus::Rx
//             },
//         }
//     }
// }

#[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
#[non_exhaustive]
pub struct Reset {}

impl Register for Reset {
    fn id() -> u8 {
        0x00
    }
}

impl WritableRegister for Reset {}

impl Into<u8> for Reset {
    fn into(self) -> u8 {
        0
    }
}

// #[derive(Format, PartialEq, Debug, Copy, Clone)]
// pub enum DirectDataPin {
//     SDIO,
//     GPIO,
// }

// impl Default for DirectDataPin {
//     fn default() -> Self {
//         Self::GPIO
//     }
// }

// #[derive(Format, PartialEq, Debug, Copy, Clone)]
// pub enum DataMode {
//     Direct,
//     FIFO,
// }

// impl Default for DataMode {
//     fn default() -> Self {
//         Self::Direct
//     }
// }

// #[derive(Format, PartialEq, Debug, Copy, Clone, Default)]
// pub struct ModeControl {
//     pub direct_data_pin_output: DirectDataPin,
//     ///  Auto RSSI measurement while entering RX mode
//     pub auto_rssi: bool,
//     /// RF LO frequency will auto offset one IF frequency while entering RX mode
//     pub auto_if: bool,
//     /// The received packet will be filtered out if CD is inactive
//     pub cd_filter: bool,
//     /// Direct/FIFO mode select
//     pub data_mode: DataMode,
//     /// ADC measurement enable (Auto clear when done)
//     pub adc_measurement_enabled: bool,
// }

// impl Register for ModeControl {
//     fn id() -> u8 {
//         0x01
//     }
// }

// impl ReadableRegister for ModeControl {}

// impl WritableRegister for ModeControl {}

// impl From<u8> for ModeControl {
//     fn from(val: u8) -> Self {
//         Self {
//             direct_data_pin_output: if 0b1000_0000 & val != 0 {
//                 DirectDataPin::SDIO
//             } else {
//                 DirectDataPin::GPIO
//             },
//             auto_rssi: 0b0100_0000 & val != 0,
//             auto_if: 0b0010_0000 & val != 0,
//             cd_filter: 0b0001_0000 & val != 0,
//             data_mode: if 0b0000_0010 & val != 0 {
//                 DataMode::Direct
//             } else {
//                 DataMode::FIFO
//             },
//             adc_measurement_enabled: 0b0000_0001 & val != 0,
//         }
//     }
// }

// impl Into<u8> for ModeControl {
//     fn into(self) -> u8 {
//         u8::from(self.direct_data_pin_output == DirectDataPin::SDIO) << 7
//             | u8::from(self.auto_rssi) << 6
//             | u8::from(self.auto_if) << 5
//             | u8::from(self.cd_filter) << 4
//             | u8::from(self.data_mode == DataMode::FIFO) << 1
//             | u8::from(self.adc_measurement_enabled)
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::super::Register as _;
//     use super::*;

//     #[test]
//     fn test_reset() {
//         assert_eq!(Reset::id(), 0x0);
//     }

//     #[test]
//     fn test_mode_control() {
//         let default: u8 = ModeControl::default().into();
//         assert_eq!(default, 0);

//         assert_eq!(ModeControl::id(), 0x1);
//     }
// }
