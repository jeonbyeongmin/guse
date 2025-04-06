use clap::Parser;
use colored::*;

use crate::config::Config;
use crate::error::GitSwitchError;
use crate::utils::{backup_config_file, get_ssh_config_path, validate_email, validate_ssh_host};

#[derive(Parser, Debug)]
#[command(about = "Add a new Git profile")]
pub struct AddCommand {
    /// Name of the profile to add
    #[arg(help = "Name of the profile to add (e.g., personal, work)")]
    pub profile: String,
}

impl AddCommand {
    pub fn execute(&self, config: &Config) -> Result<(), GitSwitchError> {
        use dialoguer::Input;
        use log::info;
        use std::fs;

        info!("Starting new profile addition: {}", self.profile);
        println!(
            "\n{}",
            format!("🆕 Adding new profile '{}'", self.profile)
                .cyan()
                .bold()
        );
        println!("{}", "=".repeat(40).cyan());

        let name: String = Input::new()
            .with_prompt("Name (user.name)")
            .interact_text()?;

        let email: String = Input::new()
            .with_prompt("Email (user.email)")
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
                .interact_text()?
        } else {
            use dialoguer::Select;

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

            let selection = Select::new()
                .with_prompt("Select SSH Host")
                .items(&items)
                .default(0)
                .interact()?;

            if selection == items.len() - 1 {
                // When "Manual Input" is selected
                Input::<String>::new()
                    .with_prompt("Enter SSH Host")
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

        config.add_profile(self.profile.clone(), profile)?;

        info!("Profile addition completed: {}", self.profile);
        println!(
            "\n{}",
            format!("✅ Profile '{}' added successfully", self.profile)
                .green()
                .bold()
        );

        Ok(())
    }
}
