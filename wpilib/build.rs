extern crate bindgen;
use std::env;
use std::path::*;

fn output() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

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
    // println!("{:?} {:?}", path, env::current_dir().unwrap());
    println!("cargo:rustc-link-search=native={}/HAL/lib", path.display());

    const SYMBOL_REGEX: &'static str = "HAL_[A-Za-z0-9]+";
    let bindings = bindgen::Builder::default()
        .derive_default(true)
        // .rustfmt_bindings(false)
        .header("HAL/include/HAL/HAL.h")
        .whitelist_type(SYMBOL_REGEX)
        .whitelist_function(SYMBOL_REGEX)
        .whitelist_var(SYMBOL_REGEX)
        // usage reporting enums
        .whitelist_type(".*tInstances")
        .whitelist_type(".*tResourceType")
        .clang_arg("-I./HAL/include")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++14");
    println!("builder_args: {:?}", bindings.command_line_flags());
    let out = bindings.generate().expect("Unable to generate bindings");

    out.write_to_file(output().join("hal_bindings.rs"))
        .expect("Couldn't write bindings!");
}
