# compile libs and assemble a header for rust-bindgen
all: load_headers libs

hal_compile:
	cd allwpilib; ./gradlew :hal:build -PreleaseBuild

wpilibc_compile:
	cd allwpilib; ./gradlew :wpilibc:build -PreleaseBuild

libs: hal_compile
	cp ./allwpilib/ni-libraries/lib/* ./lib/
	cp ./allwpilib/hal/build/libs/halAthena/shared/*.so ./lib/
	cp ./allwpilib/build/dependencies/ntcore-cpp/linuxathena/linux/athena/shared/*.so ./lib/
	cp ./allwpilib/build/dependencies/wpiutil-cpp/linuxathena/linux/athena/shared/*.so ./lib/
	# strip versions tags
	rename -f 's/.so.*/.so/' ./lib/*

load_headers: hal_compile wpilibc_compile
	# Hal Headers
	unzip -d ./include/ -u ./allwpilib/hal/build/outputs/hal-headers.zip 
	rm -rf ./include/MockData
	# NI libs headers
	cp --recursive ./allwpilib/ni-libraries/include/* ./include/
	# WPILib Util headers
	cp --recursive ./allwpilib/build/dependencies/wpiutil-cpp/headers/* ./include/
	# ntcore headers
	cp --recursive ./allwpilib/build/dependencies/ntcore-cpp/headers/* ./include/


# bindgen_header.h: load_headers
	# find ./include/HAL/ -maxdepth 1 -not -type d | sed 's/\.\/include\/HAL\//#include "HAL\//' | sed 's/\.h/\.h"/' > ./include/bindgen_header.h

clean: clean_local clean_wpilib

clean_local:
	rm -rf ./lib/*
	rm -rf ./include/*

clean_wpilib:
	cd ./allwpilib; ./gradlew clean
