pub mod analog_input;
pub mod can;
pub mod dio;
pub mod ds;
pub mod encoder;
pub mod fpga;
pub mod hal;
pub mod i2c;
pub mod iterative_robot;
pub mod notifier;
pub mod observe;
pub mod pneumatics;
pub mod pwm;
pub mod relay;
pub mod robot;
pub mod serial;
pub mod spi;

// TODO: Figure out where the PDP HAL calls dissapeared to.
// mod pdp;

pub mod prelude {
    pub use crate::analog_input::AnalogInput;
    pub use crate::can::{Can, CanData};
    pub use crate::dio::{DigitalInput, DigitalOutput};
    pub use crate::ds::*;
}

pub use wpilib_sys;
