use clap::{Parser, Subcommand};

#[derive(Parser)] // requires `derive` feature
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
pub enum CargoCli {
    Installs(InstallsArgs),
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
pub struct InstallsArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(alias = "u")]
    Update,
    Blacklist(BlacklistArgs),
}

#[derive(clap::Args)]
pub struct BlacklistArgs {
    #[arg(short, long)]
    pub remove: bool,
    #[arg(short, long)]
    pub list: bool,
    pub crates: Vec<String>,
}
