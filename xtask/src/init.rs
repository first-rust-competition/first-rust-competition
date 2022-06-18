use color_eyre::eyre::Result;
use std::io::Write;
use tracing::{info, trace, warn};
use xshell::{cmd, Shell};

/// Initialize the workspace.
pub(crate) fn init() -> Result<()> {
    let sh = Shell::new()?;

    // Establish a temporary directory that we can work in, but keep a handle to the current one.
    let mut target_dir = std::env::current_dir()?;
    target_dir.push("wpilib-sys");

    let tmp_dir = tempdir::TempDir::new("wpilib-rs")?;
    sh.change_dir(&tmp_dir);

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

    // Run Gradle to generate the necessary files.
    info!("Installing the toolchain...");
    cmd!(sh, "./gradlew installRoboRioToolchain --build-cache")
        .ignore_stdout()
        .run()?;

    info!("Building the shared library...");
    cmd!(
        sh,
        "./gradlew halLinuxathenaReleaseSharedLibrary --build-cache"
    )
    .ignore_stdout()
    .run()?;

    /*
    let wpilib_path = tmp_dir.path().as_os_str().to_str().unwrap().to_owned();

    let matches = [
    ];

    for m in matches {
        trace!("Scanning for match {m}.");
        for entry in glob(&m)? {
            info!("Found an entry for a match!");
            match entry {
                Ok(target) => cmd!(sh, "cp -R ./{target} {include_directory}")
                    .ignore_stdout()
                    .run()?,
                Err(e) => warn!("Encountered some error while searching for header files: {e}"),
            }
        }
    } */

    let target_dir_displayed = target_dir.display();
    let message = format!(
        "pub static WPILIB_COMMIT_HASH: &str = \"{}\";",
        cmd!(sh, "git ls-files -s ./ | cut -d ' ' -f 2").read()?
    );
    let mut file = std::fs::File::create(format!("{target_dir_displayed}/src/version.rs"))?;
    file.write_all(message.as_bytes())?;

    tmp_dir.close()?;
    Ok(())
}
