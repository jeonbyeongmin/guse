use clap::Parser;
use colored::*;
use std::fs::OpenOptions;
use std::io::Write;

use crate::error::GuseError;
use crate::utils::get_ssh_config_path;

#[derive(Parser, Debug)]
#[command(about = "Add a new SSH host to ~/.ssh/config")]
pub struct AddSshCommand;

impl AddSshCommand {
    pub fn execute(&self) -> Result<(), GuseError> {
        use dialoguer::Input;


        let ssh_config = get_ssh_config_path()?;

        println!("\n{}", "🔑 Add SSH Host".cyan().bold());
        println!("{}", "=".repeat(40).cyan());

        let host: String = Input::new()
            .with_prompt("Host alias (e.g., myserver)")
            .interact_text()?;
        let hostname: String = Input::new()
            .with_prompt("HostName (e.g., 192.168.0.1 or github.com)")
            .interact_text()?;
        let user: String = Input::new()
            .with_prompt("User (e.g., ubuntu)")
            .interact_text()?;
        let port: String = Input::new()
            .with_prompt("Port (default: 22)")
            .default("22".to_string())
            .interact_text()?;
        // Find files starting with id_* in ~/.ssh directory and provide as options
        let ssh_dir = shellexpand::tilde("~/.ssh").to_string();
        let mut identity_files = match std::fs::read_dir(&ssh_dir) {
            Ok(entries) => entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| {
                    if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                        name.starts_with("id_") && !name.ends_with(".pub") && p.is_file()
                    } else {
                        false
                    }
                })
                .map(|p| p.to_string_lossy().to_string())
                .collect::<Vec<_>>(),
            Err(_) => vec![],
        };
        identity_files.push("Generate new key".to_string());
        identity_files.push("Enter manually".to_string());

        let identity_file = if identity_files.len() > 2 {
            use dialoguer::Select;
            let selection = Select::new()
                .with_prompt("Select IdentityFile (or choose 'Generate new key', 'Enter manually')")
                .items(&identity_files)
                .default(0)
                .interact()?;
            if identity_files[selection] == "Enter manually" {
                Input::new()
                    .with_prompt("Enter IdentityFile path (e.g., ~/.ssh/id_rsa)")
                    .default("~/.ssh/id_rsa".to_string())
                    .interact_text()?
            } else if identity_files[selection] == "Generate new key" {
                // Use host alias as id_ prefix
                let ssh_dir = shellexpand::tilde("~/.ssh").to_string();
                let new_key_path = format!("{}/id_{}", ssh_dir, host);
                let expanded_new_key_path = shellexpand::tilde(&new_key_path).to_string();
                if !std::path::Path::new(&expanded_new_key_path).exists() {
                    println!("{} Generating SSH key...", "🔑".yellow());
                    let output = std::process::Command::new("ssh-keygen")
                        .arg("-t").arg("rsa")
                        .arg("-b").arg("4096")
                        .arg("-f").arg(&expanded_new_key_path)
                        .arg("-N").arg("")
                        .output();
                    match output {
                        Ok(out) if out.status.success() => {
                            println!("{} SSH key generated: {}", "✅".green(), expanded_new_key_path);
                        }
                        Ok(out) => {
                            eprintln!("{} ssh-keygen failed: {}", "❌".red(), String::from_utf8_lossy(&out.stderr));
                        }
                        Err(e) => {
                            eprintln!("{} ssh-keygen error: {}", "❌".red(), e);
                        }
                    }
                } else {
                    println!("{} Key already exists at: {}", "⚠️".yellow(), expanded_new_key_path);
                }
                new_key_path
            } else {
                identity_files[selection].clone()
            }
        } else {
            Input::new()
                .with_prompt("IdentityFile (e.g., ~/.ssh/id_rsa)")
                .default("~/.ssh/id_rsa".to_string())
                .interact_text()?
        };

        // If SSH key does not exist, generate automatically
        let expanded_identity_file = shellexpand::tilde(&identity_file).to_string();
        if !std::path::Path::new(&expanded_identity_file).exists() {
            println!("{} SSH key does not exist. Generating automatically...", "🔑".yellow());
            let output = std::process::Command::new("ssh-keygen")
                .arg("-t").arg("rsa")
                .arg("-b").arg("4096")
                .arg("-f").arg(&expanded_identity_file)
                .arg("-N").arg("")
                .output();
            match output {
                Ok(out) if out.status.success() => {
                    println!("{} SSH key generated: {}", "✅".green(), expanded_identity_file);
                }
                Ok(out) => {
                    eprintln!("{} ssh-keygen failed: {}", "❌".red(), String::from_utf8_lossy(&out.stderr));
                }
                Err(e) => {
                    eprintln!("{} ssh-keygen error: {}", "❌".red(), e);
                }
            }
        }

        let entry = format!(
            "\nHost {}\n    HostName {}\n    User {}\n    Port {}\n    IdentityFile {}\n",
            host, hostname, user, port, identity_file
        );

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&ssh_config)?;
        file.write_all(entry.as_bytes())?;

        println!("\n{} SSH host added to {}!", "✅".green(), ssh_config.display());
        Ok(())
    }
}
