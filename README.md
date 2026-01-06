# 🐟 KOI TERMINAL

> *Little Kois for your terminal, yay!*

<div align="center">

![Koi Terminal Demo](https://via.placeholder.com/800x400/0d1117/58a6ff?text=🐟+Koi+Terminal+Demo)

</div>

<div align="center">

[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge&logo=opensource&logoColor=white)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-100%25-orange.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/github/actions/workflow/status/siraprem/koi_terminal/ci.yml?style=for-the-badge&logo=github&logoColor=white)](https://github.com/siraprem/koi_terminal/actions)
[![Last Commit](https://img.shields.io/github/last-commit/siraprem/koi_terminal?style=for-the-badge&logo=git&logoColor=white)](https://github.com/siraprem/koi_terminal/commits/master)

</div>

---

## 🚀 Built with the tools and technologies

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Cargo](https://img.shields.io/badge/Cargo-8B4513?style=for-the-badge&logo=rust&logoColor=white)](https://doc.rust-lang.org/cargo/)
[![Terminal](https://img.shields.io/badge/Terminal-4D4D4D?style=for-the-badge&logo=windows-terminal&logoColor=white)](https://github.com/microsoft/terminal)
[![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)](https://www.linux.org/)

</div>

---

## 📖 About

Koi Terminal is a delightful terminal application that brings adorable little kois (Japanese carp fish) to your command-line environment. Built with Rust for maximum performance and reliability.

### ✨ Features

- 🐟 Smooth koi swimming animations
- 🎨 Colorful and customizable interface  
- ⚡ High performance with Rust
- 🔧 Flexible configuration
- 🌐 Cross-platform support
- 📦 Single executable with no dependencies

---

## 🐧 Installation

### 📋 Prerequisites

Make sure you have Rust installed on your system:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 🔧 Installation by Linux Distribution

<details>
<summary><b>🟠 Ubuntu / Debian / Mint</b></summary>

```bash
# Install via cargo
cargo install --git https://github.com/siraprem/koi_terminal

# Or compile from source
sudo apt update
sudo apt install build-essential
git clone https://github.com/siraprem/koi_terminal.git
cd koi_terminal
cargo build --release
sudo cp target/release/koi_terminal /usr/local/bin/
```

</details>

<details>
<summary><b>🔴 Fedora / CentOS / RHEL</b></summary>

```bash
# Install via cargo
cargo install --git https://github.com/siraprem/koi_terminal

# Or compile from source
sudo dnf groupinstall "Development Tools"
sudo dnf install rust cargo
git clone https://github.com/siraprem/koi_terminal.git
cd koi_terminal
cargo build --release
sudo cp target/release/koi_terminal /usr/local/bin/
```

</details>

<details>
<summary><b>🟢 openSUSE</b></summary>

```bash
# Install via cargo
cargo install --git https://github.com/siraprem/koi_terminal

# Or compile from source
sudo zypper install -t pattern devel_basis
sudo zypper install rust cargo
git clone https://github.com/siraprem/koi_terminal.git
cd koi_terminal
cargo build --release
sudo cp target/release/koi_terminal /usr/local/bin/
```

</details>

<details>
<summary><b>🟡 Arch Linux / Manjaro</b></summary>

```bash
# Install via cargo
cargo install --git https://github.com/siraprem/koi_terminal

# Or compile from source
sudo pacman -S base-devel rust
git clone https://github.com/siraprem/koi_terminal.git
cd koi_terminal
cargo build --release
sudo cp target/release/koi_terminal /usr/local/bin/
```

</details>

<details>
<summary><b>🟣 Gentoo</b></summary>

```bash
# Install via cargo
cargo install --git https://github.com/siraprem/koi_terminal

# Or compile from source
emerge --ask dev-lang/rust
git clone https://github.com/siraprem/koi_terminal.git
cd koi_terminal
cargo build --release
sudo cp target/release/koi_terminal /usr/local/bin/
```

</details>

<details>
<summary><b>🔵 Alpine Linux</b></summary>

```bash
# Install via cargo
cargo install --git https://github.com/siraprem/koi_terminal

# Or compile from source
sudo apk add build-base rust cargo
git clone https://github.com/siraprem/koi_terminal.git
cd koi_terminal
cargo build --release
sudo cp target/release/koi_terminal /usr/local/bin/
```

</details>

### 🎯 Quick Install (All Distributions)

```bash
# Clone and install in one line
git clone https://github.com/siraprem/koi_terminal.git && cd koi_terminal && cargo install --path .
```

---

## 🎮 Usage

### Basic Commands

```bash
# Start Koi Terminal
koi_terminal

# Run with custom settings
koi_terminal --config ~/.config/koi/config.toml

# Demo mode
koi_terminal --demo

# Show help
koi_terminal --help
```

### ⚙️ Configuration

Create a configuration file at `~/.config/koi/config.toml`:

```toml
[display]
fps = 60
width = 80
height = 24

[koi]
count = 5
speed = 1.0
colors = ["red", "blue", "yellow", "green", "purple"]

[behavior]
auto_spawn = true
collision_detection = true
```

---

## 🎨 Screenshots

<div align="center">

| Default Mode | Colorful Mode | Demo Mode |
|-------------|---------------|-----------|
| ![Default](https://via.placeholder.com/250x150/0d1117/58a6ff?text=🐟+Default) | ![Colorful](https://via.placeholder.com/250x150/0d1117/ff6b6b?text=🎨+Colorful) | ![Demo](https://via.placeholder.com/250x150/0d1117/4ecdc4?text=✨+Demo) |

</div>

---

## 🏗️ Development

### 🛠️ Building from Source

```bash
# Clone the repository
git clone https://github.com/siraprem/koi_terminal.git
cd koi_terminal

# Run tests
cargo test

# Build and run in debug mode
cargo run

# Build in release mode
cargo build --release
```

### 📁 Project Structure

```
koi_terminal/
├── src/
│   ├── main.rs          # Main entry point
│   ├── koi/             # Koi module
│   ├── terminal/        # Terminal interface
│   └── config/          # Configuration system
├── tests/               # Automated tests
├── docs/                # Documentation
└── examples/            # Usage examples
```

---

## 🤝 Contributing

Contributions are very welcome! Here's how you can help:

1. **Fork** the project
2. Create a **branch** for your feature (`git checkout -b feature/AmazingFeature`)
3. **Commit** your changes (`git commit -m 'Add some AmazingFeature'`)
4. **Push** to the branch (`git push origin feature/AmazingFeature`)
5. Open a **Pull Request**

### 📝 Guidelines

- Keep code clean and well documented
- Add tests for new features
- Follow Rust conventions (`cargo fmt` and `cargo clippy`)
- Update documentation when necessary

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- 🦀 **Rust Community** - For the amazing language
- 🐟 **Koi Enthusiasts** - For the inspiration
- 💻 **Terminal Artists** - For examples and ideas
- 🌟 **Contributors** - For making this project better

---

<div align="center">

**[⬆ Back to top](#-koi-terminal)**

Made with 💙 by [Samira](https://github.com/siraprem)

*May the kois bring tranquility to your terminal!* 🐟✨

</div>
