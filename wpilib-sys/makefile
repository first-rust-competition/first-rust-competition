# Copyright 2018 First Rust Competition Developers.
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

# compile libs and assemble an include dir for rust-bindgen
.PHONY: all wpilib_compile libs load_headers clean clean_local clean_wpilib allwpilib_repo

all: load_headers libs gen_version
	cargo build
	cargo clippy --all-targets --all-features -- -D warnings
	cargo fmt -- --check

wpilib_compile: allwpilib_repo
	cd allwpilib; ./gradlew :hal:halReleaseSharedLibrary -PreleaseBuild -PonlyAthena --console=plain

libs: wpilib_compile
	cp ./allwpilib/hal/build/libs/hal/shared/release/*.so ./lib/
	cp ./allwpilib/wpiutil/build/libs/wpiutil/shared/release/*.so ./lib/
	cp ./allwpilib/build/tmp/expandedArchives/chipobject*/linux/athena/shared/* ./lib/
	cp ./allwpilib/build/tmp/expandedArchives/netcomm*/linux/athena/shared/* ./lib/

	# strip version tags
	rename -f 's/.so.*/.so/' ./lib/*

load_headers: allwpilib_repo wpilib_compile
	cp -R ./allwpilib/hal/src/main/native/include/hal ./include/
	cp -R ./allwpilib/hal/build/generated/headers/hal ./include/
	cp -R ./allwpilib/wpiutil/src/main/native/include/* ./include/
	cp -R ./allwpilib/ntcore/src/main/native/include/* ./include/

	cp -R ./allwpilib/build/tmp/expandedArchives/chipobject*headers*/* ./include
	cp -R ./allwpilib/build/tmp/expandedArchives/netcomm*headers*/* ./include

	# TODO(lytigas) move this functionality into the python script
	# TODO(lytigas) find a better method for selecting the include dir than the one without version information
	# which is marked currently by the existence of globs.h

	# gnu/**/*.h
	python load-gcc-arm-headers.py | xargs -I '{}' find '{}' -type d -name "gnu" | xargs -I '{}' cp -R '{}' ./include/
	# sys/**/*.h
	python load-gcc-arm-headers.py | xargs -I '{}' find '{}' -type d -name "sys" | xargs -I '{}' cp -R '{}' ./include/
	# *.h in one of the include dirs that is marked by glob.h
	python load-gcc-arm-headers.py | xargs -I '{}' find '{}' -type f -name "glob.h" | xargs dirname | xargs -I '{}' bash -c 'cp -R {}/*.h ./include/'
	# same folder us a above but its the bits directory
	python load-gcc-arm-headers.py | xargs -I '{}' find '{}' -type f -name "glob.h" | xargs dirname | xargs -I '{}' cp -R '{}/bits' ./include/
	# stddef.h
	python load-gcc-arm-headers.py | xargs -I '{}' find '{}' -type f -path "*/include/stddef.h" | xargs -I '{}' cp -R '{}' ./include/

gen_version: allwpilib_repo
	echo "pub static WPILIB_COMMIT_HASH: &str = \"$(shell git ls-files -s ./allwpilib | cut -d ' ' -f 2)\";" > src/version.rs

clean: clean_local clean_wpilib

clean_local:
	rm -rf ./lib/*
	rm -rf ./include/*

clean_wpilib: allwpilib_repo
	cd ./allwpilib; ./gradlew clean

allwpilib_repo:
	git submodule sync
	git submodule update --init --recursive
