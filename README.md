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
# Download latest release
curl -L -o guse https://github.com/jeonbyeongmin/guse/releases/latest/download/guse-macos

# Make it executable
chmod +x guse

# Add to PATH (optional)
sudo mv guse /usr/local/bin/
```

#### Windows

1. Download `guse-windows.exe` from the [latest release page](https://github.com/jeonbyeongmin/guse/releases/latest)
2. Save the file to your desired location
3. Rename the file to `guse.exe` (optional)
4. Add the directory to your system's PATH environment variable (optional)

#### Linux

```bash
# Download latest release
curl -L -o guse https://github.com/jeonbyeongmin/guse/releases/latest/download/guse-linux

# Make it executable
chmod +x guse

# Add to PATH (optional)
sudo mv guse /usr/local/bin/
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

### Set a default profile

You can set a global default profile that `guse` will use for certain operations, like `guse switch` when no profile name is specified.

```bash
guse set-default <profile-name>
```
Replace `<profile-name>` with the name of one of your existing profiles (e.g., `guse set-default personal`).

### Unset the default profile

To remove the global default profile setting:

```bash
guse unset-default
```

### Switch to a profile

```bash
# Automatically uses the default profile if set, otherwise prompts for selection
guse switch

# Specify profile name directly, overriding any default
guse switch personal
```

This will:

- Set the Git name/email for the current repository.
- Rewire the remote origin URL to use the associated SSH host for the selected profile.

If a default profile is configured, running `guse switch` without a profile name will automatically switch to the default. Otherwise, it will prompt you to select a profile from the list.

### Show current Git configuration

```bash
guse show
```
Displays the current Git `user.name` and `user.email` for the repository, the `guse` profile it matches (if any), and the globally configured default `guse` profile (if set).

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
~/.git-switch-profiles.toml
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
