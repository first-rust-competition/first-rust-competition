// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use bindgen;
use std::env;
use std::path::PathBuf;

fn output_dir() -> PathBuf {
    wpilib_sys_dir().join("src")
}

fn wpilib_sys_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../wpilib-sys")
}

fn generate_bindings() {
    const INCLUDE_DIR: &str = "include";
    const SYMBOL_REGEX: &str = "HAL_[A-Za-z0-9]+";
    let bindings = bindgen::Builder::default()
        .derive_default(true)
        .rustfmt_bindings(false)
        .header(format!(
            "{}",
            wpilib_sys_dir().join("HAL_Wrapper.h").display()
        ))
        .whitelist_type(SYMBOL_REGEX)
        .whitelist_function(SYMBOL_REGEX)
        .whitelist_var(SYMBOL_REGEX)
        // usage reporting enums
        .whitelist_type(".*tInstances")
        .whitelist_type(".*tResourceType")
        .constified_enum_module("*")
        .clang_arg(format!(
            "-I{}",
            wpilib_sys_dir().join(INCLUDE_DIR).display()
        ))
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-nostdinc")
        .clang_arg("-nostdinc++")
        .clang_arg("-std=c++14");
    println!("builder_args: {:?}", bindings.command_line_flags());
    let out = bindings.generate().expect("Unable to generate bindings");

    out.write_to_file(output_dir().join("hal_bindings.rs"))
        .expect("Couldn't write bindings!");

    println!();
}

fn main() {
    generate_bindings();
}
