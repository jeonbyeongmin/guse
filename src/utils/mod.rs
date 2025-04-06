use chrono::Local;
use regex;
use std::fs;
use std::path::PathBuf;

use crate::error::GitSwitchError;

pub fn validate_email(email: &str) -> Result<(), GitSwitchError> {
    let email_regex = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .map_err(|e| GitSwitchError::ValidationError(e.to_string()))?;

    if !email_regex.is_match(email) {
        return Err(GitSwitchError::ValidationError(
            "Invalid email format.".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_ssh_host(host: &str) -> Result<(), GitSwitchError> {
    if host.trim().is_empty() {
        return Err(GitSwitchError::ValidationError(
            "SSH host cannot be empty.".to_string(),
        ));
    }
    Ok(())
}

pub fn backup_config_file(config_path: &PathBuf) -> Result<(), GitSwitchError> {
    if config_path.exists() {
        let backup_path =
            config_path.with_extension(format!("backup.{}", Local::now().format("%Y%m%d_%H%M%S")));
        fs::copy(config_path, &backup_path).map_err(|e| {
            GitSwitchError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to backup configuration file: {}", e),
            ))
        })?;
    }
    Ok(())
}
