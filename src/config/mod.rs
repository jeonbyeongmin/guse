use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct ConfigError(pub String);

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError(format!("IO Error: {}", err))
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(err: toml::ser::Error) -> Self {
        ConfigError(format!("TOML Serialization Error: {}", err))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub ssh_host: String,
}

pub type ProfileMap = HashMap<String, Profile>;

lazy_static! {
    static ref CONFIG_LOCK: Mutex<()> = Mutex::new(());
}

pub struct Config {
    pub path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".git-switch-profiles.toml");
        Self { path }
    }

    pub fn load_profiles(&self) -> Result<ProfileMap, ConfigError> {
        if !self.path.exists() {
            return Ok(HashMap::new());
        }

        let contents = fs::read_to_string(&self.path)
            .map_err(|e| ConfigError(format!("Cannot read configuration file: {}", e)))?;

        toml::from_str(&contents).map_err(|e| ConfigError(format!("TOML Parsing Error: {}", e)))
    }

    pub fn save_profiles(&self, profiles: &ProfileMap) -> Result<(), ConfigError> {
        self.backup()?;

        let _lock = CONFIG_LOCK
            .lock()
            .map_err(|_| ConfigError("Failed to acquire configuration file lock".to_string()))?;

        let updated = toml::to_string_pretty(profiles)?;
        fs::write(&self.path, updated)?;

        Ok(())
    }

    pub fn add_profile(&self, name: String, profile: Profile) -> Result<(), ConfigError> {
        let mut profiles = self.load_profiles()?;
        profiles.insert(name, profile);
        self.save_profiles(&profiles)?;
        Ok(())
    }

    pub fn update_profile(&self, name: &str, profile: Profile) -> Result<(), ConfigError> {
        let mut profiles = self.load_profiles()?;
        if !profiles.contains_key(name) {
            return Err(ConfigError(format!("Profile '{}' does not exist.", name)));
        }
        profiles.insert(name.to_string(), profile);
        self.save_profiles(&profiles)?;
        Ok(())
    }

    fn backup(&self) -> Result<(), ConfigError> {
        if self.path.exists() {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let backup_path = self.path.with_extension(format!("toml.{}", timestamp));
            fs::copy(&self.path, backup_path)?;
        }
        Ok(())
    }
}
