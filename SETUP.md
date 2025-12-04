# Aegis Setup Guide

This guide walks you through setting up Aegis for development or production use.

## Development Setup

### Prerequisites

Install the following:
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Python 3.9+](https://www.python.org/downloads/)
- [Node.js 18+](https://nodejs.org/)
- [PostgreSQL 12+](https://www.postgresql.org/download/) or use Docker

### Option 1: Local Setup

#### 1. Database Setup

Install PostgreSQL locally, or use Docker:

```bash
docker run --name aegis-postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=aegis -p 5432:5432 -d postgres:15-alpine
```

#### 2. Backend Setup

```bash
cd backend
python -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate
pip install -r requirements.txt

# Create .env file
echo "DATABASE_URL=postgresql://postgres:postgres@localhost:5432/aegis" > .env

# Run migrations (tables auto-create)
uvicorn main:app --reload
```

Backend will be at `http://localhost:8000`

#### 3. Frontend Setup

```bash
cd frontend
npm install

# Create .env.local
echo "NEXT_PUBLIC_API_URL=http://localhost:8000" > .env.local

npm run dev
```

Frontend will be at `http://localhost:3000`

#### 4. CLI Setup

```bash
cd cli
cargo build --release

# The binary will be at:
# - Linux/Mac: target/release/aegis-cli
# - Windows: target/release/aegis-cli.exe
```

### Option 2: Docker Compose (Recommended)

```bash
# Start database and backend
docker-compose up -d

# Setup frontend separately
cd frontend
npm install
echo "NEXT_PUBLIC_API_URL=http://localhost:8000" > .env.local
npm run dev
```

## Using the CLI

### Build the CLI

```bash
cd cli
cargo build --release
```

### Install Pre-commit Hook

In any git repository:

```bash
# From the cli directory
./target/release/aegis-cli install

# Or set API URL first
export AEGIS_API_URL=http://localhost:8000
./target/release/aegis-cli install
```

### Test the Scanner

Create a test file with a secret:

```bash
echo 'api_key = "AKIAIOSFODNN7EXAMPLE"' > test_secret.txt
git add test_secret.txt
git commit -m "test"  # Should be blocked!
```

## Production Deployment

### Backend (Railway/Render)

1. Push code to GitHub
2. Connect repository to Railway or Render
3. Set environment variable: `DATABASE_URL` (use Neon.tech or Railway Postgres)
4. Deploy

### Frontend (Vercel)

1. Push code to GitHub
2. Connect repository to Vercel
3. Set environment variable: `NEXT_PUBLIC_API_URL` (your backend URL)
4. Deploy

### Database (Neon.tech)

1. Create a Neon.tech account
2. Create a new project
3. Copy the connection string
4. Use as `DATABASE_URL` in backend

### CLI Distribution

Build and distribute the binary:

```bash
cd cli
cargo build --release

# For cross-platform builds:
cargo install cross
cross build --release --target x86_64-unknown-linux-musl
cross build --release --target x86_64-pc-windows-gnu
cross build --release --target x86_64-apple-darwin
```

## Troubleshooting

### CLI can't find git repository

Make sure you're in a git repository root when running `aegis-cli install`.

### Backend can't connect to database

- Check `DATABASE_URL` in `.env`
- Ensure PostgreSQL is running
- Verify connection credentials

### Frontend can't connect to backend

- Check `NEXT_PUBLIC_API_URL` in `.env.local`
- Ensure backend is running
- Check CORS settings in backend

### Pre-commit hook not running

- Verify hook is installed: `cat .git/hooks/pre-commit`
- Make sure hook is executable: `chmod +x .git/hooks/pre-commit`
- Check git config: `git config core.hooksPath`

## Testing

### CLI Tests

```bash
cd cli
cargo test
```

### Manual Testing

1. Create a test repository
2. Install the hook
3. Try committing files with/without secrets
4. Check dashboard for events

## Next Steps

- Configure secret patterns in `cli/src/scanner.rs`
- Customize dashboard in `frontend/`
- Add authentication to backend
- Set up monitoring and alerts

