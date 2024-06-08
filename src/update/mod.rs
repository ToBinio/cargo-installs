use crate::util::crates::{get_installed, CrateData, Origen};
use crate::util::settings::settings;
use crate::util::table::get_column_width;
use colored::{ColoredString, Colorize};
use fancy_duration::AsFancyDuration;
use std::process::Command;
use std::time::Instant;

pub fn update() -> anyhow::Result<()> {
    let settings = settings()?;

    let crates: Vec<CrateData> = get_installed()?
        .into_iter()
        .filter(|data| !data.is_latest_version())
        .collect();

    let mut installs = vec![];

    for data in crates {
        if settings.blacklist.contains(&data.name) {
            installs.push(InstallResult {
                name: data.name.to_string(),
                prev_version: data.version,
                new_version: "blacklisted".normal(),
                time: "-".to_string(),
            });

            continue;
        }

        if let Origen::Local = data.origen {
            installs.push(InstallResult {
                name: data.name.to_string(),
                prev_version: data.version,
                new_version: "local".normal(),
                time: "-".to_string(),
            });

            continue;
        }

        let now = Instant::now();

        let mut child = Command::new("cargo")
            .arg("install")
            .arg(&data.name)
            .spawn()?;

        let status = child.wait()?;

        let new_version = match data.origen {
            Origen::Local => {
                unreachable!()
            }
            Origen::Remote { latest_version } => {
                if status.success() {
                    latest_version.normal()
                } else {
                    "failed".red()
                }
            }
        };

        installs.push(InstallResult {
            name: data.name.to_string(),
            prev_version: data.version,
            new_version,
            time: now.elapsed().fancy_duration().truncate(2).format(),
        })
    }

    print_results(&installs);
    Ok(())
}

struct InstallResult {
    name: String,
    prev_version: String,
    new_version: ColoredString,
    time: String,
}

fn print_results(results: &Vec<InstallResult>) {
    let name_length = get_column_width("Name", results, |data| data.name.len());
    let prev_version_length = get_column_width("Before", results, |data| data.prev_version.len());
    let new_version_length = get_column_width("Now", results, |data| data.new_version.len());
    let time_length = get_column_width("Time", results, |data| data.time.len());

    println!(
        "{:name_length$} {:prev_version_length$} {:new_version_length$} {:time_length$}",
        "Name".bold(),
        "Before".bold(),
        "Now".bold(),
        "Time".bold()
    );

    for result in results {
        println!(
            "{:name_length$} {:prev_version_length$} {:new_version_length$} {:time_length$}",
            result.name,
            result.prev_version,
            result.new_version,
            //todo
            result.time
        )
    }
}
