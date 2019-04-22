extern crate wpilib;

#[derive(Debug)]
struct Robot {}

impl wpilib::IterativeRobot for Robot {
    fn disabled_init(&mut self) {
        println!("Transitioning to disabled.");
    }

    fn autonomous_init(&mut self) {
        println!("Transitioning to autonomous mode.");
    }

    fn teleop_init(&mut self) {
        println!("Transitioning to teleoperated mode.");
    }

    fn test_init(&mut self) {
        println!("Transitioning to test mode.");
    }

    fn disabled_periodic(&mut self) {}

    fn autonomous_periodic(&mut self) {}

    fn teleop_periodic(&mut self) {}

    fn test_periodic(&mut self) {}

    fn robot_periodic(&mut self) {}
}

fn main() {
    let base = wpilib::RobotBase::new().unwrap();
    let ds = base.make_ds();

    let mut robot = Robot {};

    wpilib::start_timed(&mut robot, &ds)
}
