/*
Copyright 2020 First Rust Competition Developers.
Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
option. This file may not be copied, modified, or distributed
except according to those terms.
*/

//! Interface to HIDs such as joysticks and gamepads attached to the Driver Station.

use wpilib_sys::{HAL_GetJoystickAxes, HAL_GetJoystickButtons, HAL_GetJoystickPOVs};
use wpilib_sys::{HAL_JoystickAxes, HAL_JoystickButtons, HAL_JoystickPOVs};
use wpilib_sys::{HAL_kMaxJoystickAxes, HAL_kMaxJoystickPOVs, HAL_kMaxJoysticks};

/// A valid joystick "port" on the Driver Station.
#[derive(Copy, Clone, Debug, Default)]
pub struct Port(i32);
impl Port {
    const MAX: u8 = HAL_kMaxJoysticks;

    /// Creates a new port without checking the value.
    ///
    /// # Safety
    ///
    /// The port number must not be out of bounds.
    pub const unsafe fn new_unchecked(port: u8) -> Self {
        Self(port as i32)
    }

    /// Creates a new port from a port number if it is valid.
    pub fn new(port: u8) -> Option<Self> {
        if port >= Self::MAX {
            None
        } else {
            Some(Self(i32::from(port)))
        }
    }

    /// Read the axes from this port.
    #[inline]
    pub fn axes(self) -> Axes {
        Axes::of(self)
    }

    /// Read the buttons from this port.
    #[inline]
    pub fn buttons(self) -> Buttons {
        Buttons::of(self)
    }

    /// Read the POVs from this port.
    #[inline]
    pub fn povs(self) -> Povs {
        Povs::of(self)
    }
}

/// A valid joystick axis index.
#[derive(Copy, Clone, Debug)]
pub struct Axis(pub(crate) usize);
impl Axis {
    const MAX: u8 = HAL_kMaxJoystickAxes;

    /// Axis 0, commonly the X axis on a joystick.
    /// This is the left thumbstick X axis on an XInput controller.
    pub const X: Self = Self(0);
    /// Axis 1, commonly the Y axis on a joystick.
    /// This is the left thumbstick Y axis on an XInput controller.
    pub const Y: Self = Self(1);
    /// Axis 2, commonly the Z axis or twist on a joystick.
    pub const Z: Self = Self(2);
    /// Axis 2, commonly the Z axis or twist on a joystick.
    pub const TWIST: Self = Self::Z;
    /// Axis 3, commonly the throttle on a joystick.
    pub const THROTTLE: Self = Self(3);

    /// Axis 4, the right thumbstick X axis on an XInput controller
    /// (such as Xbox controllers).
    pub const RIGHT_X: Self = Self(4);
    /// Axis 5, the right thumbstick X axis on an XInput controller
    /// (such as Xbox controllers).
    pub const RIGHT_Y: Self = Self(5);
    /// Axis 2, the left trigger axis on an XInput controller
    /// (such as Xbox controllers).
    pub const LEFT_TRIGGER: Self = Self(2);
    /// Axis 3, the right trigger axis on an XInput controller
    /// (such as Xbox controllers).
    pub const RIGHT_TRIGGER: Self = Self(3);

    /// Creates a new axis without checking the value.
    ///
    /// # Safety
    ///
    /// The axis index should not be out of bounds.
    pub const unsafe fn new_unchecked(axis: u8) -> Self {
        Self(axis as usize)
    }

    /// Creates a new axis from an axis index if the index is valid.
    pub fn new(axis: u8) -> Option<Self> {
        if axis >= Self::MAX {
            None
        } else {
            Some(Self(usize::from(axis)))
        }
    }
}

/// A valid joystick POV hat index.
#[derive(Copy, Clone, Debug, Default)]
pub struct Pov(pub(crate) usize);
impl Pov {
    const MAX: u8 = HAL_kMaxJoystickPOVs;

    /// Creates a new POV without checking the value.
    ///
    /// # Safety
    ///
    /// The index should not be out of bounds.
    pub const unsafe fn new_unchecked(pov: u8) -> Self {
        Self(pov as usize)
    }

    /// Creates a new POV hat from an index.
    pub fn new(pov: u8) -> Option<Self> {
        if pov >= Self::MAX {
            None
        } else {
            Some(Self(usize::from(pov)))
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// Buttons for XInput controllers (such as Xbox controllers).
pub enum XInputButton {
    A,
    B,
    X,
    Y,

    /// The left bumper/shoulder button.
    LeftBumper,
    /// The right bumper/shoulder button.
    RightBumper,

    Back,
    Start,

    /// The left thumbstick button.
    LeftThumb,
    /// The right thumbstick button.
    RightThumb,
}

impl From<XInputButton> for u8 {
    #[inline]
    fn from(button: XInputButton) -> u8 {
        button as u8
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Buttons(HAL_JoystickButtons);
impl Buttons {
    /// Get the current button state of the specified HID.
    #[inline]
    pub fn of(port: Port) -> Self {
        Self(buttons(port))
    }

    /// Get the state of the 0-indexed button `button`.
    ///
    /// Returns None if the button doesn't exist.
    /// This may mean the controller is unplugged.
    ///
    /// This method may take a `u8` or an [`XInputButton`].
    ///
    /// [`XInputButton`]: enum.XInputButton.html
    pub fn get(self, button: impl Into<u8>) -> Option<bool> {
        let button = button.into();
        if button >= self.0.count {
            None
        } else {
            Some(self.0.buttons & (1 << button) != 0)
        }
    }

    /// Get the number of buttons read.
    /// Returns 0 if the controller is unplugged.
    pub fn count(self) -> u8 {
        self.0.count
    }

    /// Get the raw bits representing the current button states.
    pub fn bits(self) -> u32 {
        self.0.buttons
    }

    /// Get the set of buttons that are currently pressed,
    /// but were not pressed when `other` was read.
    pub fn pressed_since(self, other: Self) -> Self {
        Self(HAL_JoystickButtons {
            count: self.0.count,
            buttons: self.0.buttons & !other.0.buttons,
        })
    }

    /// Get the set of buttons that are currently unpressed,
    /// but were pressed when `other` was read.
    pub fn released_since(self, other: Self) -> Self {
        Self(HAL_JoystickButtons {
            count: self.0.count,
            buttons: !self.0.buttons & other.0.buttons,
        })
    }
}
impl From<HAL_JoystickButtons> for Buttons {
    #[inline]
    fn from(buttons: HAL_JoystickButtons) -> Self {
        Self(buttons)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Axes(HAL_JoystickAxes);
impl Axes {
    /// Get the current axes values of the specified HID.
    #[inline]
    pub fn of(port: Port) -> Self {
        Self(axes(port))
    }

    /// Get the value of the given axis.
    ///
    /// Returns None if the axis doesn't exist.
    /// This may mean the controller is unplugged.
    pub fn get(&self, axis: Axis) -> Option<f32> {
        if axis.0 > self.0.count as usize {
            None
        } else {
            Some(self.0.axes[axis.0])
        }
    }

    /// Equivalent to `get(Axis::X)`.
    pub fn x(&self) -> Option<f32> {
        self.get(Axis::X)
    }

    /// Equivalent to `get(JoystickAxis::Y)`.
    pub fn y(&self) -> Option<f32> {
        self.get(Axis::Y)
    }

    /// Get the number of axes read.
    /// Returns 0 if the controller is unplugged.
    pub fn count(&self) -> usize {
        self.0.count as usize
    }

    /// Get all the axes read.
    pub fn all(&self) -> &[f32] {
        &self.0.axes[..self.0.count as usize]
    }
}
impl From<HAL_JoystickAxes> for Axes {
    #[inline]
    fn from(axes: HAL_JoystickAxes) -> Self {
        Self(axes)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Povs(HAL_JoystickPOVs);
impl Povs {
    /// Get the current POV hat state of the specified HID.
    #[inline]
    pub fn of(port: Port) -> Self {
        Self(povs(port))
    }

    /// Get the value of the given POV hat.
    ///
    /// Returns None if the POV hat doesn't exist.
    /// This may mean the controller is unplugged.
    pub fn get(&self, pov: Pov) -> Option<i16> {
        if pov.0 > self.0.count as usize {
            None
        } else {
            Some(self.0.povs[pov.0])
        }
    }

    /// Get the value of POV hat 0, or `None` if there are no POV hats.
    pub fn first(&self) -> Option<i16> {
        self.get(Pov(0))
    }

    /// Get the number of POV hats read.
    /// Returns 0 if the controller is unplugged.
    pub fn count(&self) -> usize {
        self.0.count as usize
    }

    /// Get all the axes read.
    pub fn all(&self) -> &[i16] {
        &self.0.povs[..self.0.count as usize]
    }
}
impl From<HAL_JoystickPOVs> for Povs {
    #[inline]
    fn from(povs: HAL_JoystickPOVs) -> Self {
        Self(povs)
    }
}

pub(crate) fn buttons(port: Port) -> HAL_JoystickButtons {
    let mut buttons: HAL_JoystickButtons = Default::default();
    unsafe { HAL_GetJoystickButtons(port.0, &mut buttons) };
    buttons
}

pub(crate) fn axes(port: Port) -> HAL_JoystickAxes {
    let mut axes: HAL_JoystickAxes = Default::default();
    unsafe { HAL_GetJoystickAxes(port.0, &mut axes) };
    axes
}

pub(crate) fn povs(port: Port) -> HAL_JoystickPOVs {
    let mut povs: HAL_JoystickPOVs = Default::default();
    unsafe { HAL_GetJoystickPOVs(port.0, &mut povs) };
    povs
}
