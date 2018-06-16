extern crate wpilib;
use std::{thread, time};
use wpilib::*;

fn main() {
    let robot = RobotBase::new().expect("HAL FAILED");
    let solenoid = pneumatics::Solenoid::new(1).expect("Could not make Solenoid");
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
        println!("Setting solenoid to {}", val);
        solenoid.set(val).expect("Could not set Solenoid");
        thread::sleep(time::Duration::from_millis(100));
    }
}
