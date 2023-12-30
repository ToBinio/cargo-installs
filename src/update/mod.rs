use crate::util::crates::{get_installed, CrateData};
use crate::util::table::get_column_width;
use colored::{ColoredString, Colorize};
use fancy_duration::AsFancyDuration;
use std::fmt::Debug;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

pub fn update() -> anyhow::Result<()> {
    let crates: Vec<CrateData> = get_installed()?
        .into_iter()
        .filter(|data| !data.is_latest_version())
        .collect();

    let mut installs = vec![];

    for data in crates {
        let now = Instant::now();

        let mut child = Command::new("cargo")
            .arg("install")
            .arg(&data.name)
            .spawn()?;

        let status = child.wait()?;

        installs.push(InstallResult {
            name: data.name.to_string(),
            prev_version: data.version,
            new_version: if status.success() {
                data.latest_version.normal()
            } else {
                "failed".red()
            },
            time: now.elapsed(),
        })
    }

    print_results(&installs);
    Ok(())
}

struct InstallResult {
    name: String,
    prev_version: String,
    new_version: ColoredString,
    time: Duration,
}

fn print_results(results: &Vec<InstallResult>) {
    let name_length = get_column_width("Name", &results, |data| data.name.len());
    let prev_version_length = get_column_width("Before", &results, |data| data.prev_version.len());
    let new_version_length = get_column_width("Now", &results, |data| data.new_version.len());
    let time_length = get_column_width("Time", &results, |data| {
        data.time.fancy_duration().format_compact().len()
    });

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
            result.time.fancy_duration().format()
        )
    }
}
