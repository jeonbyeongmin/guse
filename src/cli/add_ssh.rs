use clap::Parser;
use colored::*;
use std::fs::OpenOptions;
use std::io::Write;

use crate::error::GuseError;
use crate::utils::get_ssh_config_path;

#[derive(Parser, Debug)]
#[command(about = "Add a new SSH host to ~/.ssh/config")]
pub struct AddSshCommand;

impl AddSshCommand {
    pub fn execute(&self) -> Result<(), GuseError> {
        use dialoguer::Input;


        let ssh_config = get_ssh_config_path()?;

        println!("\n{}", "🔑 SSH 호스트 추가".cyan().bold());
        println!("{}", "=".repeat(40).cyan());

        let host: String = Input::new()
            .with_prompt("Host 별칭 (예: myserver)")
            .interact_text()?;
        let hostname: String = Input::new()
            .with_prompt("HostName (예: 192.168.0.1 또는 github.com)")
            .interact_text()?;
        let user: String = Input::new()
            .with_prompt("User (예: ubuntu)")
            .interact_text()?;
        let port: String = Input::new()
            .with_prompt("Port (기본값: 22)")
            .default("22".to_string())
            .interact_text()?;

        // ~/.ssh 디렉토리에서 id_*로 시작하는 파일을 찾아서 선택지로 제공
        let ssh_dir = shellexpand::tilde("~/.ssh").to_string();
        let mut identity_files = match std::fs::read_dir(&ssh_dir) {
            Ok(entries) => entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| {
                    if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                        name.starts_with("id_") && !name.ends_with(".pub") && p.is_file()
                    } else {
                        false
                    }
                })
                .map(|p| p.to_string_lossy().to_string())
                .collect::<Vec<_>>(),
            Err(_) => vec![],
        };
        identity_files.push("새로 생성".to_string());
        identity_files.push("직접 입력".to_string());

        let identity_file = if identity_files.len() > 2 {
            use dialoguer::Select;
            let selection = Select::new()
                .with_prompt("IdentityFile을 선택하세요 (또는 '새로 생성', '직접 입력' 선택)")
                .items(&identity_files)
                .default(0)
                .interact()?;
            if identity_files[selection] == "직접 입력" {
                Input::new()
                    .with_prompt("IdentityFile 경로를 입력하세요 (예: ~/.ssh/id_rsa)")
                    .default("~/.ssh/id_rsa".to_string())
                    .interact_text()?
            } else if identity_files[selection] == "새로 생성" {
                // Host 별칭을 id_ 접두사로 사용
                let ssh_dir = shellexpand::tilde("~/.ssh").to_string();
                let new_key_path = format!("{}/id_{}", ssh_dir, host);
                let expanded_new_key_path = shellexpand::tilde(&new_key_path).to_string();
                if !std::path::Path::new(&expanded_new_key_path).exists() {
                    println!("{} SSH 키를 생성합니다...", "🔑".yellow());
                    let output = std::process::Command::new("ssh-keygen")
                        .arg("-t").arg("rsa")
                        .arg("-b").arg("4096")
                        .arg("-f").arg(&expanded_new_key_path)
                        .arg("-N").arg("")
                        .output();
                    match output {
                        Ok(out) if out.status.success() => {
                            println!("{} SSH 키가 생성되었습니다: {}", "✅".green(), expanded_new_key_path);
                        }
                        Ok(out) => {
                            eprintln!("{} ssh-keygen 실패: {}", "❌".red(), String::from_utf8_lossy(&out.stderr));
                        }
                        Err(e) => {
                            eprintln!("{} ssh-keygen 실행 오류: {}", "❌".red(), e);
                        }
                    }
                } else {
                    println!("{} 이미 해당 경로에 키가 존재합니다: {}", "⚠️".yellow(), expanded_new_key_path);
                }
                new_key_path
            } else {
                identity_files[selection].clone()
            }
        } else {
            Input::new()
                .with_prompt("IdentityFile (예: ~/.ssh/id_rsa)")
                .default("~/.ssh/id_rsa".to_string())
                .interact_text()?
        };

        // SSH 키가 없으면 자동 생성
        let expanded_identity_file = shellexpand::tilde(&identity_file).to_string();
        if !std::path::Path::new(&expanded_identity_file).exists() {
            println!("{} SSH 키가 존재하지 않습니다. 자동으로 생성합니다...", "🔑".yellow());
            let output = std::process::Command::new("ssh-keygen")
                .arg("-t").arg("rsa")
                .arg("-b").arg("4096")
                .arg("-f").arg(&expanded_identity_file)
                .arg("-N").arg("")
                .output();
            match output {
                Ok(out) if out.status.success() => {
                    println!("{} SSH 키가 생성되었습니다: {}", "✅".green(), expanded_identity_file);
                }
                Ok(out) => {
                    eprintln!("{} ssh-keygen 실패: {}", "❌".red(), String::from_utf8_lossy(&out.stderr));
                }
                Err(e) => {
                    eprintln!("{} ssh-keygen 실행 오류: {}", "❌".red(), e);
                }
            }
        }

        let entry = format!(
            "\nHost {}\n    HostName {}\n    User {}\n    Port {}\n    IdentityFile {}\n",
            host, hostname, user, port, identity_file
        );

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&ssh_config)?;
        file.write_all(entry.as_bytes())?;

        println!("\n{} {}에 SSH 호스트가 추가되었습니다!", "✅".green(), ssh_config.display());
        Ok(())
    }
}
