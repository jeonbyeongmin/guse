mod cli;
mod config;
mod error;
mod git;
mod ui;
mod utils;

use clap::Parser;
use env_logger;

use crate::cli::{Args, Commands};
use crate::config::Config;
use crate::error::GitSwitchError;

fn main() -> Result<(), GitSwitchError> {
    env_logger::init();

    let args = Args::parse();
    let config = Config::new();

    match args.command {
        Commands::Add(cmd) => cmd.execute(&config),
        Commands::Delete(cmd) => cmd.execute(&config),
        Commands::List(cmd) => cmd.execute(&config),
        Commands::Show(cmd) => cmd.execute(),
        Commands::Switch(cmd) => cmd.execute(&config),
        Commands::Update(cmd) => cmd.execute(&config),
    }
}
