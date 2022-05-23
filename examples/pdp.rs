// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate wpilib;
use std::{thread, time};
use wpilib::{PowerDistributionPanel, RobotBase};

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
        pdp.voltage().ok(),
        pdp.temperature().ok(),
        pdp.total_current().ok(),
        pdp.current(1).ok(),
        pdp.total_power().ok(),
        pdp.total_energy().ok()
    );
}
