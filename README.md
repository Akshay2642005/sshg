# SSHG
**Empowering Seamless, Secure Remote Connections Everywhere**

![Rust](https://img.shields.io/badge/rust-100.0%25-orange.svg)
![Last Commit](https://img.shields.io/badge/last%20commit-today-brightgreen.svg)
![Languages](https://img.shields.io/badge/languages-1-blue.svg)

## Built with the tools and technologies:

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![TOML](https://img.shields.io/badge/TOML-9C4121?style=for-the-badge&logo=toml&logoColor=white)
![GitHub Actions](https://img.shields.io/badge/GitHub%20Actions-2088FF?style=for-the-badge&logo=github-actions&logoColor=white)

## Table of Contents
- [Overview](#overview)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Testing](#testing)

## Overview

sshg is a versatile developer tool that simplifies SSH host management, connection workflows, and automation. It integrates configuration parsing, an intuitive terminal UI, and remote server management into a cohesive experience for developers working across diverse environments.

### Why sshg?

This project aims to streamline SSH workflows and automate deployment processes. The core features include:

🎯 **Configuration Management**: Easily parse, add, and update SSH hosts within your `~/.ssh/config` file through interactive prompts.

🚀 **User-Friendly UI**: Select and connect to hosts seamlessly via a terminal-based interface, reducing manual command-line effort.

🔧 **Cross-Platform CI/CD**: Automate building, testing, packaging, and releasing binaries on Linux and Windows with integrated workflows.

🔒 **Secure SSH Connections**: Establish reliable remote sessions for server management and automation tasks.

⚙️ **Modular Architecture**: Integrate configuration, UI, and SSH connectivity for a cohesive developer experience.

## Getting Started

### Prerequisites

This project requires the following dependencies:

- **Programming Language**: Rust
- **Package Manager**: Cargo

### Installation

Build sshg from the source and install dependencies:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Akshay2642005/sshg
   ```

2. **Navigate to the project directory**:
   ```bash
   cd sshg
   ```

3. **Install the dependencies**:
   Using cargo:
   ```bash
   cargo build
   ```

## Usage

Run the project with:

Using cargo:
```bash
cargo run
```

## Testing

Sshg uses the Rust test framework. Run the test suite with:

Using cargo:
```bash
cargo test
```

---

*Built with ❤️ using Rust*
