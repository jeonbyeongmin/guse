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
use crate::error::GuseError;

fn main() -> Result<(), GuseError> {
    env_logger::init();

    let args = Args::parse();
    let mut config = Config::new(); // Made config mutable

    match args.command {
        Commands::Add(cmd) => cmd.execute(&config), // Add still takes &Config
        Commands::Delete(cmd) => cmd.execute(&config), // Delete still takes &Config
        Commands::List(cmd) => cmd.execute(&config),   // List still takes &Config
        Commands::ListSsh(cmd) => cmd.execute(),
        Commands::Show(cmd) => cmd.execute(), // Show does not need config in its execute signature based on previous subtasks
        Commands::Switch(cmd) => cmd.execute(&config), // Switch still takes &Config
        Commands::Update(cmd) => cmd.execute(&config), // Update still takes &Config
        Commands::SetDefault(cmd) => cmd.execute(&mut config), // Added
        Commands::UnsetDefault(cmd) => cmd.execute(&mut config), // Added
    }
}
