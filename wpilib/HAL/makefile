# This file is part of "first-rust-competition", which is free software: you
# can redistribute it and/or modify it under the terms of the GNU General
# Public License version 3 as published by the Free Software Foundation. See
# <https://www.gnu.org/licenses/> for a copy.

# compile libs and assemble an include dir for rust-bindgen
.PHONY: all hal_compile libs load_headers clean clean_local clean_wpilib allwpilib_repo

all: load_headers libs

hal_compile: allwpilib_repo
	cd allwpilib; ./gradlew --console=plain halAthenaSharedLibrary -PreleaseBuild

libs: hal_compile
	cp ./allwpilib/ni-libraries/lib/* ./lib/
	cp ./allwpilib/hal/build/libs/hal/shared/athena/*.so ./lib/
	cp ./allwpilib/wpiutil/build/libs/wpiutil/shared/athena/*.so ./lib/
	# strip version tags
	rename -f 's/.so.*/.so/' ./lib/*

load_headers: allwpilib_repo
	cp -R ./allwpilib/hal/src/main/native/include/hal ./include/
	cp -R ./allwpilib/ni-libraries/include/* ./include/
	cp -R ./allwpilib/wpiutil/src/main/native/include/* ./include/
	cp -R ./allwpilib/ntcore/src/main/native/include/* ./include/

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

clean: clean_local clean_wpilib

clean_local:
	rm -rf ./lib/*
	rm -rf ./include/*

clean_wpilib: allwpilib_repo
	cd ./allwpilib; ./gradlew clean

allwpilib_repo:
	git submodule sync
	git submodule update --init --recursive
