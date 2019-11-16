# Copyright 2018 First Rust Competition Developers.
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

local_dir := $(dir $(lastword $(MAKEFILE_LIST)))
toolchain := $$CXX_FRC

# Set default toolchain
ifndef CXX_FRC
toolchain := arm-frc2019-linux-gnueabi-g++
endif

# Get the primary frc includes folder (arm-frcXXXX-linux-gnueabi/usr/include)
includes := $(shell cd ../wpilib; echo -e '\n' \
| $(toolchain) -E -Wp,-v - 2>&1 > /dev/null \
| sed -e '1,/> search starts here/d' \
| sed -e '/End of search list/,$$d' \
| xargs -I '{}' find '{}' -type d -path */usr/include )


# compile libs and assemble an include dir for rust-bindgen
.PHONY: sys_build wpilib_compile sys_libs load_headers wpilib_repo sys_clean wpilib_clean

sys_build: local_dir:=$(local_dir)
sys_build: hal_gen sys_libs gen_version
	cd $(local_dir); cargo build

sys_ci: local_dir:=$(local_dir)
sys_ci: sys_build
	cd $(local_dir); cargo clippy --all-targets --all-features -- -D warnings
	cd $(local_dir); cargo fmt -- --check

wpilib_compile: local_dir:=$(local_dir)
wpilib_compile: wpilib_repo
	cd $(local_dir)/allwpilib; ./gradlew :hal:halReleaseSharedLibrary -PreleaseBuild -PonlyAthena --console=plain --no-scan

sys_libs: local_dir:=$(local_dir)
sys_libs: wpilib_compile
	cp $(local_dir)/allwpilib/hal/build/libs/hal/shared/release/*.so $(local_dir)/lib/
	cp $(local_dir)/allwpilib/wpiutil/build/libs/wpiutil/shared/release/*.so $(local_dir)/lib/
	cp $(local_dir)/allwpilib/build/tmp/expandedArchives/chipobject*/linux/athena/shared/* $(local_dir)/lib/
	cp $(local_dir)/allwpilib/build/tmp/expandedArchives/netcomm*/linux/athena/shared/* $(local_dir)/lib/

	# strip version tags
	rename -f 's/.so.*/.so/' $(local_dir)/lib/*

load_headers: local_dir:=$(local_dir)
load_headers: wpilib_repo wpilib_compile
	cp -R $(local_dir)/allwpilib/hal/src/main/native/include/hal $(local_dir)/include/
	cp -R $(local_dir)/allwpilib/hal/build/generated/headers/hal $(local_dir)/include/
	cp -R $(local_dir)/allwpilib/wpiutil/src/main/native/include/* $(local_dir)/include/
	cp -R $(local_dir)/allwpilib/ntcore/src/main/native/include/* $(local_dir)/include/

	cp -R $(local_dir)/allwpilib/build/tmp/expandedArchives/chipobject*headers*/* $(local_dir)/include
	cp -R $(local_dir)/allwpilib/build/tmp/expandedArchives/netcomm*headers*/* $(local_dir)/include

	cp -R $(includes)/gnu $(local_dir)/include/
	cp -R $(includes)/sys $(local_dir)/include/
	cp -R $(includes)/bits $(local_dir)/include/
	cp -R $(includes)/*.h $(local_dir)/include/

	# Since the include folder is arm-frc-linux-gnueabi/usr/include, we need to backtrack to
	# arm-frc-linux-gnueabi/usr/lib to search for stddef.h
	find $(includes)/.. -type f -path */include/stddef.h | xargs -I {} cp {} $(local_dir)/include/

gen_version: local_dir:=$(local_dir)
gen_version: wpilib_repo
	echo "pub static WPILIB_COMMIT_HASH: &str = \"$(shell git ls-files -s ./allwpilib | cut -d ' ' -f 2)\";" > $(local_dir)/src/version.rs

sys_clean: local_dir:=$(local_dir)
sys_clean:
	rm -rf $(local_dir)/lib/*
	rm -rf $(local_dir)/include/*

wpilib_clean: local_dir:=$(local_dir)
wpilib_clean: wpilib_repo
	cd $(local_dir)/allwpilib; ./gradlew clean

wpilib_repo: local_dir:=$(local_dir)
wpilib_repo:
	git submodule sync
	git submodule update --init --recursive

clean += sys_clean
clean += wpilib_clean
