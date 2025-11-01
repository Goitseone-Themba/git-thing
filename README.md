# Git Thing

A simple and efficient command-line tool to manage multiple Git profiles. Switch between different Git user profiles (work, personal, etc.) with a single command.

## Features

- 🔄 Switch between multiple Git profiles quickly
- 🔒 Secure storage of Git credentials
- 📋 List all configured profiles
- ✏️ Update existing profiles
- ❌ Remove unused profiles
- 🛠️ Easy to install and use
- 🚀 Built with Rust for performance and reliability

## Installation

You can install git-thing in one of two ways: download the prebuilt binary, or build from source.

### Download the Binary (Linux / macOS)

Download the released binary and place it in /usr/bin so it's available system-wide:

```bash
# download the binary (use the raw file URL)
curl -L -o git-thing https://raw.githubusercontent.com/Goitseone-Themba/git-thing/master/git-thing

# make it executable
chmod +x git-thing

# move to /usr/bin (requires sudo)
sudo mv git-thing /usr/bin/git-thing

# (optional) ensure ownership and permissions
sudo chown root:root /usr/bin/git-thing
sudo chmod 755 /usr/bin/git-thing
```

After this the `git-thing` command will be available from your shell. (If you prefer a user-scoped location, move the binary into `~/bin` or `$HOME/.local/bin` instead.)

### Build from Source

#### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- Git (obviously!)
- A GitHub personal access token (recommended: classic token with only full repo access) — see: https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-personal-access-token-classic

#### Build and Install

1. Clone the repository:

   ```bash
   git clone https://github.com/Goitseone-Themba/git-thing.git
   cd git-thing
   ```

2. Build and install:

   ```bash
   cargo install --path .
   ```

   This will install the `git-thing` binary to your Cargo bin directory (usually `~/.cargo/bin/`).

3. (Optional) Add the Cargo bin directory to your PATH if it's not already:

   ```bash
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc
   source ~/.bashrc  # or ~/.zshrc
   ```

## Usage

### Add a New Profile

```bash
git-thing add \
  --profilename work \
  --username work-username \
  --email work@example.com \
  --personal-access-key your-github-token
```

Or using short flags:

```bash
git-thing add -p work -u work-username -e work@example.com -k your-github-token
```

### List All Profiles

```bash
git-thing list
```

### Switch to a Profile

```bash
git-thing switch --profile work
```

Or using short flag:

```bash
git-thing switch -p work
```

### Update an Existing Profile

Update any field of an existing profile. Only the fields you specify will be updated.

```bash
# Update just the email
git-thing update --profilename work --email new.email@example.com

# Update multiple fields
git-thing update -p work -u newusername -k newtoken123
```

### Delete a Profile

Remove a profile that you no longer need:

```bash
git-thing delete --profilename oldprofile
```
Or using short flag:

```bash
git-thing delete -p oldprofile
```

## How It Works

- Profiles are stored in `~/.config/git-thing/config.json`
- Switching profiles updates:
  - Global Git username and email
  - Git credentials file (`~/.git-credentials`)
- The config file is updated when you add, update, or delete profiles

## Security Note

- The config file has standard user permissions (600)
- Personal access tokens are stored in plain text in the config file

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [clap](https://crates.io/crates/clap) for command-line parsing
- Inspired by the need for simpler Git profile management
