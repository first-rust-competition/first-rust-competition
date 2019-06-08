// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use bindgen;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::PathBuf;

fn output_dir() -> PathBuf {
    wpilib_sys_dir().join("src")
}

fn hal_src_dir() -> PathBuf {
    allwpilib_dir().join("hal/src")
}

fn allwpilib_dir() -> PathBuf {
    wpilib_sys_dir().join("allwpilib")
}

fn wpilib_sys_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../wpilib-sys")
}

#[derive(Debug)]
struct BindgenCallbacks;

impl bindgen::callbacks::ParseCallbacks for BindgenCallbacks {
    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        // note that returning `None` leaves the variant name unchanged in the generated bindings
        match enum_name {
            Some("tResourceType") => {
                Some(original_variant_name["kResourceType_".len()..].to_owned())
            }
            Some(enum_name) if original_variant_name.starts_with(enum_name) => {
                Some(original_variant_name[enum_name.len() + 1..].to_owned())
            }
            _ => None,
        }
    }

    fn int_macro(&self, name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
        match name {
            "HAL_kInvalidHandle" => Some(bindgen::callbacks::IntKind::I32),
            _ => None,
        }
    }

    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if name.ends_with("_MESSAGE") {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn generate_bindings() {
    const INCLUDE_DIR: &str = "include";
    const SYMBOL_REGEX: &str = r"HAL_\w+";
    let bindings = bindgen::Builder::default()
        .derive_default(true)
        .header(format!(
            "{}",
            wpilib_sys_dir().join("HAL_Wrapper.h").display()
        ))
        .whitelist_type(SYMBOL_REGEX)
        .whitelist_function(SYMBOL_REGEX)
        .whitelist_var(SYMBOL_REGEX)
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .parse_callbacks(Box::new(BindgenCallbacks))
        .clang_arg(format!(
            "-I{}",
            wpilib_sys_dir().join(INCLUDE_DIR).display()
        ))
        .clang_arg("-xc++")
        .clang_arg("-nostdinc")
        .clang_arg("-nostdinc++")
        .clang_arg("-std=c++14");
    println!("builder_args: {:?}", bindings.command_line_flags());
    let out = bindings.generate().expect("Unable to generate bindings");

    out.write_to_file(output_dir().join("hal_bindings.rs"))
        .expect("Couldn't write bindings!");

    println!();
}

fn write_usage_const<T: Write>(
    f: &mut io::BufWriter<T>,
    name: &str,
    value: &str,
) -> io::Result<()> {
    writeln!(f, "pub const {}: Type = {};", name, value)?;
    Ok(())
}

fn generate_usage_resource_types(generate_dir: &PathBuf, usage_out_dir: &PathBuf) {
    let f =
        File::open(generate_dir.join("ResourceType.txt")).expect("Could not open resource file");
    let f = io::BufReader::new(f);

    let fout = File::create(usage_out_dir.join("resource_types.rs"))
        .expect("Could not create resource file");
    let mut fout = io::BufWriter::new(fout);

    fout.write_all(b"pub type Type = i32;\n").unwrap();

    for line in f.lines() {
        let line = line.unwrap();

        assert!(line.starts_with("kResourceType_"));
        let mut sp = line["kResourceType_".len()..].split(" = ");
        let name = sp.next().unwrap();
        let value = sp.next().expect("Expected = in resource types list");

        write_usage_const(&mut fout, name, value).unwrap();
    }
}

fn generate_usage_instances(generate_dir: &PathBuf, usage_out_dir: &PathBuf) {
    let f = File::open(generate_dir.join("Instances.txt")).expect("Could not open instances file");
    let f = io::BufReader::new(f);

    let fout =
        File::create(usage_out_dir.join("instances.rs")).expect("Could not create instances file");
    let mut fout = io::BufWriter::new(fout);

    fout.write_all(b"pub type Type = i32;\n").unwrap();

    for line in f.lines() {
        let line = line.unwrap();

        assert!(line.starts_with('k'));
        let mut sp = line.split(" = ");
        let name = sp.next().unwrap();
        let value = sp.next().expect("Expected = in instances list");

        write_usage_const(&mut fout, name, value).unwrap();
    }
}

fn generate_usage_reporting() {
    let generate_dir = hal_src_dir().join("generate");
    let usage_out_dir = output_dir().join("usage");
    println!(
        "generate: {}, usage out: {}",
        generate_dir.display(),
        usage_out_dir.display()
    );
    generate_usage_resource_types(&generate_dir, &usage_out_dir);
    generate_usage_instances(&generate_dir, &usage_out_dir);
}

fn main() {
    generate_bindings();
    generate_usage_reporting();
}
