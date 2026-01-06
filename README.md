# 🐟 KOI TERMINAL

> *Little Kois for your terminal, yay!*

<div align="center">

[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge&logo=opensource&logoColor=white)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-100%25-orange.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
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

### 🔧 Installation Any Linux Distribution

```bash
# Install via cargo
cargo install --git https://github.com/siraprem/koi_terminal
```


## 🎮 Usage

### Basic Commands

```bash
# Start Koi Terminal
koi-screensaver
```

## 🙀 If the command is not found

### 🔧 Command not found after install?

After running:

```bash
cargo install --git https://github.com/siraprem/koi_terminal
```

If the command is not found, it means Cargo’s bin directory is not in your PATH.

Cargo installs binaries in:

```bash
~/.cargo/bin
```
Add this directory to your PATH depending on your shell:

### 🐚 Bash / Zsh
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```
### 🐟 Fish

```fish
fish_add_path ~/.cargo/bin
Then restart the terminal.
```
### 🧪 Check if it worked
Run:

```bash
which koi-screensaver
```
You should see something like:
```swift
/home/youruser/.cargo/bin/koi-screensaver
```
Now you can run:

```bash
koi-screensaver
```

---

## 🏗️ Development

### 🛠️ Building from Source

```bash
# Clone the repository
git clone https://github.com/siraprem/koi_terminal.git
cd koi_terminal

# Run tests
cargo test
```



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

Made with 💙 by [Ley](https://github.com/siraprem)

*May the kois bring tranquility to your terminal!* 🐟✨

</div>
