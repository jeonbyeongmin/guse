use crate::config::Config;
use crate::error::GuseError;
use clap::Args;

/// Sets a profile as the default.
#[derive(Args, Debug)]
pub struct SetDefaultCommand {
    /// The name of the profile to set as default
    #[arg(required = true)]
    profile_name: String,
}

impl SetDefaultCommand {
    pub fn execute(&self, config: &mut Config) -> Result<(), GuseError> {
        let profiles = config.load_profiles()?;

        if !profiles.contains_key(&self.profile_name) {
            return Err(GuseError::ConfigError(format!(
                "Profile '{}' not found.",
                self.profile_name
            )));
        }

        config.set_default_profile(Some(self.profile_name.clone()))?;

        println!("Default profile set to '{}'.", self.profile_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Profile, ProfileMap}; // Assuming ProfileMap is HashMap<String, Profile>
    use std::collections::HashMap;

    // Mock Config struct for testing purposes
    struct MockConfig {
        profiles: ProfileMap,
        default_profile_set: Option<Option<String>>, // Stores what set_default_profile was called with
        fail_set_default: bool, // To simulate errors during set_default_profile
    }

    impl MockConfig {
        fn new(profiles: ProfileMap) -> Self {
            Self {
                profiles,
                default_profile_set: None, // Initially, no call has been made
                fail_set_default: false,
            }
        }

        // Simplified version of Config::load_profiles for the mock
        fn load_profiles(&self) -> Result<ProfileMap, GuseError> {
            Ok(self.profiles.clone())
        }

        // Simplified version of Config::set_default_profile for the mock
        fn set_default_profile(&mut self, profile_name: Option<String>) -> Result<(), GuseError> {
            if self.fail_set_default {
                return Err(GuseError::ConfigError("Simulated save failure".to_string()));
            }
            self.default_profile_set = Some(profile_name);
            Ok(())
        }
    }

    #[test]
    fn test_set_default_success() {
        let mut profiles = HashMap::new();
        profiles.insert(
            "work".to_string(),
            Profile {
                name: "Work User".to_string(),
                email: "work@example.com".to_string(),
                ssh_host: "github.com".to_string(),
            },
        );
        let mut mock_config = MockConfig::new(profiles);

        let command = SetDefaultCommand {
            profile_name: "work".to_string(),
        };

        let result = command.execute(&mut mock_config);
        assert!(result.is_ok());
        assert_eq!(mock_config.default_profile_set, Some(Some("work".to_string())));
    }

    #[test]
    fn test_set_default_profile_not_found() {
        let profiles = HashMap::new(); // No profiles
        let mut mock_config = MockConfig::new(profiles);

        let command = SetDefaultCommand {
            profile_name: "nonexistent".to_string(),
        };

        let result = command.execute(&mut mock_config);
        assert!(result.is_err());
        match result.err().unwrap() {
            GuseError::ConfigError(msg) => {
                assert!(msg.contains("Profile 'nonexistent' not found."));
            }
            _ => panic!("Expected ConfigError for profile not found"),
        }
        // Ensure set_default_profile was not called
        assert_eq!(mock_config.default_profile_set, None);
    }

    #[test]
    fn test_set_default_save_failure() {
        let mut profiles = HashMap::new();
         profiles.insert(
            "home".to_string(),
            Profile {
                name: "Home User".to_string(),
                email: "home@example.com".to_string(),
                ssh_host: "gitlab.com".to_string(),
            },
        );
        let mut mock_config = MockConfig::new(profiles);
        mock_config.fail_set_default = true; // Simulate failure in the set_default_profile call

        let command = SetDefaultCommand {
            profile_name: "home".to_string(),
        };
        
        let result = command.execute(&mut mock_config);
        assert!(result.is_err());
        match result.err().unwrap() {
            GuseError::ConfigError(msg) => {
                assert_eq!(msg, "Simulated save failure");
            }
            _ => panic!("Expected ConfigError for save failure"),
        }
        // default_profile_set would still be updated in our mock before the error is returned by mock's set_default_profile
        // This depends on the mock's implementation. If the mock errors out *before* setting, this would be None.
        // In the current mock, it sets then errors, so it would be Some(Some("home")).
        // For a real Config, if save_profiles fails, config.default_profile would have been set in memory.
        // The test here is more about whether the command propagates the error from config.set_default_profile.
        // So, the assertion on default_profile_set might not be strictly necessary if we only care about error propagation.
        // Let's assume the command should not proceed if set_default_profile fails.
        // The important part is that an error is returned.
    }
}
