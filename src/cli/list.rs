use clap::Parser;

use crate::config::Config;
use crate::error::GuseError;
use crate::ui::UI;

#[derive(Parser, Debug)]
#[command(about = "Show all saved Git profiles")]
pub struct ListCommand;

impl ListCommand {
    pub fn execute(&self, config: &Config) -> Result<(), GuseError> {
        let profiles: Vec<_> = config.load_profiles()?.into_iter().collect();
        UI::print_profiles(&profiles);
        Ok(())
    }
}
