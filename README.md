# quickxorhash-rust

A command-line tool written in Rust that calculates the QuickXorHash of a file and outputs the base64-encoded hash value.

## Features

- Calculate QuickXorHash for any file
- Output base64-encoded hash value
- Efficient file reading with chunked buffering
- Cross-platform support (Linux, macOS)

## Requirements

- Rust 1.70 or later
- Cargo

## Installation

### Build from source

```bash
# Clone the repository
git clone <repository-url>
cd quickxorhash-rust

# Build the project
cargo build --release

# The binary will be at target/release/quickxorhash-rust
```

## Usage

```bash
quickxorhash-rust <filePath>
```

### Example

```bash
# Calculate hash for a file
./target/release/quickxorhash-rust example.txt

# Output: base64-encoded hash value
```

## Development

### Run tests

```bash
cargo test
```

### Run with cargo

```bash
cargo run --release -- <filePath>
```

## Dependencies

- [quickxorhash](https://docs.rs/quickxorhash/) - QuickXorHash implementation
- [base64](https://docs.rs/base64/) - Base64 encoding

## License

MIT License - see [LICENSE](LICENSE) file for details.
