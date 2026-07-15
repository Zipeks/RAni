# 🌸 RAni — Unofficial AniList TUI Client

<div align="center">

[![Rust](https://img.shields.io/badge/made_with-Rust-dca282.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
<!-- [![AUR version](https://img.shields.io/aur/version/anilist-tui-git?color=1793d1&label=AUR&logo=arch-linux&style=for-the-badge)](https://aur.archlinux.org/packages/anilist-tui-git) -->
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Ratatui](https://img.shields.io/badge/powered_by-ratatui-ff69b4.svg?style=for-the-badge)](https://github.com/ratatui-org/ratatui)

**A feature-rich, keyboard-centric, and asynchronous Terminal User Interface for [AniList.co](https://anilist.co), created with Rust 🦀.**

[Features](#features) • [Installation](#installation) • [Keybindings](#keybindings) • [Tech Stack](#tech-stack)

</div>

---

## 📸 Demo

https://github.com/user-attachments/assets/dce97ff0-b236-48ca-b40a-a4f6362089c7

---

## ✨ Features  <a id="features"></a>


- ⚡ **Blazing Fast & Asynchronous:** Built on top of **Tokio** and **Ratatui**. Non-blocking I/O ensures your UI never freezes, running at a buttery-smooth 60 FPS.
- ⌨️ **Vim-Centric Navigation:** Designed for power users. Effortlessly navigate lists, switch tabs, and edit entries without ever touching your mouse.
- 🖼️ **Terminal Cover Art Rendering:** View high-resolution anime and manga promotional posters directly inside your terminal (powered by `ratatui-image`).
- 🧠 **Intelligent State & Caching:** Utilizes high-performance in-memory caching (**Moka**). Browsing filters and search queries are persistently remembered across tab switches.
- 🛡️ **Rate-Limit Safe:** Input debouncing prevents API spam during rapid scrolling, keeping your AniList account safe from HTTP 429 bans.
- 📋 **Full List Management:**
  - Update watch/read progress and episode counts on the fly.
  - Modify custom scores and repeat/rewatch counters.
  - Add and delete entries directly from your TUI.

## 🚀 Installation <a id="installation"></a>

RAni offers multiple installation methods depending on your Linux distribution and preferences.

### Option 1: Flatpak (Universal & Recommended)
You can install RAni as a self-contained Flatpak bundle on any Linux distribution. This method bundles all required dependencies automatically.
1. Download `rani.flatpak` from the [Latest Release](https://github.com/Zipeks/RAni/releases).
2. Install it via terminal:
```bash
flatpak install rani.flatpak
```
3. Add alias to your shell file (optional)
```bash
alias rani="flatpak run com.github.zipeks.rani"
```


### Option 2: Pre-built Binary
You can download the pre-compiled native binary (`rani-linux-x86_64.tar.gz`) from the [Releases](https://github.com/Zipeks/RAni/releases) page. 
Simply extract the archive and run the `rani` executable, or move it to a directory in your `$PATH` (e.g., `~/.local/bin/`).

---

### Option 3: From Source (Cargo)
RAni is built with Rust and requires a few system libraries to compile properly, mainly **Chafa** (for rendering high-quality terminal graphics) and **D-Bus** (for secure token storage).

**1. Install System Dependencies:**
- **Arch Linux / CachyOS:**
  ```bash
  sudo pacman -S rust base-devel pkgconf chafa dbus
  ```
- **Ubuntu / Debian:**
  ```bash
  sudo apt install cargo build-essential pkg-config libchafa-dev libdbus-1-dev
  ```
- **Fedora:**
  ```bash
  sudo dnf install cargo gcc pkgconf-pkg-config chafa-devel dbus-devel
  ```

**2. Build and Install:**
You can build and install the binary directly from GitHub using Cargo. The executable will be placed in your `~/.cargo/bin` directory.

```bash
git clone https://github.com/Zipeks/RAni.git
cd RAni
cargo install --path .
```
*(Make sure `~/.cargo/bin` is added to your system's `$PATH`).*

## 🎮 Keybindings <a id="keybindings"></a>

RAni utilizes modal editing (similar to Neovim), splitting interactions into **Normal Mode** (navigation & quick toggles) and **Edit Mode** (text & numeric typing).

### 🌐 Global Navigation (Normal Mode)

| Key | Action |
| :--- | :--- |
| `j` / `↓` | Move selection down |
| `k` / `↑` | Move selection up |
| `h` / `←` | Go back / Focus Sidebar / Center |
| `l` / `→` | Select item / Open Details View |
| `Tab` / `]` | Next Category (Watching -> Watched -> Planning -> All) |
| `Shift+Tab` / `[` | Previous Category |
| `f` | Change current tab filters |
| `r` | **Reset Filters** to category defaults |
| `n` / `p` | Next / Previous page |
| `t` | Change title display language (Romaji, English, Native) |
| `q` | Quit application |

### 📝 Media Details & Actions

| Key | Action |
| :--- | :--- |
| `e` | **Open Edit Menu** (Modify status, progress, score, dates) |
| `f` | **Toggle Favorite** ❤️  |
| `d` | **Delete Entry** from your AniList  |
| `r` | Display related media  |
| `o` | Open selected media anilist page |

### 🛠️ Edit / Filter Popups (Vim Workflow)

When an edit or filter popup is active:
- Use `j` / `k` (or `Tab` / `Shift+Tab`) to navigate between form fields.
- In **Normal Mode**:
  - Use `h` / `l` (Left/Right) to quickly increment/decrement numbers (Progress, Score) or cycle through enum options (Status, Sort, Format).
  - Press `i` or `Enter` on text/numeric fields to enter **Edit/Insert Mode**.
  - Press `s` to **Save** changes and sync with AniList.
  - Press `r` to **Reset** parameters.
  - Press `Esc` or `q` to **Cancel** and close the popup.
- In **Edit/Insert Mode**:
  - Type directly to append numbers or text (e.g., Dates, Notes).
  - Press `Backspace` to delete characters or divide numbers by 10.
  - Press `Enter` or `Esc` to exit back to Normal Mode.

---

## 🏗️ Tech Stack <a id="tech-stack"></a>

- **[Ratatui](https://github.com/ratatui-org/ratatui)** — For the terminal UI rendering.
- **[Tokio](https://tokio.rs/)** — Asynchronous runtime for non-blocking network requests.
- **[GraphQL Client](https://github.com/graphql-rust/graphql-client)** — For type-safe AniList API v2 communication.
- **[Moka](https://github.com/moka-rs/moka)** — High-performance concurrent caching.
- **[Serde](https://serde.rs/)** — Serialization and deserialization of API payloads.

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/Zipeks/RAni/issues).

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.
