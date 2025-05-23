use crate::config::Config;
use crate::error::GuseError;
use clap::Args;

/// Unsets the default Git profile.
#[derive(Args, Debug)]
pub struct UnsetDefaultCommand {}

impl UnsetDefaultCommand {
    pub fn execute(&self, config: &mut Config) -> Result<(), GuseError> {
        config.set_default_profile(None)?;

        println!("Default profile has been unset.");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::GuseError;
    // Note: ProfileMap and HashMap are not strictly needed for these specific tests
    // if MockConfig doesn't use them for unset_default.

    struct MockConfig {
        default_profile_set: Option<Option<String>>, 
        fail_set_default: bool,
    }

    impl MockConfig {
        fn new() -> Self {
            Self {
                // Initialize with a dummy default to ensure it's changed.
                default_profile_set: Some(Some("initial_dummy_default".to_string())), 
                fail_set_default: false,
            }
        }

        // Mocked version of Config::set_default_profile
        fn set_default_profile(&mut self, profile_name: Option<String>) -> Result<(), GuseError> {
            // Simulate updating the internal state before a potential save error
            self.default_profile_set = Some(profile_name); 
            if self.fail_set_default {
                return Err(GuseError::ConfigError("Simulated save failure from unset".to_string()));
            }
            Ok(())
        }
    }

    #[test]
    fn test_unset_default_success() {
        let mut mock_config = MockConfig::new();
        let command = UnsetDefaultCommand {};
        let result = command.execute(&mut mock_config);

        assert!(result.is_ok(), "Expected Ok, got {:?}", result.err());
        // Check that set_default_profile(None) was effectively called on the mock
        assert_eq!(mock_config.default_profile_set, Some(None), "Expected default_profile_set to be Some(None)");
    }

    #[test]
    fn test_unset_default_save_failure() {
        let mut mock_config = MockConfig::new();
        mock_config.fail_set_default = true; // Simulate a failure during the save operation

        let command = UnsetDefaultCommand {};
        let result = command.execute(&mut mock_config);

        assert!(result.is_err(), "Expected Err for save failure, got Ok");
        match result.err().unwrap() {
            GuseError::ConfigError(msg) => {
                assert_eq!(msg, "Simulated save failure from unset");
            }
            _ => panic!("Expected ConfigError type for save failure"),
        }
        // Verify that set_default_profile(None) was attempted on the mock,
        // even if it resulted in an error. The mock's internal state reflects the attempt.
        assert_eq!(mock_config.default_profile_set, Some(None), "Expected default_profile_set to be Some(None) as the mock updates before erroring.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ProfileMap; // Keep consistency, though not strictly needed for unset
    use crate::error::GuseError;
    use std::collections::HashMap;

    // Mock Config struct for testing purposes (can be shared or adapted from set_default tests)
    struct MockConfig {
        // profiles: ProfileMap, // Not strictly needed for unset_default tests if we only check set_default_profile(None)
        default_profile_set: Option<Option<String>>, // Stores what set_default_profile was called with
        fail_set_default: bool, // To simulate errors during set_default_profile
    }

    impl MockConfig {
        fn new() -> Self {
            Self {
                // profiles: HashMap::new(), 
                default_profile_set: Some(Some("initial_dummy_default".to_string())), // Start with some default
                fail_set_default: false,
            }
        }

        // Simplified version of Config::set_default_profile for the mock
        fn set_default_profile(&mut self, profile_name: Option<String>) -> Result<(), GuseError> {
            if self.fail_set_default {
                return Err(GuseError::ConfigError("Simulated save failure from unset".to_string()));
            }
            self.default_profile_set = Some(profile_name);
            Ok(())
        }
    }

    #[test]
    fn test_unset_default_success() {
        let mut mock_config = MockConfig::new();

        let command = UnsetDefaultCommand {};

        let result = command.execute(&mut mock_config);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result.err());
        assert_eq!(mock_config.default_profile_set, Some(None), "Expected default_profile_set to be Some(None)");
    }

    #[test]
    fn test_unset_default_save_failure() {
        let mut mock_config = MockConfig::new();
        mock_config.fail_set_default = true; // Simulate failure in the set_default_profile call

        let command = UnsetDefaultCommand {};
        
        let result = command.execute(&mut mock_config);
        assert!(result.is_err(), "Expected Err, got Ok");
        match result.err().unwrap() {
            GuseError::ConfigError(msg) => {
                assert_eq!(msg, "Simulated save failure from unset");
            }
            _ => panic!("Expected ConfigError for save failure"),
        }
        // Depending on mock implementation, default_profile_set might be Some(None) if error is after set,
        // or Some(Some("initial_dummy_default")) if error is before.
        // Current mock sets then errors.
        // The primary check is that an error is propagated.
        assert_eq!(mock_config.default_profile_set, Some(Some("initial_dummy_default".to_string())), "Expected default_profile_set to remain initial value on error before actual set");
        // Correcting the mock logic: if fail_set_default is true, it should error out *before* changing default_profile_set.
    }
}

// Corrected Mock for test_unset_default_save_failure:
// The mock should ideally reflect that if set_default_profile fails, the internal state (default_profile_set)
// might not change, or its change might be irrelevant if the operation as a whole fails.
// Let's adjust the mock and test slightly for clarity.

// No, the provided diff is for the file itself. I will adjust the mock in the next iteration if needed.
// For now, the mock's behavior is: it first tries to set `self.default_profile_set` and then, if `fail_set_default` is true, it returns an error.
// This means `default_profile_set` *will* be updated in the mock even if `fail_set_default` is true.
// The test `test_unset_default_save_failure` currently asserts that `default_profile_set` remains `Some(Some("initial_dummy_default"))`.
// This implies an expectation that if `set_default_profile` returns an error, the mock's internal state `default_profile_set` should not have been updated to `Some(None)`.
// Let's refine the mock logic within the test or the mock itself for this specific test case.
// The current mock design updates `default_profile_set` *before* checking `fail_set_default`.
// This is fine, the test just needs to be aware. The crucial part is that `execute` returns an error.

// Re-evaluating `test_unset_default_save_failure` assertion for `default_profile_set`:
// If `MockConfig::set_default_profile` is called, it *will* update `self.default_profile_set = Some(profile_name);`
// *before* it returns the error if `self.fail_set_default` is true.
// So, if `fail_set_default` is true, `default_profile_set` will become `Some(None)` and then an error is returned.
// The assertion `assert_eq!(mock_config.default_profile_set, Some(Some("initial_dummy_default".to_string())))` is therefore incorrect with the current mock.
// It should be `assert_eq!(mock_config.default_profile_set, Some(None))`.
// The purpose of the test is to ensure the command's `execute` method properly propagates the error.
// The state of the mock after a simulated failure is secondary to error propagation.
// Let's fix this assertion.
