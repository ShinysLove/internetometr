
# ⚡ Internetometr

A lightweight, cross-platform CLI network speed and ping tester built in Rust. It automatically detects the optimal server region (Russia, Europe, USA, China) based on latency, measures HTTP ping, download, and upload speeds, and delivers results in a clean, color-coded terminal interface.

> 🔒 **Privacy & Transparency**  
> This tool performs standard HTTP requests to public endpoints to measure latency and throughput. It does not collect, store, or transmit any personal data, browsing history, or network payloads. All measurements are performed locally in memory and strictly adhere to standard network diagnostic practices.

---

## ⚡ Features

| Category | Description |
|---|---|
| 🌍 **Smart Routing** | Auto-detects the best region (RU, EU, USA, CN) via lowest latency probing. |
| 📏 **Core Metrics** | Accurately measures HTTP ping, download throughput, and upload speed. |
| 🪶 **Minimal Footprint** | Optimized with LTO, strip, and `panic = "abort"`. Final binary size is under 2MB. |
| 💻 **Cross-Platform** | Pre-compiled static binaries for Linux (`musl`) and Windows (`.exe`). |
| 🎨 **Clean UI** | Color-coded terminal output for instant readability. |
| ⚡ **Rust Powered** | Written in Rust for memory safety, zero-cost abstractions, and high performance. |

---

## 🛠️ Build Instructions

### 📦 Dependencies (`Cargo.toml`)
Add to your `Cargo.toml`:
```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
reqwest = { version = "0.12", default-features = false, features = ["rustls-tls"] }
tokio = { version = "1", default-features = false, features = ["rt", "macros"] }
colored = { version = "2.1", default-features = false }
rand = { version = "0.8", default-features = false, features = ["std", "std_rng"] }
thiserror = "1.0"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
panic = "abort"
```

### 🚀 Standard Compilation
Requires [Rust](https://rustup.rs/) installed.

```bash
git clone https://github.com/ShinysLove/internetometr.git
cd internetometr
cargo build --release
```
The compiled binary will be located at `target/release/Internetometr`.

### 🌐 Cross-Compilation (Static Binaries)
To build minimal static binaries from Linux:

```bash
# Install system linkers (Ubuntu/Debian)
sudo apt update && sudo apt install musl-tools mingw-w64

# Add Rust targets
rustup target add x86_64-unknown-linux-musl x86_64-pc-windows-gnu

# Build Linux (static, runs on any distro)
cargo build --release --target x86_64-unknown-linux-musl

# Build Windows (.exe)
cargo build --release --target x86_64-pc-windows-gnu
```

---

## 🚀 Usage

Run the tool with auto-detected region (default):
```bash
./Internetometr
```

Specify a region manually:
```bash
./Internetometr --region europe
# or
./Internetometr -r usa
```

Adjust the timeout (in seconds, default is 15):
```bash
./Internetometr --timeout 30
```

### Help Menu
```text
CLI network speed and ping tester

Usage: Internetometr [OPTIONS]

Options:
  -r, --region <REGION>  [default: auto] [possible values: auto, russia, europe, usa, china]
      --timeout <TIMEOUT>  [default: 15]
  -h, --help             Print help
```

---

## 📦 Installation

### Option 1: Download Pre-built Binaries
Go to the [Releases](https://github.com/ShinysLove/internetometr/releases) page and download the latest version for your OS:
- **Linux**: `Internetometr` (static `musl` binary, works on any distro)
- **Windows**: `Internetometr.exe`

### Option 2: Build from Source
See the "Build Instructions" section above.

---

