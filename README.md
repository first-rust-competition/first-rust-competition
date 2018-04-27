# FIRST Rust Competition

Rewrite it in rust.

## Building
1. Init and pull the WPILib submodule
2. Building WPILib. Verify you satisfy the [requirements](https://github.com/wpilibsuite/allwpilib#building-wpilib), then `cd HAL` and `make all`.
3. Generate rust-bindings and build the library. `cargo build`.

## TODO
1. update and merge this PR on a fork of wpilib so rust-bindgen can use C-headers
https://github.com/wpilibsuite/allwpilib/pull/535/commits/27b494baeb8dfbfc68a885644e37e089754e0e45
2. Verify the HAL works on a test RIO.
3. Write the lib.
