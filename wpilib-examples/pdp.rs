extern crate wpilib;
use std::{thread, time};
use wpilib::*;

fn main() {
    let _robot = RobotBase::new().expect("HAL FAILED");
    let pdp = PowerDistributionPanel::new().expect("Could not make PDP");
    RobotBase::start_competition();

    loop {
        print_pdp_info(&pdp).expect("Couldn't access pdp info");
        thread::sleep(time::Duration::from_millis(100));
    }
}

fn print_pdp_info(pdp: &PowerDistributionPanel) -> HalResult<()> {
    println!(
        "===== PDP =====
        Voltage: {} Volts
        Temperature: {} deg C
        Total Current: {} Amps
        Current on 1: {} Amps
        Total Power: {} Watts
        Total Energy: {} Joules",
        pdp.get_voltage()?,
        pdp.get_temperature()?,
        pdp.get_total_current()?,
        pdp.get_current(1)?,
        pdp.get_total_power()?,
        pdp.get_total_energy()?
    );
    Ok(())
}
