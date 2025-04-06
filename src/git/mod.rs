use log::info;
use std::process::Command;

#[derive(Debug)]
pub struct GitError(pub String);

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for GitError {}

pub struct Git {
    config: GitConfig,
}

#[derive(Debug)]
pub struct GitConfig {
    pub user_name: String,
    pub user_email: String,
    pub remote_url: String,
}

impl Git {
    pub fn new() -> Self {
        Self {
            config: GitConfig {
                user_name: String::new(),
                user_email: String::new(),
                remote_url: String::new(),
            },
        }
    }

    pub fn set_config(&mut self, name: &str, email: &str) -> Result<(), GitError> {
        info!("Setting Git username: {}", name);
        self.execute_command(&["config", "user.name", name])?;

        info!("Setting Git email: {}", email);
        self.execute_command(&["config", "user.email", email])?;

        self.config.user_name = name.to_string();
        self.config.user_email = email.to_string();
        Ok(())
    }

    pub fn set_remote(&mut self, host: &str, user: &str, repo: &str) -> Result<(), GitError> {
        let remote_url = format!("git@{}:{}/{}.git", host, user, repo);
        info!("Setting Git remote URL: {}", remote_url);
        self.execute_command(&["remote", "set-url", "origin", &remote_url])?;

        self.config.remote_url = remote_url;
        Ok(())
    }

    pub fn get_current_config(&self) -> Result<GitConfig, GitError> {
        let user_name = self.execute_command(&["config", "user.name"])?;
        let user_email = self.execute_command(&["config", "user.email"])?;

        // Try to get remote URL, but return empty string if origin doesn't exist
        let remote_url = match self.execute_command(&["remote", "get-url", "origin"]) {
            Ok(url) => url,
            Err(_) => String::new(),
        };

        Ok(GitConfig {
            user_name,
            user_email,
            remote_url,
        })
    }

    pub fn parse_origin_url(&self) -> Result<(String, String), GitError> {
        // Try to get remote URL, but return error if origin doesn't exist
        let url = match self.execute_command(&["remote", "get-url", "origin"]) {
            Ok(url) => url,
            Err(e) => {
                return Err(GitError(format!(
                    "No remote 'origin' found. Please add a remote repository first: {}",
                    e
                )))
            }
        };

        if url.is_empty() {
            return Err(GitError("Remote 'origin' URL is empty".to_string()));
        }

        if url.starts_with("git@") {
            let parts: Vec<&str> = url.split(':').collect();
            if parts.len() == 2 {
                let path = parts[1].trim_end_matches(".git");
                let mut segments = path.split('/');
                let user = segments
                    .next()
                    .ok_or_else(|| GitError("Invalid remote repository URL format".to_string()))?
                    .to_string();
                let repo = segments
                    .next()
                    .ok_or_else(|| GitError("Invalid remote repository URL format".to_string()))?
                    .to_string();
                return Ok((user, repo));
            }
        }

        if url.starts_with("https://") {
            let parts: Vec<&str> = url.split('/').collect();
            if parts.len() >= 2 {
                let user = parts[parts.len() - 2].to_string();
                let repo = parts[parts.len() - 1].trim_end_matches(".git").to_string();
                return Ok((user, repo));
            }
        }

        Err(GitError(
            "Unsupported remote repository URL format".to_string(),
        ))
    }

    fn execute_command(&self, args: &[&str]) -> Result<String, GitError> {
        let output = Command::new("git")
            .args(args)
            .output()
            .map_err(|e| GitError(format!("Failed to execute Git command: {}", e)))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(GitError(error.to_string()));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}
