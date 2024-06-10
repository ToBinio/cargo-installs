use colored::Colorize;
use iter_tools::Itertools;

pub fn mark_version_different(current: &str, new: &str) -> String {
    let current = current.split(".");
    let new = new.split(".");

    let mut has_different = false;

    current
        .zip(new)
        .map(|(current, new)| {
            if current != new && !has_different {
                has_different = true;
                new.red()
            } else {
                new.normal()
            }
        })
        .join(".")
}
