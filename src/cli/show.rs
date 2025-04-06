use clap::Parser;
use colored::*;

use crate::error::GuseError;
use crate::git::Git;
use crate::ui::UI;

#[derive(Parser, Debug)]
#[command(about = "Show current Git configuration")]
pub struct ShowCommand;

impl ShowCommand {
    pub fn execute(&self) -> Result<(), GuseError> {
        let git = Git::new();
        let current_config = git.get_current_config()?;

        if current_config.remote_url.is_empty() {
            println!(
                "{} {}",
                "⚠️".yellow().bold(),
                "No remote repository configured.".yellow()
            );
            println!(
                "{}",
                "To add a remote repository, use the following command:".yellow()
            );
            println!("{}", "  git remote add origin <repository-url>".cyan());
        }

        UI::print_current_config(&current_config);
        Ok(())
    }
}
