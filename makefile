.PHONY: all wpilib_hal rust_build clean

all: rust_build

hal: HAL/allwpilib/README.md
	cd HAL; make all

rust_build: hal
	cargo build

clean:
	cd HAL; make clean
	cargo clean;