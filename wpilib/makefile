# This file is part of "first-rust-competition", which is free software: you
# can redistribute it and/or modify it under the terms of the GNU General
# Public License version 3 as published by the Free Software Foundation. See
# <https://www.gnu.org/licenses/> for a copy.

.PHONY: all wpilib_hal rust_build clean hal

all: rust_build

hal:
	cd HAL; make all

rust_build: hal
	cargo build
	cargo clippy --all-targets --all-features -- -D warnings

clean:
	cd HAL; make clean
	cargo clean;
