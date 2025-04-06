use std::fmt;

#[derive(Debug)]
pub enum GitSwitchError {
    IoError(std::io::Error),
    TomlError(toml::de::Error),
    GitError(String),
    ValidationError(String),
    ConfigError(String),
    DialoguerError(dialoguer::Error),
}

impl fmt::Display for GitSwitchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GitSwitchError::IoError(e) => write!(f, "IO Error: {}", e),
            GitSwitchError::TomlError(e) => write!(f, "TOML Parsing Error: {}", e),
            GitSwitchError::GitError(e) => write!(f, "Git Command Error: {}", e),
            GitSwitchError::ValidationError(e) => write!(f, "Validation Error: {}", e),
            GitSwitchError::ConfigError(e) => write!(f, "Configuration Error: {}", e),
            GitSwitchError::DialoguerError(e) => write!(f, "Interactive Input Error: {}", e),
        }
    }
}

impl std::error::Error for GitSwitchError {}

impl From<std::io::Error> for GitSwitchError {
    fn from(err: std::io::Error) -> Self {
        GitSwitchError::IoError(err)
    }
}

impl From<toml::de::Error> for GitSwitchError {
    fn from(err: toml::de::Error) -> Self {
        GitSwitchError::TomlError(err)
    }
}

impl From<toml::ser::Error> for GitSwitchError {
    fn from(err: toml::ser::Error) -> Self {
        GitSwitchError::ConfigError(format!("TOML Serialization Error: {}", err))
    }
}

impl From<crate::config::ConfigError> for GitSwitchError {
    fn from(err: crate::config::ConfigError) -> Self {
        GitSwitchError::ConfigError(err.to_string())
    }
}

impl From<crate::git::GitError> for GitSwitchError {
    fn from(err: crate::git::GitError) -> Self {
        GitSwitchError::GitError(err.to_string())
    }
}

impl From<dialoguer::Error> for GitSwitchError {
    fn from(err: dialoguer::Error) -> Self {
        GitSwitchError::DialoguerError(err)
    }
}
