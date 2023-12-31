use crate::cli::BlacklistArgs;
use crate::util::settings::{save_settings, settings};

pub fn black_list(mut blacklist_args: BlacklistArgs) -> anyhow::Result<()> {
    let mut settings = settings()?;

    if blacklist_args.remove {
        settings
            .blacklist
            .retain(|element| !blacklist_args.crates.contains(element))
    } else {
        settings.blacklist.append(&mut blacklist_args.crates);
    }

    save_settings(&settings)?;

    Ok(())
}
