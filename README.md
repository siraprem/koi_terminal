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

## 📖 Sobre o Projeto

Koi Terminal é uma aplicação de terminal encantadora que traz pequenos kois (peixes carpa japoneses) para o seu ambiente de linha de comando. Desenvolvido em Rust para máxima performance e confiabilidade.

### ✨ Características

- 🐟 Animações fluidas de kois nadando
- 🎨 Interface colorida e customizável  
- ⚡ Alto performance com Rust
- 🔧 Configuração flexível
- 🌐 Suporte multiplataforma
- 📦 Executável único sem dependências

---

## 🐧 Instalação

### 📋 Pré-requisitos

Certifique-se de ter o Rust instalado em seu sistema:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 🔧 Instalação por Distribuição Linux

<details>
<summary><b>🟠 Ubuntu / Debian / Mint</b></summary>

```bash
# Instalação via cargo
cargo install --git https://github.com/siraprem/koi_terminal

# Ou compile do código fonte
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
# Instalação via cargo
cargo install --git https://github.com/siraprem/koi_terminal

# Ou compile do código fonte
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
# Instalação via cargo
cargo install --git https://github.com/siraprem/koi_terminal

# Ou compile do código fonte
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
# Instalação via cargo
cargo install --git https://github.com/siraprem/koi_terminal

# Ou compile do código fonte
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
# Instalação via cargo
cargo install --git https://github.com/siraprem/koi_terminal

# Ou compile do código fonte
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
# Instalação via cargo
cargo install --git https://github.com/siraprem/koi_terminal

# Ou compile do código fonte
sudo apk add build-base rust cargo
git clone https://github.com/siraprem/koi_terminal.git
cd koi_terminal
cargo build --release
sudo cp target/release/koi_terminal /usr/local/bin/
```

</details>

### 🎯 Instalação Rápida (Todas as Distribuições)

```bash
# Clone e instale em uma linha
git clone https://github.com/siraprem/koi_terminal.git && cd koi_terminal && cargo install --path .
```

---

## 🎮 Como Usar

### Comandos Básicos

```bash
# Iniciar o Koi Terminal
koi_terminal

# Executar com configurações personalizadas
koi_terminal --config ~/.config/koi/config.toml

# Modo de demonstração
koi_terminal --demo

# Exibir ajuda
koi_terminal --help
```

### ⚙️ Configuração

Crie um arquivo de configuração em `~/.config/koi/config.toml`:

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

| Modo Padrão | Modo Colorido | Modo Demo |
|-------------|---------------|-----------|
| ![Default](https://via.placeholder.com/250x150/0d1117/58a6ff?text=🐟+Default) | ![Colorful](https://via.placeholder.com/250x150/0d1117/ff6b6b?text=🎨+Colorful) | ![Demo](https://via.placeholder.com/250x150/0d1117/4ecdc4?text=✨+Demo) |

</div>

---

## 🏗️ Desenvolvimento

### 🛠️ Compilando do Código Fonte

```bash
# Clone o repositório
git clone https://github.com/siraprem/koi_terminal.git
cd koi_terminal

# Execute os testes
cargo test

# Compile e execute em modo debug
cargo run

# Compile em modo release
cargo build --release
```

### 📁 Estrutura do Projeto

```
koi_terminal/
├── src/
│   ├── main.rs          # Ponto de entrada principal
│   ├── koi/             # Módulo dos kois
│   ├── terminal/        # Interface do terminal
│   └── config/          # Sistema de configuração
├── tests/               # Testes automatizados
├── docs/                # Documentação
└── examples/            # Exemplos de uso
```

---

## 🤝 Contribuindo

Contribuições são muito bem-vindas! Veja como você pode ajudar:

1. **Fork** o projeto
2. Crie uma **branch** para sua feature (`git checkout -b feature/AmazingFeature`)
3. **Commit** suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. **Push** para a branch (`git push origin feature/AmazingFeature`)
5. Abra um **Pull Request**

### 📝 Guidelines

- Mantenha o código limpo e bem documentado
- Adicione testes para novas funcionalidades
- Siga as convenções do Rust (`cargo fmt` e `cargo clippy`)
- Atualize a documentação quando necessário

---

## 📄 Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

---

## 🙏 Agradecimentos

- 🦀 **Rust Community** - Pela linguagem incrível
- 🐟 **Koi Enthusiasts** - Pela inspiração
- 💻 **Terminal Artists** - Pelos exemplos e ideias
- 🌟 **Contributors** - Por tornar este projeto melhor

---

<div align="center">

**[⬆ Voltar ao topo](#-koi-terminal)**

Feito com 💙 por [Samira](https://github.com/siraprem)

*Que os kois tragam tranquilidade ao seu terminal!* 🐟✨

</div>
