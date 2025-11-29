# QRGen - QR Code Generator CLI (Rust Version)

[![CI](https://github.com/DougieWougie/qrgen-rust/workflows/CI/badge.svg)](https://github.com/DougieWougie/qrgen-rust/actions)

A fast, efficient command-line tool for generating QR codes from text, URLs, or any data, written in Rust.

## Features

- Generate QR codes from any text or URL
- Save as PNG images
- Display QR codes directly in the terminal using ASCII art
- Customize size, border, and error correction levels
- **Visual customization**: Custom colors and logo embedding
- **Content templates**: WiFi, vCard, SMS, email, and phone templates
- Simple and intuitive command-line interface
- High performance with Rust's speed and memory safety

## Installation

### From Pre-built Binaries (Recommended)

Download the latest release for your platform:
- [Releases Page](https://github.com/DougieWougie/qrgen-rust/releases)

Available for:
- Linux (x86_64, ARM64)
- macOS (Intel, Apple Silicon)
- Windows (x86_64)

```bash
# Linux/macOS example
wget https://github.com/DougieWougie/qrgen-rust/releases/download/v0.1.0/qrgen-0.1.0-linux-x86_64.tar.gz
tar -xzf qrgen-0.1.0-linux-x86_64.tar.gz
chmod +x qrgen
sudo mv qrgen /usr/local/bin/
```

### Build from Source

#### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs/))

#### Build

```bash
# Clone or navigate to the repository
cd qrgen-rust

# Build in release mode for optimal performance
cargo build --release

# The binary will be at ./target/release/qrgen
```

### Install to system

```bash
# Install to ~/.cargo/bin (make sure it's in your PATH)
cargo install --path .

# Or copy the binary to a system location
sudo cp target/release/qrgen /usr/local/bin/
```

## Usage

### Basic Examples

```bash
# Generate a QR code for a URL (saves to qr_code.png by default)
qrgen "https://example.com"

# Generate with custom output filename
qrgen "Hello World" -o my_qr.png

# Display QR code in terminal
qrgen "https://github.com" --terminal

# Display in terminal AND save to file
qrgen "Contact: john@example.com" -o contact.png --terminal

# Customize size and error correction
qrgen "https://example.com" -o big_qr.png --size 15 --error-correction H

# Generate a colored QR code
qrgen "Colorful QR" -o colorful.png --fill-color blue --back-color yellow

# Add a logo to the QR code
qrgen "https://mycompany.com" -o branded.png --logo logo.png

# Create a WiFi QR code
qrgen "MyNetwork,mypassword,WPA" --template wifi -o wifi.png
```

### Command-line Options

```
Usage: qrgen [OPTIONS] <DATA>

Arguments:
  <DATA>  The data to encode in the QR code (text, URL, etc.)

Options:
  -o, --output <OUTPUT>
          Output file path (PNG format). Default: qr_code.png
  -s, --size <SIZE>
          Size of each box in pixels [default: 10]
  -b, --border <BORDER>
          Border size in boxes [default: 4]
  -e, --error-correction <ERROR_CORRECTION>
          Error correction level: L(7%), M(15%), Q(25%), H(30%) [default: M]
  -t, --terminal
          Display QR code in terminal using ASCII characters
      --fill-color <FILL_COLOR>
          Fill color for QR code modules (default: black) [default: black]
      --back-color <BACK_COLOR>
          Background color for QR code (default: white) [default: white]
      --logo <LOGO>
          Path to logo image to embed in center of QR code
      --template <TEMPLATE>
          Use a template for specific content types
          [possible values: wifi, vcard, sms, email, phone]
  -h, --help
          Print help
  -V, --version
          Print version
```

## Error Correction Levels

QR codes support different levels of error correction, allowing them to be read even if partially damaged:

- **L (Low)**: Recovers up to 7% of data
- **M (Medium)**: Recovers up to 15% of data (default)
- **Q (Quartile)**: Recovers up to 25% of data
- **H (High)**: Recovers up to 30% of data

Higher error correction means the QR code can sustain more damage but will be larger.

## Visual Customization

### Custom Colors

You can customize the QR code colors using `--fill-color` and `--back-color` options:

```bash
# Blue QR code on white background
qrgen "https://example.com" -o blue_qr.png --fill-color blue

# Dark green on light yellow
qrgen "Custom colors" -o custom.png --fill-color darkgreen --back-color lightyellow

# Using hex colors
qrgen "Hex colors" -o hex.png --fill-color "#FF5733" --back-color "#C70039"
```

Supported named colors: black, white, red, green, blue, yellow, cyan, magenta, darkblue, darkgreen, lightgray, lightyellow

### Logo Embedding

Add a logo or image to the center of your QR code using the `--logo` option:

```bash
# Add company logo
qrgen "https://mycompany.com" -o branded.png --logo company_logo.png

# With high error correction (recommended for logos)
qrgen "https://example.com" -o logo_qr.png --logo logo.png --error-correction H
```

**Note**: When embedding logos, use high error correction (H) to ensure the QR code remains scannable even with the logo covering part of the code.

## Content Templates

Generate QR codes for common use cases with built-in templates:

### WiFi QR Code

```bash
# Format: SSID,password,encryption
qrgen "MyNetwork,mypassword123,WPA" --template wifi -o wifi.png

# Or use interactive mode (just provide any text)
qrgen "wifi" --template wifi -o wifi.png
# You'll be prompted for network details
```

### vCard (Contact Card)

```bash
# Format: name,phone,email,organization
qrgen "John Doe,+1234567890,john@example.com,Acme Corp" --template vcard -o contact.png

# Or use interactive mode
qrgen "contact" --template vcard -o contact.png
```

### SMS

```bash
# Format: phone_number,message
qrgen "1234567890,Hello there!" --template sms -o sms.png
```

### Email

```bash
# Format: email,subject,body
qrgen "contact@example.com,Hello,Message body" --template email -o email.png
```

### Phone Number

```bash
qrgen "1234567890" --template phone -o phone.png
```

## Examples

### Generate a QR code for a website

```bash
qrgen "https://github.com"
# Output: qr_code.png
```

### Generate a vCard contact QR code (using template)

```bash
qrgen "John Doe,+1234567890,john@example.com,Acme Corp" --template vcard -o contact.png
```

### Quick preview in terminal

```bash
qrgen "Quick message" --terminal
```

### Large, high-quality QR code with logo

```bash
qrgen "Important data" -o important.png --size 20 --error-correction H --logo mylogo.png
```

### Colored QR code for branding

```bash
qrgen "https://mybrand.com" -o branded.png --fill-color "#003366" --back-color "#FFCC00"
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run without installing
cargo run -- "https://example.com" -o test.png
```

### Testing

The project includes comprehensive test coverage with 68 tests (47 unit tests + 21 integration tests).

```bash
# Run all tests
cargo test

# Run tests in release mode (faster)
cargo test --release

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_parse_color_hex
```

Test coverage includes:
- ✅ QR code generation (all error levels)
- ✅ Color parsing (named and hex colors)
- ✅ All template types (WiFi, vCard, SMS, email, phone)
- ✅ CLI argument parsing
- ✅ Error handling and edge cases

See [TESTING.md](TESTING.md) for detailed test documentation.

## Dependencies

- `clap` - Command-line argument parsing
- `qrcode` - QR code generation
- `image` - Image processing and PNG encoding
- `anyhow` - Error handling

## Advantages of the Rust Version

- **Performance**: Significantly faster than the Python version
- **Memory Safety**: No runtime errors from memory issues
- **Single Binary**: No need for Python interpreter or dependencies
- **Small Binary Size**: Optimized release binary
- **Cross-platform**: Easily compile for Linux, macOS, and Windows

## License

MIT License - feel free to use this tool for any purpose.

## Author

Dougie Richardson

## Migration from Python Version

If you're coming from the Python version, the Rust version provides the same functionality with better performance. The command-line interface is identical, so you can use the same commands. The main difference is that you'll need to build the Rust binary instead of installing Python packages.

Note: The Rust version currently doesn't support Micro QR codes, but includes all other features from the Python version.
