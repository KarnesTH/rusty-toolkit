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
rusty-toolkit password-gen
rusty-toolkit download
rusty-toolkit system-monitor
```

## Dependencies
- clap
- tokio
- serde
- ring
- chrono

## Contributing
1. Fork repository
2. Create feature branch
3. Commit changes
4. Push and create PR

## License
MIT License
