use clap::Parser;
use colored::*;
use dialoguer::{Input, Select};

use crate::config::Config;
use crate::error::GitSwitchError;
use crate::utils::{backup_config_file, get_ssh_config_path, validate_email, validate_ssh_host};

#[derive(Parser, Debug)]
#[command(about = "Update an existing Git profile")]
pub struct UpdateCommand {
    /// Name of the profile to update
    #[arg(
        help = "Name of the profile to update (e.g., personal, work). If not provided, you will be prompted to select from available profiles."
    )]
    #[arg(required = false)]
    pub profile: Option<String>,
}

impl UpdateCommand {
    pub fn execute(&self, config: &Config) -> Result<(), GitSwitchError> {
        use log::info;
        use std::fs;

        let profiles: Vec<_> = config.load_profiles()?.into_iter().collect();
        if profiles.is_empty() {
            println!("{}", "❌ No profiles found.".red().bold());
            return Ok(());
        }

        let profile_names: Vec<String> = profiles.iter().map(|(name, _)| name.clone()).collect();
        let selection = if self.profile.is_none() {
            Select::new()
                .with_prompt("Select profile to update")
                .items(&profile_names)
                .default(0)
                .interact()?
        } else {
            let profile_name = self.profile.as_ref().unwrap();
            match profile_names.iter().position(|x| x == profile_name) {
                Some(idx) => idx,
                None => {
                    println!(
                        "{}",
                        format!("❌ Profile '{}' not found.", profile_name)
                            .red()
                            .bold()
                    );
                    return Ok(());
                }
            }
        };

        let profile_to_update = &profile_names[selection];
        let existing_profile = profiles[selection].1.clone();

        info!("Starting profile update: {}", profile_to_update);
        println!(
            "\n{}",
            format!("🆕 Updating profile '{}'", profile_to_update)
                .cyan()
                .bold()
        );
        println!("{}", "=".repeat(40).cyan());

        let name: String = Input::new()
            .with_prompt("Name (user.name)")
            .default(existing_profile.name.clone())
            .interact_text()?;

        let email: String = Input::new()
            .with_prompt("Email (user.email)")
            .default(existing_profile.email.clone())
            .interact_text()?;

        // Get SSH host list
        let ssh_config = get_ssh_config_path()?;

        #[derive(Clone)]
        struct SshHost {
            name: String,
            hostname: String,
            user: String,
            port: String,
        }

        let mut hosts = Vec::new();
        let mut current_host: Option<SshHost> = None;

        if let Ok(content) = fs::read_to_string(&ssh_config) {
            for line in content.lines() {
                let line = line.trim();

                if line.starts_with("Host ") {
                    // Save previous host info if exists
                    if let Some(host) = current_host.take() {
                        hosts.push(host);
                    }

                    // Start new host
                    let name = line.split_whitespace().nth(1).unwrap_or("").to_string();
                    current_host = Some(SshHost {
                        name,
                        hostname: String::new(),
                        user: String::new(),
                        port: String::new(),
                    });
                } else if let Some(ref mut host) = current_host {
                    // Parse host info
                    if line.starts_with("HostName ") {
                        host.hostname = line.split_whitespace().nth(1).unwrap_or("").to_string();
                    } else if line.starts_with("User ") {
                        host.user = line.split_whitespace().nth(1).unwrap_or("").to_string();
                    } else if line.starts_with("Port ") {
                        host.port = line.split_whitespace().nth(1).unwrap_or("").to_string();
                    }
                }
            }

            // Save last host info
            if let Some(host) = current_host {
                hosts.push(host);
            }
        }

        // Select SSH host
        let ssh_host = if hosts.is_empty() {
            Input::<String>::new()
                .with_prompt("SSH Host (e.g., github-personal)")
                .default(existing_profile.ssh_host.clone())
                .interact_text()?
        } else {
            // Convert host info to display format
            let host_items: Vec<String> = hosts
                .iter()
                .map(|host| {
                    let mut info = format!("{}", host.name);
                    if !host.hostname.is_empty() {
                        info.push_str(&format!(" ({})", host.hostname));
                    }
                    if !host.user.is_empty() {
                        info.push_str(&format!(" - User: {}", host.user));
                    }
                    if !host.port.is_empty() {
                        info.push_str(&format!(" - Port: {}", host.port));
                    }
                    info
                })
                .collect();

            // Add "Manual Input" option
            let mut items = host_items.clone();
            items.push("Manual Input".to_string());

            // Find index of existing SSH host
            let default_index = hosts
                .iter()
                .position(|h| h.name == existing_profile.ssh_host)
                .unwrap_or(0);

            let selection = Select::new()
                .with_prompt("Select SSH Host")
                .items(&items)
                .default(default_index)
                .interact()?;

            if selection == items.len() - 1 {
                // When "Manual Input" is selected
                Input::<String>::new()
                    .with_prompt("Enter SSH Host")
                    .default(existing_profile.ssh_host.clone())
                    .interact_text()?
            } else {
                hosts[selection].name.clone()
            }
        };

        validate_email(&email)?;
        validate_ssh_host(&ssh_host)?;

        // Backup configuration file
        backup_config_file(&config.path)?;

        let profile = crate::config::Profile {
            name,
            email,
            ssh_host,
        };

        config.update_profile(profile_to_update, profile)?;

        info!("Profile update completed: {}", profile_to_update);
        println!(
            "\n{}",
            format!("✅ Profile '{}' updated successfully", profile_to_update)
                .green()
                .bold()
        );

        Ok(())
    }
}
