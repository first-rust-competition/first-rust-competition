// This file is part of "first-rust-competition", which is free software: you
// can redistribute it and/or modify it under the terms of the GNU General
// Public License version 3 as published by the Free Software Foundation. See
// <https://www.gnu.org/licenses/> for a copy.

extern crate bindgen;

// extern crate fs_extra;
// use fs_extra::dir::*;
use std::env;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_dir;
use std::path::*;

fn output() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

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

const LIB_DIR: &'static str = "HAL/lib";
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

/// Code-generation for the HAL
fn generate_bindings() {
    const INCLUDE_DIR: &'static str = "HAL/include";
    const SYMBOL_REGEX: &'static str = "HAL_[A-Za-z0-9]+";
    // Not needed due to `always-run()`
    // println!("cargo:rerun-if-changed={}/*", INCLUDE_DIR);
    let bindings = bindgen::Builder::default()
        .derive_default(true)

        .rustfmt_bindings(false)
        .header(format!("{}{}", INCLUDE_DIR, "/hal/HAL.h"))
        .whitelist_type(SYMBOL_REGEX)
        .whitelist_function(SYMBOL_REGEX)
        .whitelist_var(SYMBOL_REGEX)
        // usage reporting enums
        .whitelist_type(".*tInstances")
        .whitelist_type(".*tResourceType")
        .clang_arg(format!("-I{}", INCLUDE_DIR))
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-nostdinc")
        .clang_arg("-nostdinc++")
        .clang_arg("-std=c++14");
    println!("builder_args: {:?}", bindings.command_line_flags());
    let out = bindings.generate().expect("Unable to generate bindings");

    out.write_to_file(output().join("hal_bindings.rs"))
        .expect("Couldn't write bindings!");

    // write the bindings to a file for viewing
    #[cfg(feature = "dev")]
    {
        let dev_dir = env::current_dir().unwrap();
        out.write_to_file(dev_dir.join("HAL_bindings_temp.rs"))
            .expect("Couldn't write bindings to temporary file!");
    }
}

fn main() {
    always_run();
    announce_lib();
    generate_bindings();
    link();
}
