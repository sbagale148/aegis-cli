# Aegis Project Structure

```
aegis/
├── cli/                          # Rust CLI application
│   ├── src/
│   │   ├── main.rs              # CLI entry point and commands
│   │   ├── scanner.rs           # Secret pattern matching engine
│   │   ├── entropy.rs           # Shannon entropy calculation
│   │   ├── git.rs               # Git repository integration
│   │   └── api.rs               # HTTP API client for reporting
│   ├── Cargo.toml               # Rust dependencies
│   └── README.md                # CLI-specific documentation
│
├── backend/                      # FastAPI backend
│   ├── main.py                  # FastAPI application and routes
│   ├── models.py                # SQLAlchemy database models
│   ├── database.py              # Database configuration
│   ├── requirements.txt         # Python dependencies
│   ├── Dockerfile               # Docker image for backend
│   └── README.md                # Backend-specific documentation
│
├── frontend/                     # Next.js dashboard
│   ├── app/                     # Next.js app directory
│   │   ├── layout.tsx          # Root layout
│   │   ├── page.tsx            # Dashboard homepage
│   │   └── globals.css         # Global styles
│   ├── components/              # React components
│   │   ├── ScanEventTable.tsx  # Events table component
│   │   └── StatsCard.tsx       # Statistics card component
│   ├── lib/
│   │   └── api.ts              # API client utilities
│   ├── package.json            # Node.js dependencies
│   ├── tsconfig.json           # TypeScript configuration
│   ├── tailwind.config.js      # Tailwind CSS configuration
│   └── README.md               # Frontend-specific documentation
│
├── docker-compose.yml           # Docker Compose for local development
├── README.md                    # Main project documentation
├── SETUP.md                     # Detailed setup guide
├── PROJECT_STRUCTURE.md         # This file
└── proposal.txt                 # Original project proposal
```

## Key Components

### CLI (`cli/`)
- **main.rs**: Handles CLI commands (scan, install, uninstall)
- **scanner.rs**: Contains regex patterns for secret detection
- **entropy.rs**: Calculates Shannon entropy for false positive reduction
- **git.rs**: Integrates with git to get staged files
- **api.rs**: Sends scan events to backend API (async, non-blocking)

### Backend (`backend/`)
- **main.py**: FastAPI routes for receiving events and serving statistics
- **models.py**: Database schema for scan events
- **database.py**: SQLAlchemy setup and connection management

### Frontend (`frontend/`)
- **page.tsx**: Main dashboard showing events and statistics
- **ScanEventTable.tsx**: Displays scan events in a table
- **StatsCard.tsx**: Shows aggregate statistics
- **api.ts**: Client for fetching data from backend

## Data Flow

1. Developer commits code → Git pre-commit hook triggers
2. CLI scans staged files → Detects secrets using regex + entropy
3. If secrets found → Commit blocked, events sent to backend
4. Backend stores events → PostgreSQL database
5. Dashboard displays events → Real-time updates via API

## Technologies

- **CLI**: Rust (clap, regex, reqwest, tokio, git2)
- **Backend**: Python (FastAPI, SQLAlchemy, PostgreSQL)
- **Frontend**: TypeScript/React (Next.js, Tailwind CSS)
- **Infrastructure**: Docker, Docker Compose

