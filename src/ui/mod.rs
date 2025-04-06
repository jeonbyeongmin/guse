use crate::config::Profile;
use crate::git::GitConfig;
use colored::*;
use prettytable::*;

pub struct UI;

impl UI {
    pub fn print_profile_table(profile: &Profile, github_user: &str, repo_name: &str) {
        let mut table = Table::new();
        table.add_row(row!["Item", "Value"]);
        table.add_row(row!["Name", &profile.name]);
        table.add_row(row!["Email", &profile.email]);
        table.add_row(row!["SSH Host", &profile.ssh_host]);
        table.add_row(row!["GitHub User", github_user]);
        table.add_row(row!["Repository", repo_name]);
        table.printstd();
    }

    pub fn print_current_config(config: &GitConfig) {
        println!("\n{}", "⚙️  Current Git Configuration:".cyan().bold());
        println!("{}", "=".repeat(40).cyan());

        let mut table = Table::new();
        table.add_row(row!["Item", "Value"]);
        table.add_row(row!["Name", &config.user_name]);
        table.add_row(row!["Email", &config.user_email]);

        // Display remote URL or "Not configured" if empty
        let remote_display = if config.remote_url.is_empty() {
            "Not configured".red()
        } else {
            config.remote_url.green()
        };
        table.add_row(row!["Remote", remote_display]);

        table.printstd();
        println!();
    }

    pub fn print_profiles(profiles: &Vec<(String, Profile)>) {
        println!("\n{}", "📋 Available Profiles:".cyan().bold());
        println!("{}", "=".repeat(40).cyan());

        let mut table = Table::new();
        table.add_row(row!["Profile", "Name", "Email", "SSH Host"]);

        for (name, profile) in profiles {
            table.add_row(row![name, profile.name, profile.email, profile.ssh_host]);
        }

        table.printstd();
        println!();
    }

    pub fn print_ssh_hosts(hosts: &[String]) {
        println!("\n{}", "🔑 SSH Host List:".cyan().bold());
        println!("{}", "=".repeat(40).cyan());

        let mut table = Table::new();
        table.add_row(row!["Host"]);

        for host in hosts {
            table.add_row(row![host]);
        }

        table.printstd();
        println!();
    }
}
