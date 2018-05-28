# compile libs and assemble an include dir for rust-bindgen
.PHONY: all hal_compile wpilibc_compile libs load_headers clean clean_local clean_wpilib allwpilib_repo

all: load_headers libs

hal_compile: allwpilib_repo
	cd allwpilib; ./gradlew --console=plain :hal:build -PreleaseBuild

wpilibc_compile: allwpilib_repo
	cd allwpilib; ./gradlew --console=plain :wpilibc:build -PreleaseBuild

libs: hal_compile wpilibc_compile
	cp ./allwpilib/ni-libraries/lib/* ./lib/
	cp ./allwpilib/hal/build/libs/hal/shared/athena/*.so ./lib/
	cp ./allwpilib/ntcore/build/libs/ntcore/shared/athena/*.so ./lib/
	cp ./allwpilib/wpiutil/build/libs/wpiutil/shared/athena/*.so ./lib/
	cp ./allwpilib/cscore/build/libs/cscore/shared/athena/*.so ./lib/
	# strip versions tags
	rename -f 's/.so.*/.so/' ./lib/*

load_headers: allwpilib_repo
	cp --recursive ./allwpilib/hal/src/main/native/include/HAL/ ./include/
	cp --recursive ./allwpilib/ni-libraries/include/* ./include/
	cp --recursive ./allwpilib/wpiutil/src/main/native/include/* ./include/
	cp --recursive ./allwpilib/ntcore/src/main/native/include/* ./include/

clean: clean_local clean_wpilib

clean_local:
	rm -rf ./lib/*
	rm -rf ./include/*

clean_wpilib: allwpilib_repo
	cd ./allwpilib; ./gradlew clean

allwpilib_repo:
	git submodule sync
	git submodule update --init --recursive
