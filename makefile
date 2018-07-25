# This file is part of "first-rust-competition", which is free software: you
# can redistribute it and/or modify it under the terms of the GNU General
# Public License version 3 as published by the Free Software Foundation. See
# <https://www.gnu.org/licenses/> for a copy.

.PHONY: all cargo-frc wpilib

all: cargo-frc wpilib
	sh publish.sh

cargo-frc:
	cd cargo-frc; cargo build

wpilib:
	cd wpilib; make all
	cd wpilib-examples; cargo build
