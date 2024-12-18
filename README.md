---

# Audium - TUI Audio Player

Audium is a fork of the **Rust-Audio-Player-Application**, built with a text user interface (TUI) using the Rust programming language. The application provides basic audio player functionality, but with a terminal-based user experience.

## Features

- Play and pause audio
- Display total duration and current progress of the audio
- Navigate and control audio via TUI interface

## Dependencies

Make sure you have the required dependencies to run Audium:

### Linux Package Fix

If you encounter the error `failed to run custom build command for alsa-sys v0.3.1` on Linux systems, install the necessary libraries using the following command (apt):

```bash
sudo apt install libasound2-dev pkg-config
```
Or the equivalent one for your distro

## How to Use

1. Clone the repository:

```bash
git clone https://github.com/your-username/audium.git
cd audium
```

2. Build the project:

```bash
cargo build --release
```

3. Run the application:

```bash
cargo run
```

Navigate and control the audio player directly in the terminal.

## Contributions

Contributions are welcome! Feel free to open issues or pull requests.

---
