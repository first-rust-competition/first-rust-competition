use hal::*;

pub struct RobotBase {}

impl RobotBase {
    /// Create a new robot, initializing hardware in the process.
    /// Call before initializing any other wpilib stuff.
    pub fn new() -> Result<RobotBase, &'static str> {
        if unsafe { HAL_Initialize(500, 0) } != 0 {
            return Err("HAL Initialized Failed");
        }
        report_usage(
            tResourceType_kResourceType_Language,
            tInstances_kLanguage_CPlusPlus, // one day, we will have our own.
        );
        println!("\n********** Hardware Init **********\n");
        return Ok(RobotBase {});
    }

    /// Call when your robot is ready to be enabled.
    /// Make sure your hardware and threads have been created, etc.
    pub fn start_competition() {
        unsafe {
            HAL_ObserveUserProgramStarting();
        }
        println!("\n********** Robot program starting **********\n");
    }
}
