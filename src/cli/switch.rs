use clap::Parser;
use colored::*;
use dialoguer::Select;

use crate::config::Config;
use crate::error::GitSwitchError;
use crate::git::Git;
use crate::ui::UI;

#[derive(Parser, Debug)]
#[command(about = "Switch to a different Git profile")]
pub struct SwitchCommand {
    /// Name of the profile to switch to
    #[arg(
        help = "Name of the profile to switch to (e.g., personal, work). If not provided, you will be prompted to select from available profiles."
    )]
    #[arg(required = false)]
    pub profile: Option<String>,
}

impl SwitchCommand {
    pub fn execute(&self, config: &Config, git: &mut Git) -> Result<(), GitSwitchError> {
        use log::info;

        let profiles: Vec<_> = config.load_profiles()?.into_iter().collect();
        if profiles.is_empty() {
            println!("{}", "❌ No profiles found.".red().bold());
            return Ok(());
        }

        let profile_names: Vec<String> = profiles.iter().map(|(name, _)| name.clone()).collect();
        let selection = if self.profile.is_none() {
            Select::new()
                .with_prompt("Select profile to switch to")
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

        let profile_to_switch = &profile_names[selection];
        info!("Loading profile '{}'", profile_to_switch);
        println!(
            "{} {}",
            "🔄".blue().bold(),
            format!("Loading profile '{}'...", profile_to_switch).blue()
        );

        let profile_data = profiles
            .get(selection)
            .map(|(_, profile)| profile.clone())
            .unwrap();

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

        Ok(())
    }
}
