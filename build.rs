use std::env;

fn main() {
    extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    for lib in ["HALAthena",
                "wpiutil",
                "FRC_NetworkCommunication",
                "RoboRIO_FRC_ChipObject",
                "NiFpga",
                "NiFpgaLv",
                "niriosession",
                "spi",
                "i2c",
                "visa",
                "NiRioSrv",
                "niriodevenum"]
        .iter() {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }

    let path = env::current_dir().unwrap();

    println!("cargo:rustc-link-search=native={}/allwpilib/ni-libraries", path.display());
    println!("cargo:rustc-link-search=native={}/allwpilib/hal/src/main/native/", path.display());

        // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("wpilib_hal_ffi.rs"))
        .expect("Couldn't write bindings!");
}
