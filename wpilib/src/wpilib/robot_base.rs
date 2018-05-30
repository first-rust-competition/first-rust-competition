use super::ds::*;
use hal::*;
use std::sync::*;

pub struct RobotBase {
    ds: ThreadSafeDs,
}

impl RobotBase {
    /// Create a new robot, initializing hardware in the process.
    /// Call before initializing any other wpilib stuff.
    pub fn new() -> Result<RobotBase, &'static str> {
        if unsafe { HAL_Initialize(500, 0) } == 0 {
            return Err("HAL Initialized Failed");
        }
        report_usage(
            nUsageReporting_tResourceType_kResourceType_Language,
            nUsageReporting_tInstances_kLanguage_CPlusPlus, // one day, we will have our own.
        );
        println!("\n********** Hardware Init **********\n");
        let mut ds = Arc::new(RwLock::new(DriverStation::new()));
        DriverStation::spawn_updater(&mut ds);
        Ok(RobotBase { ds })
    }

    /// Call when your robot is ready to be enabled.
    /// Make sure your hardware and threads have been created, etc.
    pub fn start_competition() {
        unsafe {
            HAL_ObserveUserProgramStarting();
        }
        println!("\n********** Robot program starting **********\n");
    }

    pub fn get_ds_instance(&self) -> ThreadSafeDs {
        self.ds.clone()
    }
}
