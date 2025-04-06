use clap::Parser;
use colored::*;
use dialoguer::Select;

use crate::config::Config;
use crate::error::GitSwitchError;
use crate::utils::backup_config_file;

#[derive(Parser, Debug)]
#[command(about = "Delete an existing Git profile")]
pub struct DeleteCommand {
    /// Name of the profile to delete
    #[arg(
        help = "Name of the profile to delete (e.g., personal, work). If not provided, you will be prompted to select from available profiles."
    )]
    #[arg(required = false)]
    pub profile: Option<String>,
}

impl DeleteCommand {
    pub fn execute(&self, config: &Config) -> Result<(), GitSwitchError> {
        use log::info;

        let profiles: Vec<_> = config.load_profiles()?.into_iter().collect();
        if profiles.is_empty() {
            println!("{}", "❌ No profiles found.".red().bold());
            return Ok(());
        }

        let profile_names: Vec<String> = profiles.iter().map(|(name, _)| name.clone()).collect();
        let selection = if self.profile.is_none() {
            Select::new()
                .with_prompt("Select profile to delete")
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

        let profile_to_delete = &profile_names[selection];
        info!("Deleting profile: {}", profile_to_delete);
        println!(
            "\n{}",
            format!("🗑️  Deleting profile '{}'", profile_to_delete)
                .red()
                .bold()
        );
        println!("{}", "=".repeat(40).red());

        backup_config_file(&config.path)?;
        config.delete_profile(profile_to_delete)?;

        info!("Profile deletion completed: {}", profile_to_delete);
        println!(
            "\n{}",
            format!("✅ Profile '{}' deleted successfully", profile_to_delete)
                .green()
                .bold()
        );
        Ok(())
    }
}
