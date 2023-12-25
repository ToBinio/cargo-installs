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
        .map(|data| data.newest_version.len())
        .max()
        .unwrap()
        .max("Newest".len());
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
        "Newest".bold(),
    );

    for data in crates {
        println!(
            "{:name_length$} {:version_length$} {:newest_version_length$}",
            data.name, data.version, data.newest_version
        )
    }
}
