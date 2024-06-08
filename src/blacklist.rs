use color_eyre::eyre;
use crate::cli::BlacklistArgs;
use crate::util::settings::{save_settings, settings};
use colored::Colorize;

pub fn black_list(mut blacklist_args: BlacklistArgs) -> eyre::Result<()> {
    let mut settings = settings()?;

    if blacklist_args.list || blacklist_args.crates.is_empty() {
        println!("{}", "Blacklist".bold());

        for name in &settings.blacklist {
            println!("{}", name);
        }
    } else if blacklist_args.remove {
        settings
            .blacklist
            .retain(|element| !blacklist_args.crates.contains(element))
    } else {
        settings.blacklist.append(&mut blacklist_args.crates);
    }

    save_settings(&settings)?;

    Ok(())
}
