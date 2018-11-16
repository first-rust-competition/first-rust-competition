# Copyright 2018 First Rust Competition Developers.
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

.PHONY: all build clean

all: build

build:
	cargo build --features dev
	cargo clippy --all-targets --all-features -- -D warnings
	cargo fmt --all -- --check

clean:
	cargo clean
