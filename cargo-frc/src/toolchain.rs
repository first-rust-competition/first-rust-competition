#[cfg(target_os = "linux")]
mod os {
    use std::path::PathBuf;

    pub fn install_dir() -> PathBuf {
        dirs::home_dir()
            .expect("Can't determine home directory")
            .join("wpilib")
        //.ok_or("Can't determine home directory".to_owned())
    }

    pub const TOOLCHAIN_URL_2019: &str = "https://github.com/wpilibsuite/toolchain-builder/releases/download/v2019-3/FRC-2019-Linux-Toolchain-6.3.0.tar.gz";
    pub const TOOLCHAIN_URL_2020: &str = "https://github.com/wpilibsuite/roborio-toolchain/releases/download/v2020-2/FRC-2020-Linux-Toolchain-7.3.0.tar.gz";
}

#[cfg(target_os = "macos")]
mod os {
    use std::path::PathBuf;

    pub fn install_dir() -> PathBuf {
        dirs::home_dir()
            .expect("Can't determine home directory")
            .join("wpilib")
        //.ok_or("Can't determine home directory".to_owned())
    }

    const TOOLCHAIN_URL_2019: &str = "https://github.com/wpilibsuite/toolchain-builder/releases/download/v2019-3/FRC-2019-Mac-Toolchain-6.3.0.tar.gz";
    const TOOLCHAIN_URL_2020: &str = "https://github.com/wpilibsuite/roborio-toolchain/releases/download/v2020-2/FRC-2020-Mac-Toolchain-7.3.0.tar.gz";
}

//#[cfg(target_os = "windows")]
//mod os {
//    const TOOLCHAIN_URL_2019: &str = "https://github.com/wpilibsuite/toolchain-builder/releases/download/v2019-3/FRC-2019-Windows-Toolchain-6.3.0.zip";
//    const TOOLCHAIN_URL_2020: &str = "https://github.com/wpilibsuite/roborio-toolchain/releases/download/v2020-2/FRC-2020-Windows-Toolchain-7.3.0.zip";
//}

#[cfg(unix)]
pub fn install(toolchain: Toolchain) -> Result<(), String> {
    if toolchain.installed() {
        info!("Requested toolchain appears to already be installed");
        return Err("Requested toolchain appears to already be installed".to_owned());
    }

    fs::create_dir_all(toolchain.path())
        .map_err(str_map("Could not create toolchain install directory"))?;

    if !Command::new("sh")
        .arg("-c")
        .arg(format!(
            "wget -c {} -O - | tar -xz -C {} --strip-components=1",
            toolchain.url(),
            toolchain.path().to_str().unwrap()
        ))
        .status()
        .map_err(str_map("Failed to install toolchain"))?
        .success()
    {
        return Err("Download and unarchive failed".to_owned());
    }

    Ok(())
}

pub use os::install_dir;

use crate::util::str_map;
use clap::ArgMatches;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub enum Toolchain {
    Y2019,
    Y2020,
}

impl Toolchain {
    pub fn url(&self) -> &'static str {
        match self {
            Toolchain::Y2020 => os::TOOLCHAIN_URL_2020,
            Toolchain::Y2019 => os::TOOLCHAIN_URL_2019,
        }
    }

    pub fn year(&self) -> &'static str {
        match self {
            Toolchain::Y2019 => "2019",
            Toolchain::Y2020 => "2020",
        }
    }

    pub fn path(&self) -> PathBuf {
        os::install_dir().join(self.year())
    }

    pub fn installed(&self) -> bool {
        self.path().exists()
    }

    pub fn linker(&self) -> PathBuf {
        self.path().join(format!(
            "roborio/bin/arm-frc{}-linux-gnueabi-gcc",
            self.year()
        ))
    }

    pub fn from_year(year: &str) -> Option<Self> {
        match year {
            "2020" => Some(Toolchain::Y2020),
            "2019" => Some(Toolchain::Y2019),
            _ => None,
        }
    }
}

impl Default for Toolchain {
    fn default() -> Self {
        Toolchain::Y2020
    }
}

pub fn handle_cmd(matches: &ArgMatches) -> Result<(), String> {
    match matches.subcommand_name() {
        Some("install") => install_command(matches.subcommand_matches("install").unwrap()),
        _ => unimplemented!(),
    }
}

fn install_command(matches: &ArgMatches) -> Result<(), String> {
    if let Some(toolchain) = matches.value_of("YEAR").and_then(Toolchain::from_year) {
        install(toolchain)
    } else {
        Err("Invalid toolchain year specified".to_owned())
    }
}
