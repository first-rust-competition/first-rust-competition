// This file is part of "first-rust-competition", which is free software: you
// can redistribute it and/or modify it under the terms of the GNU General
// Public License version 3 as published by the Free Software Foundation. See
// <https://www.gnu.org/licenses/> for a copy.

extern crate wpilib;
use std::{thread, time};
use wpilib::*;

fn main() {
    let _robot = RobotBase::new().expect("HAL FAILED");
    let pdp = PowerDistributionPanel::new().expect("Could not make PDP");
    RobotBase::start_competition();

    loop {
        print_pdp_info(&pdp);
        thread::sleep(time::Duration::from_millis(100));
        pdp.clear_sticky_faults().expect("CAN Timeout");
        pdp.reset_total_energy().expect("CAN Timeout");
    }
}

fn print_pdp_info(pdp: &PowerDistributionPanel) {
    println!(
        "===== PDP =====
        Voltage: {} Volts
        Temperature: {} deg C
        Total Current: {} Amps
        Current on 1: {} Amps
        Total Power: {} Watts
        Total Energy: {} Joules",
        pdp.get_voltage().ok(),
        pdp.get_temperature().ok(),
        pdp.get_total_current().ok(),
        pdp.get_current(1).ok(),
        pdp.get_total_power().ok(),
        pdp.get_total_energy().ok()
    );
}
