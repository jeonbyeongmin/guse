# guse

> ⚡ Effortless Git user profile switching — powered by Rust.

`guse` is a fast and intuitive command-line tool to manage and switch between multiple Git identities across projects.  
It handles `user.name`, `user.email`, and even your SSH remote configuration automatically, using simple profile-based rules.

<br>

## Managing Git Identities

Working across different Git repositories often requires switching between multiple identities.  
Manually updating `user.name`, `user.email`, and remote URLs can be repetitive and error-prone.  

`guse` automates these steps using profile-based configuration, so each repository uses the correct settings without manual edits.

<br>

## ✨ Features

- 🔁 Switch Git `user.name` / `user.email` per repository  
- 🔐 Automatically update `git remote` with SSH host  
- 📝 Add, update, and remove profiles interactively  
- 💾 Profile settings stored in a simple TOML file  

<br>

## 🚀 Quick Start

### Install

```bash
cargo install --path .
```

Or build and copy manually:

```bash
cargo build --release
cp ./target/release/guse ~/bin/guse
```

Add to your shell PATH if needed:

```bash
echo 'export PATH="$HOME/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

<br>

## 🛠️ Usage

### Add a new profile

```bash
guse add personal
```

You’ll be prompted to enter:

- Git user.name  
- Git user.email  
- SSH host alias (auto-detected from `~/.ssh/config`)  

<br>

### Switch to a profile

```bash
guse switch personal
```

This will:

- Set the Git name/email for the current repository  
- Rewire the remote origin URL to use the associated SSH host  

<br>

### Show current Git configuration

```bash
guse show
```

<br>

### List available profiles

```bash
guse list
```

<br>

### List configured SSH hosts (from `~/.ssh/config`)

```bash
guse list-ssh
```

<br>

### Update a profile

```bash
guse update work
```

<br>

## 🧠 How It Works

Profiles are stored in a `.toml` file at:

```
~/.git-switch-profiles.toml
```

Each profile includes:

```toml
[personal]
name = "Hapbee"
email = "hapbee@personal.com"
ssh_host = "github-personal"
```

The `ssh_host` must match a `Host` alias in your `~/.ssh/config` file.

<br>

## 🔐 Example `~/.ssh/config`

```ssh
Host github-personal
  HostName github.com
  User git
  IdentityFile ~/.ssh/id_ed25519_personal

Host github-work
  HostName github.com
  User git
  IdentityFile ~/.ssh/id_ed25519_work
```

<br>

## ❤️ Contributing

Contributions are welcome!  
Feel free to open issues or PRs.

<br>

## 📜 License

MIT License © 2025 Jeon Byeongmin
