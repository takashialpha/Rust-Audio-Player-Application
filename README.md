Here’s an improved version of your `README.md`, with the added dependencies, credits, and improved formatting:

---

# Audium - TUI Audio Player

Audium is a terminal user interface (TUI) audio player, built as a fork of the **Rust-Audio-Player-Application**. This application provides basic audio playback functionality with a terminal-based user experience, written in the Rust programming language.

## Features

- Play and pause audio
- Display total duration and current progress of the audio
- Navigate and control the audio player via the TUI interface

## Dependencies

To run Audium, make sure your system has the following dependencies:

### Linux Package Fix

If you encounter the error `failed to run custom build command for alsa-sys v0.3.1` on Linux systems, install the required libraries:

```bash
sudo apt install libasound2-dev pkg-config
```

Alternatively, use the equivalent package manager for your distro.

### All Dependencies

- **Rust**: The Rust programming language and its package manager, Cargo (required to build the application)
- **libasound2-dev**: Required for audio playback on Linux systems
- **pkg-config**: For managing library compilation and linking
- **Ranger**: For enhanced file navigation (optional, but recommended for better file exploration)
- **Terminal**: Your terminal emulator must support interactive UIs and color rendering.

### Installation of Dependencies

You can install additional dependencies using the following:

```bash
sudo apt install ranger
```

Ensure your terminal supports interactive UIs and color schemes for the best experience. 

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

Use the TUI interface to navigate and control audio playback directly in the terminal.

## Credits

- **Ranger**: The file manager used for navigation in the terminal. [Ranger GitHub](https://github.com/ranger/ranger)

## Contributions

Contributions are welcome! If you'd like to improve Audium or add new features, feel free to open issues or submit pull requests.

---

This version highlights the dependencies more clearly and gives credit to **Ranger** for its file navigation feature. It also includes a bit more detailed guidance for using the app and preparing your environment.
