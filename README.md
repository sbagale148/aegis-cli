# Aegis

Aegis stops secrets at the commit line. It's a two-part system: a local Rust CLI that scans code before it's committed, and a cloud dashboard that gives security teams a view of what's almost been leaked.

## 🎯 Overview

Aegis prevents accidental secret leaks by scanning code before commits happen, at the pre-commit hook level. It combines fast regex-based pattern matching with Shannon entropy analysis to detect secrets with high accuracy and low false positives.

## 🏗️ Architecture

### Components

1. **Rust CLI** (`cli/`) - Pre-commit hook scanner
   - Fast regex-based secret detection
   - Shannon entropy analysis for false positive reduction
   - Non-blocking async API reporting

2. **FastAPI Backend** (`backend/`) - Event collection API
   - RESTful API for receiving scan events
   - PostgreSQL database for event storage
   - Statistics and aggregation endpoints

3. **Next.js Dashboard** (`frontend/`) - Security dashboard
   - Real-time scan event visualization
   - Project and secret type statistics
   - Modern, responsive UI

## 🚀 Quick Start

### Prerequisites

- Rust (latest stable)
- Python 3.9+
- Node.js 18+
- PostgreSQL 12+

### 1. Build the CLI

```bash
cd cli
cargo build --release
```

The binary will be at `cli/target/release/aegis-cli` (or `.exe` on Windows).

### 2. Install Pre-commit Hook

In your git repository:

```bash
./cli/target/release/aegis-cli install
```

Or set the API URL:

```bash
export AEGIS_API_URL=http://localhost:8000
./cli/target/release/aegis-cli install
```

### 3. Setup Backend

```bash
cd backend
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt
```

Create a `.env` file:

```bash
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/aegis
```

Run database migrations (tables are auto-created on first run):

```bash
uvicorn main:app --reload
```

The API will be available at `http://localhost:8000`

### 4. Setup Frontend

```bash
cd frontend
npm install
```

Create a `.env.local` file:

```
NEXT_PUBLIC_API_URL=http://localhost:8000
```

Run the development server:

```bash
npm run dev
```

The dashboard will be available at `http://localhost:3000`

## 📖 Usage

### CLI Commands

```bash
# Scan staged files
aegis-cli scan

# Scan with API reporting
AEGIS_API_URL=http://localhost:8000 aegis-cli scan

# Scan without API reporting
aegis-cli scan --no-report

# Install pre-commit hook
aegis-cli install

# Uninstall pre-commit hook
aegis-cli uninstall
```

### Pre-commit Hook

Once installed, the hook runs automatically on every `git commit`. If secrets are detected:
- The commit is blocked
- Details are shown in the terminal
- Events are reported to the API (if configured)

## 🔍 Supported Secret Patterns

- AWS Access Key IDs
- AWS Secret Access Keys
- GitHub Tokens
- Slack Tokens
- Generic API Keys
- JWT Tokens
- Private Keys (RSA, DSA, EC)
- Database Connection Strings
- Passwords in Configuration Files

## 🎛️ Configuration

### CLI Environment Variables

- `AEGIS_API_URL` - Backend API URL for reporting events

### Backend Environment Variables

- `DATABASE_URL` - PostgreSQL connection string

### Frontend Environment Variables

- `NEXT_PUBLIC_API_URL` - Backend API URL

## 📊 Performance Targets

- **Performance**: CLI adds <500ms to pre-commit process
- **Accuracy**: <5% false positive rate
- **Reliability**: 99.9% API event delivery

## 🛠️ Development

### Running Tests

```bash
# CLI tests
cd cli
cargo test

# Backend tests (when implemented)
cd backend
pytest
```

### Project Structure

```
aegis/
├── cli/                 # Rust CLI application
│   ├── src/
│   │   ├── main.rs     # CLI entry point
│   │   ├── scanner.rs  # Secret pattern matching
│   │   ├── entropy.rs  # Shannon entropy calculation
│   │   ├── git.rs      # Git repository integration
│   │   └── api.rs      # API client
│   └── Cargo.toml
├── backend/             # FastAPI backend
│   ├── main.py         # FastAPI application
│   ├── models.py       # Database models
│   ├── database.py     # Database configuration
│   └── requirements.txt
├── frontend/           # Next.js dashboard
│   ├── app/            # Next.js app directory
│   ├── components/     # React components
│   └── lib/            # Utilities
└── README.md
```

## 🚢 Deployment

### Backend (Railway)

1. Connect your repository to Railway
2. Set environment variables (DATABASE_URL)
3. Deploy

### Frontend (Vercel)

1. Connect your repository to Vercel
2. Set environment variables (NEXT_PUBLIC_API_URL)
3. Deploy

### Database (Neon.tech)

1. Create a Neon.tech PostgreSQL instance
2. Use the connection string as DATABASE_URL

## 📝 License

MIT

## 🤝 Contributing

Contributions welcome! Please open an issue or submit a pull request.

