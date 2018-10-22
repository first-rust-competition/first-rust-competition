// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
