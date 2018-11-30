// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(intra_doc_link_resolution_failure)]
#![allow(clippy::useless_transmute)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::trivially_copy_pass_by_ref)]

use std::ffi;
use std::fmt;

include!("./hal_bindings.rs");

impl fmt::Debug for HAL_JoystickDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name: Vec<_> = self.name.iter().map(|item| *item as u8).collect();
        let name = &ffi::CString::new(name);
        f.debug_struct("HAL_JoystickDescriptor")
            .field("name", name)
            .field("isXbox", &self.isXbox)
            .field("axisCount", &self.axisCount)
            .field("buttonCount", &self.buttonCount)
            .field("povCount", &self.povCount)
            .finish()
    }
}
