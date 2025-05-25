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

#### macOS

```bash
# Download the latest macOS binary from the release page
curl -LO https://github.com/jeonbyeongmin/guse/releases/latest/download/guse-x86_64-apple-darwin.tar.gz

# Extract the archive
tar -xzvf guse-x86_64-apple-darwin.tar.gz

# Make it executable
chmod +x guse-x86_64-apple-darwin

# (Optional) Move to your PATH
sudo mv guse-x86_64-apple-darwin /usr/local/bin/guse
```

#### Windows

1. Download `guse-x86_64-pc-windows-gnu.zip` from the [latest release page](https://github.com/jeonbyeongmin/guse/releases/latest).
2. Extract the zip file and place `guse-x86_64-pc-windows-gnu.exe` wherever you like.
3. (Optional) Rename the file to `guse.exe`.
4. (Optional) Add the folder to your system PATH.

#### Linux

```bash
# Download the latest Linux binary from the release page
curl -LO https://github.com/jeonbyeongmin/guse/releases/latest/download/guse-x86_64-unknown-linux-gnu.tar.gz

# Extract the archive
tar -xzvf guse-x86_64-unknown-linux-gnu.tar.gz

# Make it executable
chmod +x guse-x86_64-unknown-linux-gnu

# (Optional) Move to your PATH
sudo mv guse-x86_64-unknown-linux-gnu /usr/local/bin/guse
```

## Usage

After installation, you can run the following command in your terminal:

```bash
guse
```

<br>

## 🛠️ Usage

### Add a new profile

```bash
guse add personal
```

You'll be prompted to enter:

- Git user.name
- Git user.email
- SSH host alias (auto-detected from `~/.ssh/config`)

### Switch to a profile

```bash
# Select profile interactively
guse switch

# Specify profile name directly
guse switch personal
```

This will:

- Set the Git name/email for the current repository
- Rewire the remote origin URL to use the associated SSH host

### Show current Git configuration

```bash
guse show
```

### List available profiles

```bash
guse list
```

### List configured SSH hosts (from `~/.ssh/config`)

```bash
guse list-ssh
```

### Update a profile

```bash
# Select profile interactively
guse update

# Specify profile name directly
guse update work
```

### Delete a profile

```bash
# Select profile interactively
guse delete

# Specify profile name directly
guse delete work
```

<br>

## 🧠 How It Works

Profiles are stored in a `.toml` file at:

```
~/.guse-profiles.toml
```

Each profile includes:

```toml
[personal]
name = "byeongmin.jeon"
email = "jeonbyeongmin@personal.com"
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
