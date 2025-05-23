use clap::Parser;
use colored::*;
use dialoguer::Select;

use crate::config::{Config, Profile};
use crate::error::GuseError;
use crate::git::Git;
use crate::ui::UI;
use log::info;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(about = "Switch to a different Git profile")]
pub struct SwitchCommand {
    /// Name of the profile to switch to
    #[arg(
        help = "Name of the profile to switch to (e.g., personal, work). If not provided, attempts to use default or prompts for selection."
    )]
    #[arg(required = false)]
    pub profile: Option<String>,
}

impl SwitchCommand {
    fn perform_switch(
        &self,
        git: &mut Git,
        profile_data: &Profile,
        switched_by_default: bool,
    ) -> Result<(), GuseError> {
        info!(
            "Performing switch to profile: '{}'",
            profile_data.name
        );
        println!(
            "{} {}",
            "⚙️".blue().bold(),
            format!("Changing Git configuration for '{}'...", profile_data.name).blue()
        );

        git.set_config(&profile_data.name, &profile_data.email)?;

        match git.parse_origin_url() {
            Ok((github_user, repo_name)) => {
                if !profile_data.ssh_host.is_empty() {
                    git.set_remote(&profile_data.ssh_host, &github_user, &repo_name)?;
                    info!(
                        "Git remote updated for profile '{}' with ssh_host '{}'",
                        profile_data.name, profile_data.ssh_host
                    );
                    println!(
                        "\n{}",
                        format!(
                            "✅ Git account {}switched to '{}':",
                            if switched_by_default { "automatically (default) " } else { "" },
                            profile_data.name
                        )
                        .green()
                        .bold()
                    );
                    UI::print_profile_table(profile_data, &github_user, &repo_name);
                } else {
                    info!(
                        "Git profile '{}' does not have an ssh_host configured. Remote not updated.",
                        profile_data.name
                    );
                    println!(
                        "\n{}",
                        format!(
                            "✅ Git profile {}switched to '{}' (remote not updated as no ssh_host is set for this profile):",
                            if switched_by_default { "automatically (default) " } else { "" },
                            profile_data.name
                        )
                        .green()
                        .bold()
                    );
                    UI::print_profile_table(profile_data, &github_user, &repo_name);
                }
            }
            Err(_e) => {
                println!(
                    "{} {}",
                    "⚠️".yellow().bold(),
                    "Remote repository not found or not in a recognized format. Only local Git profile has been updated.".yellow()
                );
                println!(
                    "{}",
                    "To add or reconfigure a remote repository, use: git remote add origin <repository-url>".yellow()
                );
                info!(
                    "Git profile switch for '{}' completed (without remote update due to parsing/missing remote)",
                    profile_data.name
                );
                println!(
                    "\n{}",
                    format!(
                        "✅ Git profile {}switched to '{}' (remote not updated):",
                        if switched_by_default { "automatically (default) " } else { "" },
                        profile_data.name
                    )
                    .green()
                    .bold()
                );
                UI::print_profile_table(profile_data, "N/A", "N/A");
            }
        }
        Ok(())
    }

    pub fn execute(&self, config: &Config) -> Result<(), GuseError> {
        let mut git = Git::new();
        let profiles_map = config.load_profiles()?;

        if profiles_map.is_empty() {
            println!("{}", "❌ No profiles found. Add one using 'guse add'.".red().bold());
            return Ok(());
        }
        
        let profile_to_switch_name: String;
        let mut switched_by_default = false;

        if let Some(ref specific_profile_name) = self.profile {
            // User provided a profile name argument
            profile_to_switch_name = specific_profile_name.clone();
            if !profiles_map.contains_key(&profile_to_switch_name) {
                 println!(
                    "{}",
                    format!("❌ Profile '{}' not found.", profile_to_switch_name)
                        .red()
                        .bold()
                );
                return Ok(());
            }
        } else {
            // No profile name argument, try default or interactive
            if let Some(default_profile_name) = config.get_default_profile() {
                if profiles_map.contains_key(&default_profile_name) {
                    profile_to_switch_name = default_profile_name;
                    switched_by_default = true;
                    println!(
                        "{} {}",
                        "ℹ️".blue().bold(),
                        format!("Using default profile '{}'.", profile_to_switch_name).blue()
                    );
                } else {
                    // Default profile is set but not found in current profiles (corrupted state?)
                    println!(
                        "{} {}",
                        "⚠️".yellow().bold(),
                        format!("Default profile '{}' is set but not found. Please check your configuration or select manually.", default_profile_name).yellow()
                    );
                    // Fallback to interactive selection
                    let profile_names: Vec<String> = profiles_map.keys().cloned().collect();
                     let selection_idx = Select::new()
                        .with_prompt("Select profile to switch to")
                        .items(&profile_names)
                        .default(0)
                        .interact()?;
                    profile_to_switch_name = profile_names[selection_idx].clone();
                }
            } else {
                // No default profile, proceed with interactive selection
                let profile_names: Vec<String> = profiles_map.keys().cloned().collect();
                 let selection_idx = Select::new()
                    .with_prompt("Select profile to switch to")
                    .items(&profile_names)
                    .default(0)
                    .interact()?;
                profile_to_switch_name = profile_names[selection_idx].clone();
            }
        }

        info!("Attempting to switch to profile: '{}'", profile_to_switch_name);
        println!(
            "{} {}",
            "🔄".blue().bold(),
            format!("Loading profile '{}'...", profile_to_switch_name).blue()
        );
        
        match profiles_map.get(&profile_to_switch_name) {
            Some(profile_data) => {
                self.perform_switch(&mut git, profile_data, switched_by_default)
            }
            None => {
                // This case should ideally be caught earlier if a specific name was given
                // or if default profile name was invalid.
                 println!(
                    "{}",
                    format!("❌ Unexpected error: Profile '{}' could not be loaded.", profile_to_switch_name)
                        .red()
                        .bold()
                );
                Ok(())
            }
        }
    }
}
