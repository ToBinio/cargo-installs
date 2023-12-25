use crate::util::crates::{get_installed, CrateData};
use colored::Colorize;

pub fn list() -> anyhow::Result<()> {
    let crates = get_installed()?;

    print_crates(crates);

    Ok(())
}

pub fn print_crates(crates: Vec<CrateData>) {
    let name_length = crates
        .iter()
        .map(|data| data.name.len())
        .max()
        .unwrap()
        .max("Name".len());
    let newest_version_length = crates
        .iter()
        .map(|data| data.latest_version.len())
        .max()
        .unwrap()
        .max("Latest".len());
    let version_length = crates
        .iter()
        .map(|data| data.version.len())
        .max()
        .unwrap()
        .max("Version".len());

    println!(
        "{:name_length$} {:version_length$} {:newest_version_length$}",
        "Name".bold(),
        "Version".bold(),
        "Latest".bold(),
    );

    for data in crates {
        let latest_version = if data.is_latest_version() {
            data.latest_version.normal()
        } else {
            data.latest_version.red()
        };

        println!(
            "{:name_length$} {:version_length$} {:newest_version_length$}",
            data.name, data.version, latest_version
        )
    }
}
