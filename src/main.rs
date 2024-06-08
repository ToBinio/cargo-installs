use crate::blacklist::black_list;
use crate::cli::{CargoCli, Commands};
use crate::list::list;
use crate::update::update;
use clap::Parser;
use color_eyre::eyre;

pub mod blacklist;
pub mod cli;
pub mod list;
pub mod update;

pub mod util;

fn main() -> eyre::Result<()> {
    let CargoCli::Installs(args) = CargoCli::parse();

    if let Some(commands) = args.command {
        match commands {
            Commands::Update => update()?,

            Commands::Blacklist(blacklist_args) => black_list(blacklist_args)?,
        }
    } else {
        list()?;
    }

    Ok(())
}
