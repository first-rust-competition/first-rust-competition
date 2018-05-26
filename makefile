.PHONY: all

all:
	cd wpilib; make all
	cd cargo-frc; cargo build
	cd wpilib-examples; cargo build
