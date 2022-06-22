use bindgen::callbacks::IntKind;
use std::env;
use std::path::PathBuf;

fn output_dir() -> PathBuf {
    wpilib_sys_dir().join("src")
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

    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        match name {
            "HAL_kInvalidHandle" => Some(IntKind::I32),
            "HAL_kMaxJoystickAxes" | "HAL_kMaxJoystickPOVs" | "HAL_kMaxJoysticks" => {
                Some(IntKind::U8)
            }
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
        .allowlist_type(SYMBOL_REGEX)
        .allowlist_function(SYMBOL_REGEX)
        .allowlist_var(SYMBOL_REGEX)
        .allowlist_type("HALUsageReporting::.*")
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .parse_callbacks(Box::new(BindgenCallbacks))
        .clang_arg(format!(
            "-I{}",
            wpilib_sys_dir().join(INCLUDE_DIR).display()
        ))
        .clang_arg("-xc++")
        .clang_arg("-nostdinc")
        .clang_arg("-nostdinc++")
        .clang_arg("-std=c++17");
    println!("builder_args: {:?}", bindings.command_line_flags());
    let out = bindings.generate().expect("Unable to generate bindings");

    out.write_to_file(output_dir().join("hal_bindings.rs"))
        .expect("Couldn't write bindings!");

    println!();
}

fn main() {
    generate_bindings();
}
