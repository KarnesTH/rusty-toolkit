# Rusty Toolkit 🦀

## Overview
A modular CLI utility suite written in Rust for file management, password tools, downloads, and system monitoring.

## Features
- 📁 Advanced file search
- 🔐 Secure password management
- 📥 Parallel download manager
- 💻 Real-time system monitoring

## Prerequisites
- Rust 1.70.0+
- Cargo package manager

## Installation
```bash
git clone https://github.com/KarnesTH/rusty-toolkit.git
cd rusty-toolkit
cargo build --release
```

## Usage
```bash
# General syntax
rusty-toolkit  [options]

# Available commands
rusty-toolkit file-search
rusty-toolkit password
rusty-toolkit download
rusty-toolkit system-monitor
```

### File Search

```bash
# Search for files in the interactive mode
rusty-toolkit file-search

# Search for files with a specific name
rusty-toolkit file-search -n <file_name>
```

### Password

Passwords have the following options:

- Generate - To generate a random password
- Manage - To manage passwords

```bash
# Generate a random password in interactive mode
rusty-toolkit password generate
# Generate a random password with a specific length
rusty-toolkit password generate -l <length>

# Manage passwords in interactive mode
# Add a new pasword
rusty-toolkit password manage add
# List all passwords
rusty-toolkit password manage list
# Remove a password
rusty-toolkit password manage remove
# Update a password
rusty-toolkit password manage update
# Search for a password
rusty-toolkit password manage search
# Show a password
rusty-toolkit password manage show
# Export passwords
rusty-toolkit password manage export

# Manage passwords with a specific input
# Add a new password
rusty-toolkit password manage add -s <service> -u <username> -p <password> --url <url> -n <notes>
# Remove a password
rusty-toolkit password manage remove -i <id>
# Update a password
rusty-toolkit password manage update -i <id> -s <service> -u <username> -p <password> --url <url> -n <notes>
# Search for a password
rusty-toolkit password manage search -q <query>
# Show a password
rusty-toolkit password manage show -i <id>
# Export passwords
rusty-toolkit password manage export -p <path>
```

## License
MIT License
