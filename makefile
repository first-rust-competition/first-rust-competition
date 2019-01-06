# Copyright 2018 First Rust Competition Developers.
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

.PHONY: all cargo-frc wpilib ci

all: wpilib cargo-frc

ci: cargo-frc wpilib
	sh publish.sh

cargo-frc:
	cd cargo-frc; make all

wpilib:
	cd wpilib-sys; make load_headers
	cd hal-gen; make all
	cd wpilib-sys; make all
	cd wpilib; make all
	cd wpilib-examples; make all
