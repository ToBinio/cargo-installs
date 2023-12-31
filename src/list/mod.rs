use crate::util::crates::{get_installed, CrateData};
use crate::util::table::get_column_width;
use colored::Colorize;
use iter_tools::Itertools;

pub fn list() -> anyhow::Result<()> {
    let crates = get_installed()?;

    print_crates(crates);

    Ok(())
}

pub fn print_crates(crates: Vec<CrateData>) {
    let name_length = get_column_width("Name", &crates, |data| data.name.len());
    let version_length = get_column_width("Version", &crates, |data| data.version.len());
    let newest_version_length =
        get_column_width("Latest", &crates, |data| data.latest_version.len());

    println!(
        "{:name_length$} {:version_length$} {:newest_version_length$}",
        "Name".bold(),
        "Version".bold(),
        "Latest".bold(),
    );

    crates
        .iter()
        .sorted_by(|a, b| a.name.cmp(&b.name))
        .sorted_by(|a, b| a.is_latest_version().cmp(&b.is_latest_version()))
        .for_each(|data| {
            let latest_version = if data.is_latest_version() {
                data.latest_version.normal()
            } else {
                data.latest_version.red()
            };

            println!(
                "{:name_length$} {:version_length$} {:newest_version_length$}",
                data.name, data.version, latest_version
            )
        });
}
