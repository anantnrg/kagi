# Kagi

<p align="center">
  <img src="https://img.shields.io/github/languages/top/anantnrg/kagi?style=for-the-badge"/>
  <img src="https://img.shields.io/github/commit-activity/m/anantnrg/kagi?style=for-the-badge"/>
  <img src="https://img.shields.io/github/stars/anantnrg/kagi?style=for-the-badge"/>
  <img src="https://img.shields.io/github/watchers/anantnrg/kagi.svg?style=for-the-badge"/>
  <img src="https://img.shields.io/github/license/anantnrg/kagi?style=for-the-badge"/>
</p>

A **lightweight, fast, and zero-bullshit** music player built in Rust. No web garbage, no Electron bloat—just **pure, native SPEEEEEDDDD**.

<p align=center>
    <img src="https://github.com/anantnrg/kagi/blob/main/assets/showcase.png" />
</p>

## **Features** ⚡

- **🦀 Written in Rust**
- **🧠 Low Memory & CPU Usage** – **<75MB RAM, <2% CPU**.
- **🚀 Blazing Fast** – **200+ tracks load in under 3 sec thanks to [bincode](https://github.com/bincode-org/bincode)** (tested on a 5+yo HDD).
- **📂 Local Folder Playback**
- **🎨 Themeing with Hot-Reload**
- **🔄 Saves Playback State** – Pickup right where you left off.
- **🔍 Fuzzy Search** – Powered by **Nucleo** (sometimes derps, working on it).

### **Coming Soon™**

- **🎧 Crossfade between tracks**
- **📥 Download playlists from YouTube**
- **📝 Lyrics support**
- **📜 Custom playlists without needing folders**
- **⚡ More optimizations, less jank**

## **Getting Started** 🛠️

#### **1. Install Rust**

If you don’t already have Rust installed:

```sh
curl https://sh.rustup.rs -sSf | sh
```
Or use your package manager if you’re fancy:

```sh
sudo pacman -S rust # Arch
sudo apt install rustc cargo # Debian
```

#### **2. Clone & Build**
```sh
git clone https://github.com/anantnrg/kagi
cd kagi
cargo build --release
```

#### **3. Run It**

```sh
./target/release/kagi
```

# License

Kagi is licensed under the GNU GPL v3 License. See [LICENSE](https://github.com/anantnrg/kagi/tree/main/LICENSE) for details.

# Contributing

Refer to [CONTRIBUTING.md](https://github.com/anantnrg/kagi/tree/main/CONTRIBUTING.md)
