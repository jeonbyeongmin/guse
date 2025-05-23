use clap::Parser;
use colored::*;

use crate::config::Config; // Added
use crate::error::GuseError;
use crate::git::Git;
use crate::ui::UI;

#[derive(Parser, Debug)]
#[command(about = "Show current Git configuration and guse default")]
pub struct ShowCommand;

impl ShowCommand {
    pub fn execute(&self) -> Result<(), GuseError> {
        let git = Git::new();
        let config = Config::new(); // Added: Instantiate Config

        let current_git_config = git.get_current_config()?;

        // UI::print_current_config already handles the "No remote" warning if url is empty
        // and prints the table for Name, Email, Remote.
        UI::print_current_config(&current_git_config);

        // Display Matched guse Profile (after the table printed by UI::print_current_config)
        let profiles = config.load_profiles()?;
        let mut matched_guse_profile_name: Option<String> = None;

        if !current_git_config.user_name.is_empty() || !current_git_config.user_email.is_empty() {
            for (name, profile) in profiles {
                if profile.name == current_git_config.user_name && profile.email == current_git_config.user_email {
                    matched_guse_profile_name = Some(name);
                    break;
                }
            }

            if let Some(name) = matched_guse_profile_name {
                // Adding a bit of spacing if UI::print_current_config ends tightly.
                // UI::print_current_config prints a newline at the end, so this should be fine.
                println!("  {}{}", "✓ Matched guse Profile: ".dimmed(), name.green());
            } else {
                println!("  {}{}", "✗ Matched guse Profile: ".dimmed(), "None (current Git config does not match any guse profile, or is incomplete)".yellow());
            }
        }
        // If both current git name and email are empty, UI::print_current_config will show that.
        // No "Matched guse Profile" line needed in that case, as there's nothing to match.
        
        println!(); // Extra blank line for separation before guse global config

        // Display guse default profile
        println!("{}", "guse Global Configuration:".cyan().bold());
        println!("{}", "=".repeat(40).cyan());
        match config.get_default_profile() {
            Some(default_profile_name) => {
                println!(
                    "  Default Profile: {} {}",
                    default_profile_name.cyan(),
                    "(set globally)".dimmed()
                );
            }
            None => {
                println!("  Default Profile: {}", "None".yellow());
            }
        }
        println!(); // Final blank line for clean output

        Ok(())
    }
}
