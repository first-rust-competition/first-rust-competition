use color_eyre::eyre::Result;
use fs_extra::{dir, error, file};
use std::io::Write;
use std::path::PathBuf;
use tracing::{error, info};
use xshell::{cmd, Shell};

/// Initialize the workspace.
pub(crate) fn init() -> Result<()> {
    let sh = Shell::new()?;

    let xtask_directory = PathBuf::from(format!("{}/target/xtask/", env!("CARGO_WORKSPACE_DIR")));
    let wpilib_directory =
        PathBuf::from(format!("{}/allwpilib", xtask_directory.to_str().unwrap()));
    let crate_directory = PathBuf::from(format!(
        "{}/crates/wpilib-sys/",
        env!("CARGO_WORKSPACE_DIR")
    ));
    let include_directory = PathBuf::from(format!("{}/include", crate_directory.to_str().unwrap()));

    // Create the above directories not exists.
    for directory in &[&xtask_directory, &include_directory] {
        match dir::create(directory, false) {
            Ok(_) => (),
            Err(e) => match e.kind {
                error::ErrorKind::AlreadyExists => (),
                _ => error!("Failure in creating {:?}: {:?}", directory.file_name(), e),
            },
        };

        assert!(directory.exists())
    }

    if !wpilib_directory.exists() {
        info!("Downloading WPILib...");
        let wpilib_directory = wpilib_directory.to_str().unwrap();
        cmd!(
            sh,
            "git clone --quiet --depth 1 --branch v2022.4.1 https://github.com/wpilibsuite/allwpilib {wpilib_directory}"
        )
        .ignore_stdout()
        .ignore_stderr()
        .run()?;
    }
    sh.change_dir(&wpilib_directory);

    // Run Gradle to generate the necessary files.
    info!("Installing the toolchain...");
    cmd!(sh, "./gradlew installRoboRioToolchain --build-cache").run()?;

    info!("Building the shared library...");
    cmd!(sh, "./gradlew :hal:build --build-cache").run()?;

    let message = format!(
        "pub static WPILIB_COMMIT_HASH: &str = \"{}\";",
        cmd!(sh, "git ls-files -s ./ | cut -d ' ' -f 2").read()?
    );
    let mut file = std::fs::File::create(format!(
        "{}/src/version.rs",
        crate_directory.to_str().unwrap()
    ))?;
    file.write_all(message.as_bytes())?;

    let copied_directories = [
        "hal/src/main/native/include/hal/",
        "wpiutil/src/main/native/include/wpi/",
    ];

    for directory in copied_directories {
        let res = fs_extra::dir::copy(
            format!("{}/{directory}", &wpilib_directory.to_str().unwrap()),
            &include_directory,
            &dir::CopyOptions::default(),
        );

        match res {
            Ok(_) => (),
            Err(e) => match e.kind {
                error::ErrorKind::AlreadyExists => (),
                _ => error!("{:?}", e),
            },
        }
    }

    match file::copy(
        format!(
            "{}/hal/build/generated/headers/hal/FRCUsageReporting.h",
            wpilib_directory.to_str().unwrap()
        ),
        format!(
            "{}/hal/FRCUsageReporting.h",
            include_directory.to_str().unwrap()
        ),
        &file::CopyOptions::default(),
    ) {
        Ok(_) => (),
        Err(e) => match e.kind {
            error::ErrorKind::AlreadyExists => (),
            _ => error!("{:?}", e),
        },
    }

    // info!("Copying the NI libraries over...");
    // fs_extra::dir::copy(
    //     format!("{tmp_dir_displayed}/ni-libraries/"),
    //     format!("{include_dir}/../../../target/ni-libraries/"),
    //     &copy_options,
    // )?;

    info!("Generating bindings...");
    bindgen::generate_bindings();

    Ok(())
}

mod bindgen {

    use bindgen::callbacks::IntKind;
    use std::env;
    use std::path::PathBuf;

    fn wpilib_sys_dir() -> PathBuf {
        PathBuf::from(env::var("CARGO_WORKSPACE_DIR").unwrap())
            .join("crates")
            .join("wpilib-sys")
    }

    fn output_dir() -> PathBuf {
        wpilib_sys_dir().join("src")
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

    pub fn generate_bindings() {
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
}
