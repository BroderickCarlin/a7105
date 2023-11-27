# a7105

## Overview

`a7105` is a Rust crate that provides a high-level interface for interacting with the A7105 2.4GHz FSK/GFSK Transceiver, built on top of [`embedded-hal`](https://crates.io/crates/embedded-hal) traits. This crate supports both synchronous (sync) and asynchronous (async) APIs, allowing users to choose the mode that best fits their application requirements.

This crate makes no assumptions for the protocol, if any, used on top of the a7105. Instead, the responsibility of this crate end at configuring the radio and reading/writing raw bytes over the air. 

Sync and Async support is through [`embedded-hal`](https://crates.io/crates/embedded-hal) and [`embedded-hal-async`](https://crates.io/crates/embedded-hal-async), configurable through the `async` and `blocking` features. By default the crate will use `async` variants. If blocking APIs are preferred, the `blocking` feature can be specified and default features disabled. 

## Getting Started

### Installation

Add the following line to your Cargo.toml file:

```toml
[dependencies]
a7105 = "0.1.0"
```

### Example (Async)

This example utilizes the default async APIs. The blocking API has the exact same function signatures.  

```rust
use a7105::prelude::*;

// Get a handle to the SPI peripheral attached to the A7105. This step
// will be highly specific to the hardware used and if a blocking or 
// async peripheral is being used
let spi = unimplemented!();

let mut radio = A7105::new(spi);

// It is usually a good idea to reset the radio before anything else
radio.command(Command::Reset).await.unwrap();

// Write a register, in this example setting IdData to 0x01234567
radio.write_reg(registers::IdData { 
    id: 0x01234567
}).await.unwrap();

// Read a register, in this example `DataRate`
let data_rate: registers::DataRate = radio.read_reg().await.unwrap();

// Transmit the given bytes 
radio.tx(&[0, 1, 2, 3]).await.unwrap();

// Receive a set number of bytes into the provided buffer
let mut buf = [0; 8];
radio.rx(&mut buf).await.unwrap();

// Set the radio's mode
radio.set_mode(Mode::Idle).await.unwrap();

// Destroys the radio instance and gets back the SPI peripheral
let spi = radio.destroy();
```

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This work is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.