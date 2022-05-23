// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error;
use subprocess::ExitStatus;
use subprocess::PopenError;

pub fn handle_subprocess(
    command_name: &str,
    handle: Result<ExitStatus, PopenError>,
) -> Result<(), String> {
    let exit_code = handle.map_err(|e| {
        format!(
            "Could not spawn subprocess for '{}': {}.",
            command_name,
            e.to_string()
        )
    })?;
    handle_subprocess_exit(command_name, exit_code)
}

pub fn handle_subprocess_exit(command_name: &str, exit_code: ExitStatus) -> Result<(), String> {
    match exit_code {
        ExitStatus::Exited(0) => Ok(()),
        ExitStatus::Signaled(code) => Err(format!(
            "'{}' exited from Signal or Other, code {}.",
            command_name, code
        )),
        // duplicate because above code is u8 and this one is i32
        ExitStatus::Other(code) => Err(format!(
            "'{}' exited from Signal or Other, code {}.",
            command_name, code
        )),
        _ => Err(format!("'{}' exited Undetermined.", command_name,)),
    }
}

pub fn str_map<E: Error>(prelude: &'static str) -> impl FnOnce(E) -> String {
    move |e: E| format!("{}: {}", prelude, e.to_string())
}
