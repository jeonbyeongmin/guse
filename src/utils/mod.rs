use regex;
use std::path::PathBuf;

use crate::error::GuseError;

pub fn validate_email(email: &str) -> Result<(), GuseError> {
    regex::Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$")
        .map_err(|e| GuseError::ValidationError(e.to_string()))?;

    if !email.contains('@') {
        return Err(GuseError::ValidationError(
            "Email address must contain '@' character.".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_ssh_host(host: &str) -> Result<(), GuseError> {
    if host.is_empty() {
        return Err(GuseError::ValidationError(
            "SSH host cannot be empty.".to_string(),
        ));
    }
    Ok(())
}

pub fn get_ssh_config_path() -> Result<PathBuf, GuseError> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        GuseError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find home directory.".to_string(),
        ))
    })?;
    Ok(home_dir.join(".ssh").join("config"))
}

pub fn backup_config_file(config_path: &PathBuf) -> Result<(), GuseError> {
    if config_path.exists() {
        let backup_path = config_path.with_extension("config.bak");
        std::fs::copy(config_path, &backup_path).map_err(|e| {
            GuseError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to backup configuration file: {}", e),
            ))
        })?;
    }
    Ok(())
}
