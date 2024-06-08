use crate::util::crates::Origen::{Local, Remote};
use crate::util::crates::{get_installed, CrateData, Origen};
use crate::util::table::get_column_width;
use colored::{ColoredString, Colorize};
use iter_tools::Itertools;

pub fn list() -> anyhow::Result<()> {
    let crates = get_installed()?;

    print_crates(crates);

    Ok(())
}

pub fn print_crates(crates: Vec<CrateData>) {
    let name_length = get_column_width("Name", &crates, |data| data.name.len());
    let version_length = get_column_width("Version", &crates, |data| data.version.len());
    let newest_version_length = get_column_width("Latest", &crates, |data| {
        get_latest_version_text(data).len()
    });

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
            let latest_version = get_latest_version_text(data);

            println!(
                "{:name_length$} {:version_length$} {:newest_version_length$}",
                data.name, data.version, latest_version
            )
        });
}

pub fn get_latest_version_text(crate_data: &CrateData) -> ColoredString {
    match &crate_data.origen {
        //todo - gray instead of green...
        Local => "local".green(),
        Remote { latest_version } => {
            if crate_data.is_latest_version() {
                latest_version.normal()
            } else {
                latest_version.red()
            }
        }
    }
}
