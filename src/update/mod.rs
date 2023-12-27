use crate::util::crates::{get_installed, CrateData};
use std::process::{Command, Stdio};

pub fn update() -> anyhow::Result<()> {
    let crates: Vec<CrateData> = get_installed()?
        .into_iter()
        .filter(|data| !data.is_latest_version())
        .collect();

    for data in crates {
        let mut child = Command::new("cargo")
            .arg("install")
            .arg(data.name)
            .spawn()?;

        child.wait()?;
    }

    Ok(())
}
