// This file is part of "first-rust-competition", which is free software: you
// can redistribute it and/or modify it under the terms of the GNU General
// Public License version 3 as published by the Free Software Foundation. See
// <https://www.gnu.org/licenses/> for a copy.

extern crate wpilib;
use std::{thread, time};
use wpilib::*;

fn main() {
    let robot = RobotBase::new().expect("HAL FAILED");
    let solenoid = pneumatics::DoubleSolenoid::new(4, 5).expect("Could not make DoubleSolenoid");
    RobotBase::start_competition();

    let mut val;
    let ds = robot.get_ds_instance();
    loop {
        {
            val = match ds.read().unwrap().get_state() {
                ds::RobotState::Disabled => pneumatics::Action::Forward,
                _ => pneumatics::Action::Reverse,
            }
        }
        println!("Setting solenoid to {:?}", val);
        solenoid.set(val).expect("Could not set Solenoid");
        thread::sleep(time::Duration::from_millis(100));
    }
}
