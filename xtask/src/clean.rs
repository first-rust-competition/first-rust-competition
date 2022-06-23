use color_eyre::eyre::Result;
use fs_extra::dir;

pub fn clean() -> Result<()> {
    dir::remove(format!("{}/target/xtask", env!("CARGO_WORKSPACE_DIR")))?;

    Ok(())
}
