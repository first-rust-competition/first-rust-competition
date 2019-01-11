# Copyright 2018 First Rust Competition Developers.
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

local_dir := $(dir $(lastword $(MAKEFILE_LIST)))

.PHONY: hal_gen hal_gen_ci

hal_gen: local_dir:=$(local_dir)
hal_gen: load_headers
	echo "dir: $(dir)";
	cd $(local_dir); cargo run

hal_gen_ci: local_dir:=$(local_dir)
hal_gen_ci: hal_gen
	cd $(local_dir); cargo fmt -- --check
	cd $(local_dir); cargo clippy --all-targets --all-features -- -D warnings

ci += hal_gen_ci
