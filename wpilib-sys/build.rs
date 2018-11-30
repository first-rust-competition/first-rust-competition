// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_dir;
use std::path::*;

/// announce our lib dir with a symlink cargo-frc to copy
fn announce_lib() {
    #![allow(unreachable_code)] // compile-dependent panic for not windows or unix platform
    let mut lib_path = PathBuf::new();
    lib_path.push(env::var("CARGO_MANIFEST_DIR").expect("Couldn't read manifest dir env var."));
    lib_path.push(LIB_DIR);
    let mut symlink_path = env::temp_dir();
    let mut flag_path = symlink_path.clone();
    symlink_path.push("frc-libs");
    flag_path.push("frc-flag");

    println!("cargo:rerun-if-changed={}", flag_path.display());
    fs::write(flag_path, b"flag").expect("Could not write to temp flag file");

    fs::remove_file(symlink_path.clone()).ok(); //ignore the err
    #[cfg(unix)]
    {
        symlink(lib_path, symlink_path).expect("Could not create lib symlink");
    }
    #[cfg(windows)]
    {
        symlink_dir(lib_path, symlink_path).expect("Could not create lib symlink");
    }
    #[cfg(not(any(windows, unix)))]
    {
        panic!("Platform is not Windows or UNIX!");
    }
}

const LIB_DIR: &'static str = "lib";
const LIB_LIST: &'static [&'static str] = &[
    "FRC_NetworkCommunication",
    "NiFpga",
    "NiFpgaLv",
    "niriodevenum",
    "niriosession",
    "NiRioSrv",
    "RoboRIO_FRC_ChipObject",
    "visa",
    "wpiHal",
    "wpiutil",
];

/// Tell cargo to link our libs
fn link() {
    for lib in LIB_LIST.iter() {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }

    let path = env::current_dir().unwrap();
    println!(
        "cargo:rustc-link-search=native={}/{}",
        path.display(),
        LIB_DIR
    );
}

// For some reason, this makes the build script always run on compile
// While not ideal, this is kind of necessary for cargo-frc to do its thing
// If you plan to have multiple FRC projects and deploy them randomly
// (How do I know I have a symlink to the right lib version?)
// TODO(Lytigas) replace this hack with a file cargo-frc will touch on run
fn always_run() {
    #[cfg(feature = "dev")]
    println!("cargo:rerun-if-changed=*");
}

fn main() {
    always_run();
    announce_lib();
    link();
}
