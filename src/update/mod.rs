use crate::util::crates::{get_installed, CrateData};
use crate::util::table::get_column_width;
use color_eyre::eyre;
use colored::Colorize;
use console::Term;
use fancy_duration::AsFancyDuration;
use std::io::Write;
use std::ops::Not;
use std::process::{Child, Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub fn update() -> eyre::Result<()> {
    let installed = get_installed()?;

    let has_binstall = installed.iter().any(|data| data.name == "cargo-binstall");

    let crates: Vec<CrateData> = installed
        .into_iter()
        .filter(|data| data.is_latest_version().not())
        .collect();

    //todo - better  term handling (no arc&mutex)
    let term = Arc::new(Mutex::new(Term::buffered_stdout()));
    term.lock().unwrap().hide_cursor()?;

    print_start(&crates, &mut term.lock().unwrap())?;
    term.lock().unwrap().flush()?;

    for data in crates {
        let now = Instant::now();

        let mut child = update_crate(&data, has_binstall)?;

        let thread_lock = term.clone();
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut last_time_size = 0;
            while rx.try_recv().is_err() {
                let time = now.elapsed().fancy_duration().truncate(2).format().normal();
                let mut thread_term = thread_lock.lock().unwrap();

                thread_term.move_cursor_right(last_time_size)?;
                thread_term.clear_chars(last_time_size)?;
                last_time_size = time.len();

                write!(thread_term, "{}", time)?;
                thread_term.move_cursor_left(time.len())?;
                thread_term.flush()?;

                thread::sleep(Duration::from_millis(100));
            }
            Ok::<_, eyre::Error>(())
        });

        let status = child.wait()?;
        tx.send(())?;

        let mut thread_term = term.lock().unwrap();

        if status.success().not() {
            write!(thread_term, "{}", "failed".red())?;
            thread_term.move_cursor_left("failed".len())?;
        }

        thread_term.move_cursor_down(1)?;
        thread_term.flush()?;
    }

    term.lock().unwrap().show_cursor()?;
    term.lock().unwrap().flush()?;

    Ok(())
}

fn update_crate(data: &CrateData, has_binstall: bool) -> eyre::Result<Child> {
    let mut command = Command::new("cargo");

    if has_binstall {
        command.arg("binstall").arg(&data.name).arg("-y");
    } else {
        command.arg("install").arg(&data.name);
    }

    let child = command
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    Ok(child)
}

/// returns the inset of where the time should begin to be rendered
///
/// moves the terminal to where the first time should be written
fn print_start(crates: &Vec<CrateData>, term: &mut Term) -> eyre::Result<()> {
    let name_length = get_column_width("Name", crates, |data| data.name.len());
    let prev_version_length = get_column_width("Before", crates, |data| data.version.len());
    let new_version_length = get_column_width("Now", crates, |data| {
        data.latest_version()
            .expect("local version never get updated")
            .len()
    });
    let time_length = "Time".len();

    writeln!(
        term,
        "{:name_length$} {:prev_version_length$} {:new_version_length$} {:time_length$}",
        "Name".bold(),
        "Before".bold(),
        "Now".bold(),
        "Time".bold()
    )?;

    for data in crates {
        writeln!(
            term,
            "{:name_length$} {:prev_version_length$} {:new_version_length$}",
            data.name,
            data.version,
            data.latest_version()
                .expect("local version never get updated"),
        )?;
    }

    //move to first line
    term.move_cursor_up(crates.len())?;
    term.move_cursor_right(name_length + prev_version_length + new_version_length + 3)?;

    Ok(())
}
