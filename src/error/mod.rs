use thiserror::Error;

#[derive(Debug, Error)]
pub enum GuseError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("TOML Parsing Error: {0}")]
    TomlError(#[from] toml::de::Error),
    
    #[error("Git Command Error: {0}")]
    GitError(String),
    
    #[error("Validation Error: {0}")]
    ValidationError(String),
    
    #[error("Configuration Error: {0}")]
    ConfigError(String),
    
    #[error("Interactive Input Error: {0}")]
    DialoguerError(#[from] dialoguer::Error),
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

#[cfg(test)]
mod tests;


