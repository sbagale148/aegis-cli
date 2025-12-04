# Aegis CLI

The Aegis command-line interface for scanning git commits for secrets.

## Building

```bash
cargo build --release
```

The binary will be at `target/release/aegis-cli` (or `.exe` on Windows).

## Usage

```bash
# Scan staged files
./target/release/aegis-cli scan

# Scan with API reporting
AEGIS_API_URL=http://localhost:8000 ./target/release/aegis-cli scan

# Install pre-commit hook
./target/release/aegis-cli install

# Uninstall pre-commit hook
./target/release/aegis-cli uninstall
```

## Development

```bash
# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run -- scan
```

