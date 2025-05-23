use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;

// Helper function to get the path to the compiled binary
fn guse_cmd() -> Command {
    Command::new(env!("CARGO_BIN_EXE_guse"))
}

// Helper function to set up a temporary config file
fn setup_test_config(temp_dir: &TempDir, initial_toml_content: &str) -> PathBuf {
    let config_file = temp_dir.child("profiles.toml");
    config_file
        .write_str(initial_toml_content)
        .expect("Failed to write initial config");
    config_file.path().to_path_buf()
}

#[test]
fn test_set_default_profile_success() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let initial_toml = r#"
[profiles.p1]
name = "User One"
email = "p1@example.com"
ssh_host = "github.com"

[profiles.p2]
name = "User Two"
email = "p2@example.com"
ssh_host = "gitlab.com"
"#;
    let config_path = setup_test_config(&temp_dir, initial_toml);

    let mut cmd = guse_cmd();
    cmd.arg("--config")
        .arg(config_path.to_str().unwrap())
        .arg("set-default")
        .arg("p1");

    cmd.assert().success().stdout(predicate::str::contains("Default profile set to 'p1'."));

    let final_toml_content = fs::read_to_string(config_path).expect("Failed to read config file after set-default");
    assert!(final_toml_content.contains("default_profile = \"p1\""));
    assert!(final_toml_content.contains("[profiles.p1]")); // Ensure profiles are still there
}

#[test]
fn test_set_default_profile_non_existent() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let initial_toml = r#"
[profiles.p1]
name = "User One"
email = "p1@example.com"
ssh_host = "github.com"
"#;
    let config_path = setup_test_config(&temp_dir, initial_toml);

    let mut cmd = guse_cmd();
    cmd.arg("--config")
        .arg(config_path.to_str().unwrap())
        .arg("set-default")
        .arg("non_existent_profile");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Profile 'non_existent_profile' not found."));
}

#[test]
fn test_unset_default_profile_success() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let initial_toml = r#"
default_profile = "p1"

[profiles.p1]
name = "User One"
email = "p1@example.com"
ssh_host = "github.com"
"#;
    let config_path = setup_test_config(&temp_dir, initial_toml);

    let mut cmd = guse_cmd();
    cmd.arg("--config")
        .arg(config_path.to_str().unwrap())
        .arg("unset-default");

    cmd.assert().success().stdout(predicate::str::contains("Default profile has been unset."));

    let final_toml_content = fs::read_to_string(config_path).expect("Failed to read config file after unset-default");
    assert!(!final_toml_content.contains("default_profile =")); // Key should be gone
    assert!(final_toml_content.contains("[profiles.p1]")); // Ensure profile is still there
}

#[test]
fn test_unset_default_profile_when_none_is_set() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let initial_toml = r#"
[profiles.p1]
name = "User One"
email = "p1@example.com"
ssh_host = "github.com"
"#; // No default_profile key initially
    let config_path = setup_test_config(&temp_dir, initial_toml);

    let mut cmd = guse_cmd();
    cmd.arg("--config")
        .arg(config_path.to_str().unwrap())
        .arg("unset-default");

    cmd.assert().success().stdout(predicate::str::contains("Default profile has been unset."));

    let final_toml_content = fs::read_to_string(config_path).expect("Failed to read config file after unset-default");
    assert!(!final_toml_content.contains("default_profile ="));
}

// Helper to run git commands in a specific directory
fn git_config_get(repo_dir: &Path, key: &str) -> String {
    let output = Command::new("git")
        .current_dir(repo_dir)
        .arg("config")
        .arg(key)
        .output()
        .expect("Failed to execute git config get");
    String::from_utf8(output.stdout).expect("Failed to parse git output").trim().to_string()
}

// Helper to init a git repo
fn init_git_repo(temp_dir: &TempDir) -> PathBuf {
    let repo_dir = temp_dir.child("test_repo");
    repo_dir.create_dir_all().expect("Failed to create repo dir");
    Command::new("git")
        .current_dir(repo_dir.path())
        .arg("init")
        .output()
        .expect("Failed to init git repo");
    repo_dir.path().to_path_buf()
}

#[test]
fn test_switch_uses_default_profile() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let repo_dir = init_git_repo(&temp_dir);
    let initial_toml = r#"
default_profile = "work"

[profiles.work]
name = "Work User"
email = "work@example.com"
ssh_host = "github.com"

[profiles.personal]
name = "Personal User"
email = "personal@example.com"
ssh_host = "gitlab.com"
"#;
    let config_path = setup_test_config(&temp_dir, initial_toml);

    let mut cmd = guse_cmd();
    cmd.current_dir(&repo_dir) // Run switch command in the context of the repo
        .arg("--config")
        .arg(config_path.to_str().unwrap())
        .arg("switch");

    cmd.assert().success().stdout(predicate::str::contains("Using default profile 'work'."));
    
    let email = git_config_get(&repo_dir, "user.email");
    assert_eq!(email, "work@example.com");
}

#[test]
fn test_switch_uses_specific_profile_over_default() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let repo_dir = init_git_repo(&temp_dir);
    let initial_toml = r#"
default_profile = "work"

[profiles.work]
name = "Work User"
email = "work@example.com"
ssh_host = "github.com"

[profiles.personal]
name = "Personal User"
email = "personal@example.com"
ssh_host = "gitlab.com"
"#;
    let config_path = setup_test_config(&temp_dir, initial_toml);

    let mut cmd = guse_cmd();
    cmd.current_dir(&repo_dir)
        .arg("--config")
        .arg(config_path.to_str().unwrap())
        .arg("switch")
        .arg("personal");

    cmd.assert().success().stdout(predicate::str::contains("Changing Git configuration for 'personal'..."));
    
    let email = git_config_get(&repo_dir, "user.email");
    assert_eq!(email, "personal@example.com");
}

#[test]
fn test_switch_prompts_when_no_default_and_no_arg() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let repo_dir = init_git_repo(&temp_dir); // Init repo, though we won't check git config change here
    let initial_toml = r#"
[profiles.work]
name = "Work User"
email = "work@example.com"
ssh_host = "github.com"
"#;
    let config_path = setup_test_config(&temp_dir, initial_toml);

    let mut cmd = guse_cmd();
    cmd.current_dir(&repo_dir)
        .arg("--config")
        .arg(config_path.to_str().unwrap())
        .arg("switch");
    
    // Testing interactive prompts is tricky. We'll check if it prints the prompt.
    // Actual selection cannot be easily automated here without specific PTY tools.
    cmd.assert().success().stdout(predicate::str::contains("Select profile to switch to"));
}


#[test]
fn test_show_with_default_profile_set() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let initial_toml = r#"
default_profile = "testdefault"

[profiles.testdefault]
name = "Default Test User"
email = "default@example.com"
ssh_host = "example.com"
"#;
    let config_path = setup_test_config(&temp_dir, initial_toml);

    let mut cmd = guse_cmd();
    cmd.arg("--config")
        .arg(config_path.to_str().unwrap())
        .arg("show");

    cmd.assert().success().stdout(
        predicate::str::contains("Default Profile: testdefault")
        .and(predicate::str::contains("(set globally)"))
    );
}

#[test]
fn test_show_with_no_default_profile_set() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let initial_toml = r#"
[profiles.someprofile]
name = "Some User"
email = "some@example.com"
ssh_host = "example.com"
"#; // No default_profile key
    let config_path = setup_test_config(&temp_dir, initial_toml);

    let mut cmd = guse_cmd();
    cmd.arg("--config")
        .arg(config_path.to_str().unwrap())
        .arg("show");

    cmd.assert().success().stdout(predicate::str::contains("Default Profile: None"));
}
