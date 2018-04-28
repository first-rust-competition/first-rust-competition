extern crate bindgen;
use std::env;

const BINDINGS_FILE_NAME: &str = "hal.rs";

fn main() {
    for lib in [
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
    ].iter()
    {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }

    let path = env::current_dir().unwrap();
    println!("cargo:rustc-link-search=native={}/HAL/lib", path.display());

    let bindings = bindgen::Builder::default()
        .header("HAL/include/HAL/HAL.h")
        .clang_arg("-I./HAL/include")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(dead_code)] // prune once the lib is in a good state")
        .raw_line("#![allow(non_upper_case_globals)]")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = env::current_dir().unwrap().join("src");
    bindings
        .write_to_file(out_path.join(BINDINGS_FILE_NAME))
        .expect("Couldn't write bindings!");

}
