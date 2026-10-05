# 🖼️ imgbb-tui — Paste · Upload · Copy

> **Paste an image from your clipboard, upload it to imgbb, and get a copied URL — all inside a beautiful terminal UI.**

[![Rust Edition](https://img.shields.io/badge/Rust-2021%20Edition-DEA584?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20Wayland-FCC624?style=flat-square&logo=linux&logoColor=black)](https://wayland.freedesktop.org/)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)
[![imgbb](https://img.shields.io/badge/API-imgbb-cba6f7?style=flat-square)](https://api.imgbb.com/)

`imgbb-tui` is a fast, keyboard-driven TUI built in Rust. Copy any image to your clipboard, hit **`Ctrl+V`**, watch it upload with a live spinner, and get the direct URL **auto-copied** back to your clipboard.

---

## ✨ Key Features

- **📋 One-Key Clipboard Upload** — `Ctrl+V` reads `image/png` straight from the Wayland clipboard via `wl-paste` and uploads it instantly.
- **🖼️ Three Input Paths** — Clipboard image, typed/pasted file path, or drag-and-drop a path into the input box.
- **🎨 Beautiful Ratatui UI** — Rounded borders, spinner animation, and a color-coded result card (accent / success / error states).
- **🔗 Auto-Copy URL** — The direct image URL is copied to your clipboard with `wl-copy` the moment the upload succeeds.
- **📚 Upload History** — Keeps the last 50 uploads in `~/.local/share/imgbb-tui/history.json`.
- **🔐 First-Run Setup** — Prompts for your imgbb API key once and saves it to `~/.config/imgbb-tui/config.toml`.
- **🛡️ Zero-Config Wayland Native** — Uses `wl-paste` / `wl-copy`, no X11 shims, no daemons, no dependencies beyond the binary.

---

## ⚡ Quick Start

### 🪄 One-Liner Magic (Recommended)

```bash
curl -fsSL https://raw.githubuser [!NOTE]
> Pass options through `bash -s --`, e.g. install a specific tag or prefix:
> `curl -fsSL .../install.sh | bash -s -- --version v0.1.0 --prefix /usr/local/bin`

### 🛠️ Building From Source

```bash
git clone https://github.com/Praveensenpai/imgbb-tui.git
cd imgbb-tui
cargo build --release
./target/release/imgbb-tui
```

Or let the installer handle it:

```bash
./scripts/install.sh --from-source
```

> [!TIP]
> **Requirements:** Linux with Wayland, plus `wl-clipboard` (`wl-paste` / `wl-copy`).
> Install on Arch with `sudo pacman -S wl-clipboard`, on Debian/Ubuntu with `sudo apt install wl-clipboard`.

---

## 🔄 Architecture & Workflow

```text
┌─────────────────────────┐
│   ⌨️  Terminal (TUI)    │
│   · Ctrl+V / path input │
└────────────┬────────────┘
             │  1. Read image bytes
             ▼
┌─────────────────────────┐
│   📋 Wayland Clipboard  │
│   · wl-paste image/png  │
└────────────┬────────────┘
             │  2. Base64 encode
             ▼
┌─────────────────────────┐
│   ☁️  imgbb REST API    │
│   · multipart upload    │
└────────────┬────────────┘
             │  3. Parse URL + viewer + thumb
             ▼
┌──────────────────────────────────────────┐
│   ✅ Result Card                          │
│   · URL auto-copied via wl-copy          │
│   · Appended to history.json             │
└──────────────────────────────────────────┘
```

### Layer Map

| Layer | Responsibility |
| :--- | :--- |
| `src/domain/` | Pure models (`ImagePayload`, `UploadResult`) and typed `AppError` — zero I/O. |
| `src/infra/` | Clipboard (`wl-paste`/`wl-copy`), TOML config, JSON history persistence. |
| `src/api/` | `ImgbbClient` — multipart upload, response parsing, error decoding. |
| `src/cli/` | `App` state machine + async event loop (crossterm event stream). |
| `src/ui/` | Ratatui rendering: header, input, spinner, result card, footer. |

---

## 💻 Usage

| Key | Action |
| :--- | :--- |
| `Ctrl+V` | Read image from clipboard and upload |
| `Enter` | Upload the image at the typed path |
| `Backspace` | Delete last character of the path |
| `Delete` | Clear the entire path input |
| `Ctrl+C` / `Esc` | Quit |

### First Run

On first launch you'll be asked for an imgbb API key. Get a free one at [api.imgbb.com](https://api.imgbb.com/). It is stored at:

```toml
# ~/.config/imgbb-tui/config.toml
api_key = "your_key_here"
auto_copy = true
keep_history = true
```

---

## ⚙️ Configuration

| Key | Default | Description |
| :--- | :--- | :--- |
| `api_key` | `""` | Your imgbb API v1 key (prompted on first run). |
| `auto_copy` | `true` | Copy the direct URL to the clipboard after upload. |
| `keep_history` | `true` | Persist recent uploads to `~/.local/share/imgbb-tui/history.json`. |

---

## 📜 License

Licensed under the [MIT License](LICENSE).
© Praveen Senpai ([@Praveensenpai](https://github.com/Praveensenpai))