mod config;
mod error;
mod git;
mod ui;
mod utils;

use clap::Parser;
use colored::*;
use dialoguer::{Input, Select};
use env_logger;
use log::info;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use toml;

use crate::config::{Config, Profile};
use crate::error::GitSwitchError;
use crate::git::Git;
use crate::ui::UI;
use crate::utils::{backup_config_file, validate_email, validate_ssh_host};

/// CLI 인자 정의
#[derive(Parser, Debug)]
#[command(author, version, about = "Git Account Switcher")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Add a new profile
    Add {
        /// Profile name to add
        profile: String,
    },
    /// Show current settings
    Show,
    /// List available profiles
    List,
    /// List SSH configurations
    ListSsh,
    /// Switch to a profile
    Switch {
        /// Profile name to use (e.g., personal, work)
        profile: String,
    },
    /// Update an existing profile
    Update {
        /// Profile name to update
        profile: String,
    },
}

type ProfileMap = HashMap<String, Profile>;

fn load_profile(profile_name: &str, config: &Config) -> Result<Profile, GitSwitchError> {
    let config_data = fs::read_to_string(&config.path).map_err(|e| GitSwitchError::IoError(e))?;

    let profiles: ProfileMap =
        toml::from_str(&config_data).map_err(|e| GitSwitchError::TomlError(e))?;

    match profiles.get(profile_name).cloned() {
        Some(profile) => Ok(profile),
        None => {
            println!(
                "{} {}",
                "❓".yellow().bold(),
                format!("Profile '{}' does not exist.", profile_name).yellow()
            );
            print!("{}", "➕ Would you like to add it now? (y/N): ".cyan());
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() == "y" {
                add_profile_interactively(profile_name, config)?;
                load_profile(profile_name, config)
            } else {
                Err(GitSwitchError::ConfigError(
                    "Operation cancelled.".to_string(),
                ))
            }
        }
    }
}

fn main() -> Result<(), GitSwitchError> {
    env_logger::init();

    let args = Args::parse();
    let config = Config::new();
    let mut git = Git::new();

    match &args.command {
        Commands::List => {
            let profiles: Vec<_> = config.load_profiles()?.into_iter().collect();
            UI::print_profiles(&profiles);
            return Ok(());
        }
        Commands::Show => {
            let current_config = git.get_current_config()?;

            // Check if remote URL is empty and provide a helpful message
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
            return Ok(());
        }
        Commands::ListSsh => {
            let ssh_config = dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".ssh/config");

            let mut hosts = Vec::new();
            if let Ok(content) = fs::read_to_string(&ssh_config) {
                for line in content.lines() {
                    if line.trim_start().starts_with("Host ") {
                        if let Some(name) = line.trim().split_whitespace().nth(1) {
                            hosts.push(name.to_string());
                        }
                    }
                }
            }
            UI::print_ssh_hosts(&hosts);
            return Ok(());
        }
        Commands::Add { profile } => {
            add_profile_interactively(profile, &config)?;
            return Ok(());
        }
        Commands::Update { profile } => {
            update_profile_interactively(&profile, &config)?;
            return Ok(());
        }
        Commands::Switch { profile } => {
            info!("Loading profile '{}'", profile);
            println!(
                "{} {}",
                "🔄".blue().bold(),
                format!("Loading profile '{}'...", profile).blue()
            );

            let profile_data = load_profile(profile, &config)?;

            info!("Starting Git configuration change");
            println!(
                "{} {}",
                "⚙️".blue().bold(),
                "Changing Git configuration...".blue()
            );

            git.set_config(&profile_data.name, &profile_data.email)?;

            // 원격 저장소 정보가 있는 경우에만 remote URL을 변경
            match git.parse_origin_url() {
                Ok((github_user, repo_name)) => {
                    git.set_remote(&profile_data.ssh_host, &github_user, &repo_name)?;
                    info!("Git account switch completed");
                    println!("\n{}", "✅ Git account switch completed:".green().bold());
                    UI::print_profile_table(&profile_data, &github_user, &repo_name);
                }
                Err(_e) => {
                    println!(
                        "{} {}",
                        "⚠️".yellow().bold(),
                        "Remote repository not found. Only Git profile has been updated.".yellow()
                    );
                    println!(
                        "{}",
                        "To add a remote repository, use the following command:".yellow()
                    );
                    println!("{}", "  git remote add origin <repository-url>".cyan());
                    info!("Git profile switch completed (without remote update)");
                    println!("\n{}", "✅ Git profile switch completed:".green().bold());
                    UI::print_profile_table(&profile_data, "N/A", "N/A");
                }
            }
            return Ok(());
        }
    }
}

fn add_profile_interactively(profile_name: &str, config: &Config) -> Result<(), GitSwitchError> {
    info!("Starting new profile addition: {}", profile_name);
    println!(
        "\n{}",
        format!("🆕 Adding new profile '{}'", profile_name)
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
    let ssh_config = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".ssh/config");

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

    let profile = Profile {
        name,
        email,
        ssh_host,
    };

    config.add_profile(profile_name.to_string(), profile)?;

    info!("Profile addition completed: {}", profile_name);
    println!(
        "\n{}",
        format!("✅ Profile '{}' added successfully", profile_name)
            .green()
            .bold()
    );

    Ok(())
}

fn update_profile_interactively(profile_name: &str, config: &Config) -> Result<(), GitSwitchError> {
    info!("Starting profile update: {}", profile_name);
    println!(
        "\n{}",
        format!("🆕 Updating profile '{}'", profile_name)
            .cyan()
            .bold()
    );
    println!("{}", "=".repeat(40).cyan());

    // Load existing profile info
    let profiles = config.load_profiles()?;
    let existing_profile = profiles.get(profile_name).ok_or_else(|| {
        GitSwitchError::ConfigError(format!("Profile '{}' does not exist.", profile_name))
    })?;

    let name: String = Input::new()
        .with_prompt("Name (user.name)")
        .default(existing_profile.name.clone())
        .interact_text()?;

    let email: String = Input::new()
        .with_prompt("Email (user.email)")
        .default(existing_profile.email.clone())
        .interact_text()?;

    // Get SSH host list
    let ssh_config = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".ssh/config");

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

    let profile = Profile {
        name,
        email,
        ssh_host,
    };

    config.update_profile(profile_name, profile)?;

    info!("Profile update completed: {}", profile_name);
    println!(
        "\n{}",
        format!("✅ Profile '{}' updated successfully", profile_name)
            .green()
            .bold()
    );

    Ok(())
}
