// This file is part of "first-rust-competition", which is free software: you
// can redistribute it and/or modify it under the terms of the GNU General
// Public License version 3 as published by the Free Software Foundation. See
// <https://www.gnu.org/licenses/> for a copy.

extern crate wpilib;
use std::{thread, time};
use wpilib::*;

fn main() {
    let robot = RobotBase::new().expect("HAL FAILED");
    let mut out = DigitalOutput::new(1).expect("Could not make digital output");
    RobotBase::start_competition();

    let mut val;
    let ds = robot.get_ds_instance();
    loop {
        {
            val = match ds.read().unwrap().get_state() {
                ds::RobotState::Disabled => true,
                _ => false,
            }
        }
        println!("Setting output to {}", val);
        out.set(val).expect("Could not set DIO");
        thread::sleep(time::Duration::from_millis(100));
    }
}
