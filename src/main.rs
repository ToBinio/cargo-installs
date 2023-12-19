use crate::cli::{CargoCli, Commands};
use crate::list::list;
use crate::update::update;
use clap::Parser;

pub mod cli;
pub mod list;
pub mod update;

fn main() {
    let CargoCli::Installs(args) = CargoCli::parse();

    if let Some(commands) = args.command {
        match commands {
            Commands::Update => update(),
        }
    } else {
        list()
    }
}
