use crate::util::crates::{installed, CrateData};
use colored::Colorize;

pub fn list() -> anyhow::Result<()> {
    let crates = installed()?;

    print_crates(crates);

    Ok(())
}

pub fn print_crates(crates: Vec<CrateData>) {
    let name_length = crates.iter().map(|data| data.name.len()).max().unwrap();
    let version_length = crates.iter().map(|data| data.version.len()).max().unwrap();

    println!(
        "{:name_length$} {:version_length$}",
        "Name".bold(),
        "Version".bold(),
        name_length = name_length,
        version_length = version_length
    );

    for data in crates {
        println!(
            "{:name_length$} {:version_length$}",
            data.name,
            data.version,
            name_length = name_length,
            version_length = version_length
        )
    }
}
