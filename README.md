# Git Profile Manager

A simple and efficient command-line tool to manage multiple Git profiles. Switch between different Git user profiles (work, personal, etc.) with a single command.

## Features

- 🔄 Switch between multiple Git profiles quickly
- 🔒 Secure storage of Git credentials
- 📋 List all configured profiles
- 🛠️ Easy to install and use
- 🚀 Built with Rust for performance and reliability

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- Git (obviously!)

### Install from Source

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/git-profile-manager.git
   cd git-profile-manager
   ```

2. Build and install:

   ```bash
   cargo install --path .
   ```

   This will install the `git-profile-manager` binary to your Cargo bin directory (usually `~/.cargo/bin/`).

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

## How It Works

- Profiles are stored in `~/.config/git-profile-manager/config.json`
- Switching profiles updates:
  - Global Git username and email
  - Git credentials file (`~/.git-credentials`)

## Security Note

- Personal access tokens are stored in plain text in the config file
- The config file has standard user permissions (600)
- For enhanced security, consider using environment variables or a credential manager

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [clap](https://crates.io/crates/clap) for command-line parsing
- Inspired by the need for simpler Git profile management
