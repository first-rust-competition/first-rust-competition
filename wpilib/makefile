.PHONY: all wpilib_hal rust_build clean

all: rust_build

hal:
	cd HAL; make all

rust_build: hal
	cargo build

clean:
	cd HAL; make clean
	cargo clean;
