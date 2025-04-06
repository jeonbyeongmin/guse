use std::fmt;

#[derive(Debug)]
pub enum GuseError {
    IoError(std::io::Error),
    TomlError(toml::de::Error),
    GitError(String),
    ValidationError(String),
    ConfigError(String),
    DialoguerError(dialoguer::Error),
}

impl fmt::Display for GuseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GuseError::IoError(e) => write!(f, "IO Error: {}", e),
            GuseError::TomlError(e) => write!(f, "TOML Parsing Error: {}", e),
            GuseError::GitError(e) => write!(f, "Git Command Error: {}", e),
            GuseError::ValidationError(e) => write!(f, "Validation Error: {}", e),
            GuseError::ConfigError(e) => write!(f, "Configuration Error: {}", e),
            GuseError::DialoguerError(e) => write!(f, "Interactive Input Error: {}", e),
        }
    }
}

impl std::error::Error for GuseError {}

impl From<std::io::Error> for GuseError {
    fn from(err: std::io::Error) -> Self {
        GuseError::IoError(err)
    }
}

impl From<toml::de::Error> for GuseError {
    fn from(err: toml::de::Error) -> Self {
        GuseError::TomlError(err)
    }
}

impl From<toml::ser::Error> for GuseError {
    fn from(err: toml::ser::Error) -> Self {
        GuseError::ConfigError(format!("TOML Serialization Error: {}", err))
    }
}

impl From<crate::config::ConfigError> for GuseError {
    fn from(err: crate::config::ConfigError) -> Self {
        GuseError::ConfigError(err.to_string())
    }
}

impl From<crate::git::GitError> for GuseError {
    fn from(err: crate::git::GitError) -> Self {
        GuseError::GitError(err.to_string())
    }
}

impl From<dialoguer::Error> for GuseError {
    fn from(err: dialoguer::Error) -> Self {
        GuseError::DialoguerError(err)
    }
}
