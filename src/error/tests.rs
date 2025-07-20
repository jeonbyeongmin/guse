#[cfg(test)]
mod tests {
    use crate::error::GuseError;
    use std::io::{Error, ErrorKind};

    #[test]
    fn test_guse_error_display_messages() {
        // Test IoError variant
        let io_error = Error::new(ErrorKind::NotFound, "file not found");
        let guse_error = GuseError::IoError(io_error);
        assert!(guse_error.to_string().contains("IO Error:"));
        assert!(guse_error.to_string().contains("file not found"));

        // Test ValidationError variant
        let validation_error = GuseError::ValidationError("invalid email".to_string());
        assert_eq!(validation_error.to_string(), "Validation Error: invalid email");

        // Test GitError variant
        let git_error = GuseError::GitError("commit failed".to_string());
        assert_eq!(git_error.to_string(), "Git Command Error: commit failed");

        // Test ConfigError variant
        let config_error = GuseError::ConfigError("missing profile".to_string());
        assert_eq!(config_error.to_string(), "Configuration Error: missing profile");
    }

    #[test]
    fn test_guse_error_from_conversions() {
        // Test From<std::io::Error>
        let io_error = Error::new(ErrorKind::PermissionDenied, "access denied");
        let guse_error: GuseError = io_error.into();
        assert!(matches!(guse_error, GuseError::IoError(_)));

        // Test From<toml::de::Error>
        let invalid_toml = "invalid = toml = content";
        let toml_parse_result: Result<toml::Value, toml::de::Error> = toml::from_str(invalid_toml);
        if let Err(toml_error) = toml_parse_result {
            let guse_error: GuseError = toml_error.into();
            assert!(matches!(guse_error, GuseError::TomlError(_)));
            assert!(guse_error.to_string().contains("TOML Parsing Error:"));
        }
    }

    #[test]
    fn test_custom_from_implementations() {
        // Test From<toml::ser::Error> 
        let data = std::collections::HashMap::from([("key", std::f64::NAN)]);
        let toml_ser_result = toml::to_string(&data);
        if let Err(toml_ser_error) = toml_ser_result {
            let guse_error: GuseError = toml_ser_error.into();
            assert!(matches!(guse_error, GuseError::ConfigError(_)));
            assert!(guse_error.to_string().contains("TOML Serialization Error:"));
        }
    }

    #[test]
    fn test_error_is_send_sync() {
        // Test that GuseError implements Send + Sync (important for error handling)
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<GuseError>();
    }
}