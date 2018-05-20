extern crate first_rust_competition;
use first_rust_competition::*;
use std::{thread, time};

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
