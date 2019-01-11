# Copyright 2018 First Rust Competition Developers.
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

include wpilib/wpilib.mk
include cargo-frc/cargo-frc.mk
include hal-gen/hal-gen.mk
include wpilib-sys/wpilib-sys.mk
include wpilib-examples/wpilib-examples.mk

.PHONY: all build ci clean

all: build

ci: $(ci)
	sh publish.sh

build: $(build)

clean: $(clean)
	cargo clean
