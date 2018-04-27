extern crate bindgen;
use std::env;
use std::path::PathBuf;

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
                "wpiutil"
                ].iter() {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }

    let path = env::current_dir().unwrap();
    println!("cargo:rustc-link-search=native={}/HAL/lib", path.display());

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("HAL/include/HAL/HAL.h")
        .clang_arg("-I./HAL/include")
        .clang_arg("-x c++")
        .clang_arg("-std=c++11")
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
