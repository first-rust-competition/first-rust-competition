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

// fn place_libs() {
//     let options = CopyOptions {
//         overwrite: true,
//         skip_exist: false,
//         buffer_size: 6400,
//         copy_inside: true,
//         depth: 100,
//     };
//     let mut out_path = PathBuf::new();
//     out_path.push(env::var("CARGO_MANIFEST_DIR").expect("Couldn't read manifest dir env var."));
//     out_path.push(".frc");
//     let mut input = env::current_dir().expect("Couldn't find current directory");
//     input.push("HAL/lib");
//     copy(input, out_path, &options).expect("Couldn't copy libs.");
// }

fn announce_lib() {
    #![allow(unreachable_code)] // compile-dependent panic for not windows or unix platform
    let mut lib_path = PathBuf::new();
    lib_path.push(env::var("CARGO_MANIFEST_DIR").expect("Couldn't read manifest dir env var."));
    lib_path.push(LIB_DIR);
    let mut symlink_path = env::temp_dir();
    let mut flag_path = symlink_path.clone();
    symlink_path.push("frc-libs");
    flag_path.push("frc-flag");
    fs::write(flag_path.clone(), b"flag").expect("Could not write to temp flag file");

    println!("cargo:rerun-if-changed={}", flag_path.display());
    fs::remove_file(symlink_path.clone()).expect("Could not remove stale symlink");
    #[cfg(unix)]
    {
        symlink(lib_path, symlink_path).expect("Could not create lib symlink");
    }
    #[cfg(windows)]
    {
        symlink_dir(lib_path, symlink_path).expect("Could not create lib symlink");
    }
    #[cfg(any(windows, unix))]
    {
        return;
    }
    panic!("Platform is not Windows or UNIX!");
}

const LIB_DIR: &'static str = "HAL/lib";

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
// If you plan to have multilple FRC projects and deploy them randomly
// (How do I know I have a symlink to the right lib version?)
// TODO(Lytigas) replace this hack with an updated index
fn always_run() {
    #[cfg(feature = "dev")]
    println!("cargo:rerun-if-changed=*");
}

fn generate_bindings() {
    const INCLUDE_DIR: &'static str = "HAL/include";
    const SYMBOL_REGEX: &'static str = "HAL_[A-Za-z0-9]+";
    // Not needed due to `always-run()`
    // println!("cargo:rerun-if-changed={}/*", INCLUDE_DIR);
    let bindings = bindgen::Builder::default()
        .derive_default(true)

        .rustfmt_bindings(false)
        .header(format!("{}{}", INCLUDE_DIR, "/HAL/HAL.h"))
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

fn main() {
    always_run();
    announce_lib();
    generate_bindings();
    link();
}
