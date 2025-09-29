# Aegis - Pre-commit Secret Scanner

A robust command-line tool that prevents secrets from accidentally being committed to your repository. Built in Rust for performance and reliability.

## ✨ Features

- **High-Performance Scanning:** Lightning-fast detection that adds minimal overhead to your workflow
- **Multiple Secret Patterns:** Detects AWS keys, GitHub tokens, API keys, and other common credentials
- **Git Integration:** Automatically scans staged changes via pre-commit hooks
- **Security-First Design:** Only metadata is reported - actual secrets never leave your machine
- **Clear Error Messages:** Tells you exactly what and where the problem is

## 📁 Project Structure

    aegis-cli/
    ├── src/
    │   ├── main.rs         # CLI entry point and command handling
    │   ├── scanner.rs      # Core scanning logic with pattern matching
    │   └── config.rs       # Configuration management
    ├── Cargo.toml          # Rust dependencies and project config
    └── README.md

## 🛠️ Installation & Setup

1. Clone the repository:

```bash
git clone <your-repo-url>
cd aegis-cli
```

2. Build the project:

```bash
cargo build --release
The compiled binary will be available at `./target/release/aegis.`
```

## 🧪 Testing

Verify the scanner works by creating a test file with fake secrets:

```bash
echo "aws_access_key_id=AKIAIOSFODNN7EXAMPLE" > test.txt
echo "github_token=ghp_1234567890abcdefghijklmnopqrstuvwxyz" >> test.txt
```

Then run the scanner:

```bash
./target/release/aegis scan --file test.txt
The scanner should detect both secrets and exit with an error code.
```

## 🚀 Usage

**Basic Scanning**

```bash
# Scan current directory
./target/release/aegis scan

# Scan specific file
./target/release/aegis scan --file config.yaml

# Scan specific path
./target/release/aegis scan /path/to/your/code
```

**Git Integration**

Set up Aegis as a pre-commit hook in your repository:

```bash
# Navigate to your Git repository
cd /path/to/your/repo

# Install the hook (make sure aegis is in your PATH)
aegis install-hook
```

Now every git commit will automatically run Aegis and block commits containing potential secrets.

**Example Output**

When Aegis finds a potential secret:

`Error: Potential secret found in config.yaml:15 - AWS Access Key ID`

When the scan passes:

`Scan completed successfully - no secrets found`

## 🔧 Configuration

The current version uses sensible defaults. Future versions will support:

- Custom pattern matching rules
- File type inclusion/exclusion
- Backend event reporting configuration
- Custom secret patterns

## 🛡️ Security Notes

- **Local Processing:** All scanning happens locally on your machine
- **No Secret Transmission:** Actual credential values are never sent anywhere
- **Metadata Only:** Only file paths and secret types are reported to optional backend services
- **Fail-Safe:** If backend services are unavailable, local scanning continues working

## 📋 Current Limitations

This is an early version with some known limitations:

- Limited set of secret patterns (AWS, GitHub, generic API keys)
- Basic pattern matching without entropy analysis
- Configuration is currently hardcoded
- Git hook setup may need manual adjustment for some environments

## 🔮 What's Next

Upcoming features in development:

- Entropy analysis for detecting random-looking strings
- Expanded secret patterns for more services
- Configuration file support
- Improved error messages with fix suggestions
- Performance optimizations for larger codebases

## 🤝 Contributing
We're not quite ready for external contributions yet, but check back in a few weeks as the project stabilizes.

## 📄 License
This project is licensed under the MIT License.