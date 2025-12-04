# Aegis Backend API

FastAPI backend for receiving and storing secret scan events.

## Setup

```bash
# Create virtual environment
python -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Set environment variables
echo "DATABASE_URL=postgresql://postgres:postgres@localhost:5432/aegis" > .env

# Run server
uvicorn main:app --reload
```

## API Endpoints

- `GET /` - API info
- `GET /health` - Health check
- `POST /api/v1/events` - Create scan event
- `GET /api/v1/events` - List scan events (with optional `project_name` query param)
- `GET /api/v1/stats` - Get aggregated statistics

## API Documentation

Once running, visit:
- Swagger UI: http://localhost:8000/docs
- ReDoc: http://localhost:8000/redoc

## Database

Tables are automatically created on first run. For production, use Alembic migrations:

```bash
alembic init alembic
alembic revision --autogenerate -m "Initial migration"
alembic upgrade head
```

