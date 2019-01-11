# Copyright 2018 First Rust Competition Developers.
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

.PHONY: cfrc_build cfrc_ci

local_dir := $(dir $(lastword $(MAKEFILE_LIST)))


cfrc_build: local_dir:=$(local_dir)
cfrc_build:
	cd $(local_dir); cargo build

cfrc_ci: local_dir:=$(local_dir)
cfrc_ci: cfrc_build
	cd $(local_dir); cargo fmt -- --check
	cd $(local_dir); cargo clippy --all-targets --all-features -- -D warnings

build += cfrc_build
ci += cfrc_ci
