# WayRuler

A Wayland-native smart screen measurement tool for KDE Plasma. Measure distances between UI elements, windows, and screen boundaries in real-time with intelligent edge detection.

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/rust-edition%202021-orange)

## Features

- **Crosshair Measurement Mode**: Interactive crosshair that snaps to detected edges
- **Intelligent Edge Detection**: Automatically detects visual boundaries based on luminance contrast
- **Real-time Pixel Measurements**: Displays width and height measurements as you move your cursor
- **Scroll-to-Skip**: Use mouse wheel to cycle through multiple detected edges along the same axis
- **Wayland Native**: Built specifically for Wayland compositors using layer-shell protocol
- **Transparent Overlay**: Non-intrusive overlay that doesn't interfere with normal workflow

## How It Works

1. **Screen Capture**: Uses `spectacle` (KDE's screenshot tool) to capture the current screen
2. **Edge Detection**: Analyzes the screenshot to find visual edges based on contrast thresholds
3. **Interactive Overlay**: Displays a GTK4 overlay with Cairo-rendered measurement lines and HUD
4. **Live Tracking**: Cursor movement updates measurements in real-time

## Requirements

- **Operating System**: Linux with Wayland session
- **Desktop Environment**: KDE Plasma 6 (tested)
- **Dependencies**:
  - `spectacle` - For screen capture
  - `gtk4` - GUI framework
  - `cairo` - 2D graphics rendering

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/Mahmoud-walid/linux-screen-ruler.git
cd wayruler

# Build the project
cargo build --release

# Run the application
./target/release/wayruler
```

### Arch Linux (using PKGBUILD)

```bash
# Build and install using the provided PKGBUILD
makepkg -si
```

## Usage

1. Launch WayRuler:
   ```bash
   wayruler
   ```

2. The overlay will appear on your screen with a crosshair following your cursor

3. Move your cursor to measure distances between UI elements

4. Use the mouse scroll wheel to cycle through multiple detected edges

5. Press `Esc` or close the window to exit

## Controls

- **Mouse Movement**: Position the crosshair
- **Scroll Wheel**: Cycle through detected edges (forward/backward)
- **Escape**: Close the application

## Technical Details

### Architecture

- **capture.rs**: Handles screen capture using `spectacle` command-line tool
- **edge_detector.rs**: Implements edge detection algorithm based on luminance contrast
- **overlay.rs**: Creates the GTK4 layer-shell overlay with Cairo rendering
- **main.rs**: Application entry point and async runtime setup

### Edge Detection Algorithm

The edge detection works by:
1. Starting from the cursor position
2. Scanning in both directions along horizontal and vertical axes
3. Calculating luminance values for each pixel
4. Detecting edges where contrast exceeds the threshold (default: 25)
5. Returning the coordinates of detected edges

### Dependencies

- `gtk4` (0.8) - Modern GUI toolkit
- `gtk4-layer-shell` (0.3) - Wayland layer-shell integration
- `cairo-rs` (0.19) - 2D graphics rendering
- `image` (0.24) - Image processing and screenshot handling
- `tokio` (1.36) - Async runtime
- `zbus` (4.0) / `ashpd` (0.8) - D-Bus communication (for future Portal support)

## Building from Source

### Prerequisites

```bash
# Ubuntu/Debian
sudo apt install libgtk-4-dev libcairo2-dev spectacle

# Arch Linux
sudo pacman -S gtk4 cairo spectacle

# Fedora
sudo dnf install gtk4-devel cairo-devel spectacle
```

### Build Commands

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run directly with cargo
cargo run --release
```

## Project Structure

```
wayruler/
├── src/
│   ├── main.rs           # Application entry point
│   ├── capture.rs        # Screen capture functionality
│   ├── edge_detector.rs  # Edge detection algorithm
│   └── overlay.rs        # GTK4 overlay UI
├── scripts/
│   └── generate-full-project-content.rs  # Utility script
├── Cargo.toml            # Rust project manifest
├── PKGBUILD              # Arch Linux build script
└── README.md             # This file
```

## Development

### Running in Development Mode

```bash
cargo run --release
```

### Code Formatting

```bash
cargo fmt
```

### Clippy Linting

```bash
cargo clippy -- -D warnings
```

## Known Limitations

- Currently designed for KDE Plasma 6 on Wayland
- Edge detection may not work perfectly on all screen contents
- Requires `spectacle` to be installed for screen capture
- Single monitor support (primary display only)

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is licensed under the MIT License.

## Acknowledgments

- KDE community for Spectacle and Plasma
- GTK4 developers for the excellent GUI framework
- The Wayland community for the layer-shell protocol

## Support

For issues, questions, or suggestions, please open an issue on the [GitHub repository](https://github.com/Mahmoud-walid/linux-screen-ruler).