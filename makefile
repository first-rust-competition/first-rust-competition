.PHONY: all cargo-frc wpilib

all: cargo-frc wpilib
	:

cargo-frc:
	cd cargo-frc; cargo build

wpilib:
	cd wpilib; make all
	cd wpilib-examples; cargo build
