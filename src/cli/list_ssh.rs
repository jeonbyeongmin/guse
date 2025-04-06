use clap::Parser;
use colored::*;
use prettytable::*;
use std::fs;

use crate::error::GuseError;
use crate::utils::get_ssh_config_path;

#[derive(Parser, Debug)]
#[command(about = "List configured SSH hosts from ~/.ssh/config")]
pub struct ListSshCommand;

impl ListSshCommand {
    pub fn execute(&self) -> Result<(), GuseError> {
        let ssh_config = get_ssh_config_path()?;

        #[derive(Clone)]
        struct SshHost {
            name: String,
            hostname: String,
            user: String,
            port: String,
            identity_file: String,
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
                        identity_file: String::new(),
                    });
                } else if let Some(ref mut host) = current_host {
                    // Parse host info
                    if line.starts_with("HostName ") {
                        host.hostname = line.split_whitespace().nth(1).unwrap_or("").to_string();
                    } else if line.starts_with("User ") {
                        host.user = line.split_whitespace().nth(1).unwrap_or("").to_string();
                    } else if line.starts_with("Port ") {
                        host.port = line.split_whitespace().nth(1).unwrap_or("").to_string();
                    } else if line.starts_with("IdentityFile ") {
                        host.identity_file =
                            line.split_whitespace().nth(1).unwrap_or("").to_string();
                    }
                }
            }

            // Save last host info
            if let Some(host) = current_host {
                hosts.push(host);
            }
        }

        println!("\n{}", "🔑 Configured SSH Hosts:".cyan().bold());
        println!("{}", "=".repeat(40).cyan());

        let mut table = Table::new();
        table.add_row(row!["Host", "Hostname", "User", "Port", "Identity File"]);

        for host in hosts {
            table.add_row(row![
                host.name,
                host.hostname,
                host.user,
                host.port,
                host.identity_file
            ]);
        }

        table.printstd();
        println!();

        Ok(())
    }
}
