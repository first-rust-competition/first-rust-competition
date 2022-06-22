use color_eyre::eyre::Result;
use std::io::Write;
use std::path::Path;
use tracing::info;
use xshell::{cmd, Shell};

/// Initialize the workspace.
pub(crate) fn init(directory: &Option<String>) -> Result<()> {
    let sh = Shell::new()?;

    // Establish a temporary directory that we can work in, but keep a handle to the current one.
    let mut target_dir = std::env::current_dir()?;
    target_dir.push("crates");
    target_dir.push("wpilib-sys");

    let tempdir = tempdir::TempDir::new("wpilib-rs")?;
    let tempdir_path = tempdir.path();
    let (tmp_dir, needs_initialization) = directory
        .as_ref()
        .map_or((tempdir_path, true), |x| (Path::new(x.as_str()), false));
    sh.change_dir(&tmp_dir);

    let new_dir = tmp_dir.join("allwpilib");
    let tmp_dir = if needs_initialization {
        // Clone wpilib and enter the directory.
        info!("Cloning wpilib into {tmp_dir:?}...");
        cmd!(
        sh,
        "git clone --quiet --depth 1 --branch v2022.4.1 https://github.com/wpilibsuite/allwpilib"
    )
        .ignore_stdout()
        .ignore_stderr()
        .run()?;
        sh.change_dir("allwpilib");

        new_dir.as_path()
    } else {
        tmp_dir
    };

    // Run Gradle to generate the necessary files.
    info!("Installing the toolchain...");
    cmd!(sh, "./gradlew installRoboRioToolchain --build-cache")
        .ignore_stdout()
        .run()?;

    info!("Building the shared library...");
    cmd!(sh, "./gradlew :hal:build --build-cache")
        .ignore_stdout()
        .run()?;

    let target_dir_displayed = target_dir.display();
    let message = format!(
        "pub static WPILIB_COMMIT_HASH: &str = \"{}\";",
        cmd!(sh, "git ls-files -s ./ | cut -d ' ' -f 2").read()?
    );
    let mut file = std::fs::File::create(format!("{target_dir_displayed}/src/version.rs"))?;
    file.write_all(message.as_bytes())?;

    let include_dir = format!("{target_dir_displayed}/include/");
    fs_extra::dir::create(&include_dir, true)?;
    let copy_options = fs_extra::dir::CopyOptions::new();
    let tmp_dir_displayed = tmp_dir.to_str().unwrap();

    fs_extra::dir::copy(
        format!("{tmp_dir_displayed}/hal/src/main/native/include/hal/"),
        &include_dir,
        &copy_options,
    )?;

    fs_extra::dir::copy(
        format!("{tmp_dir_displayed}/wpiutil/src/main/native/include/wpi/"),
        &include_dir,
        &copy_options,
    )?;

    fs_extra::file::copy(
        format!("{tmp_dir_displayed}/hal/build/generated/headers/hal/FRCUsageReporting.h"),
        format!("{include_dir}/hal/FRCUsageReporting.h"),
        &fs_extra::file::CopyOptions::default(),
    )?;

    Ok(())
}
