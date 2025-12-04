"""
Aegis Backend API
FastAPI server for receiving and storing secret scan events
"""
from fastapi import FastAPI, HTTPException, Depends
from fastapi.middleware.cors import CORSMiddleware
from sqlalchemy.orm import Session
from pydantic import BaseModel
from typing import List, Optional
from datetime import datetime
import os

from database import SessionLocal, engine, Base
from models import ScanEvent as ScanEventModel

# Create database tables
Base.metadata.create_all(bind=engine)

app = FastAPI(
    title="Aegis API",
    description="API for receiving secret scan events from Aegis CLI",
    version="1.0.0"
)

# CORS middleware for frontend access
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # In production, replace with specific origins
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Dependency to get DB session
def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()


# Pydantic models for request/response
class ScanEventCreate(BaseModel):
    timestamp: str
    project_name: str
    file_path: str
    secret_type: str
    confidence: float
    line_number: int
    preview: str


class ScanEventResponse(BaseModel):
    id: int
    timestamp: str
    project_name: str
    file_path: str
    secret_type: str
    confidence: float
    line_number: int
    preview: str
    created_at: str

    class Config:
        from_attributes = True


@app.get("/")
async def root():
    return {"message": "Aegis API", "version": "1.0.0"}


@app.get("/health")
async def health_check():
    return {"status": "healthy"}


@app.post("/api/v1/events", response_model=ScanEventResponse, status_code=201)
async def create_scan_event(
    event: ScanEventCreate,
    db: Session = Depends(get_db)
):
    """Receive and store a scan event from the CLI"""
    try:
        # Parse timestamp
        event_timestamp = datetime.fromisoformat(event.timestamp.replace('Z', '+00:00'))
    except Exception:
        event_timestamp = datetime.utcnow()

    db_event = ScanEventModel(
        timestamp=event_timestamp,
        project_name=event.project_name,
        file_path=event.file_path,
        secret_type=event.secret_type,
        confidence=event.confidence,
        line_number=event.line_number,
        preview=event.preview,
    )

    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return ScanEventResponse(
        id=db_event.id,
        timestamp=db_event.timestamp.isoformat(),
        project_name=db_event.project_name,
        file_path=db_event.file_path,
        secret_type=db_event.secret_type,
        confidence=db_event.confidence,
        line_number=db_event.line_number,
        preview=db_event.preview,
        created_at=db_event.created_at.isoformat(),
    )


@app.get("/api/v1/events", response_model=List[ScanEventResponse])
async def get_scan_events(
    project_name: Optional[str] = None,
    limit: int = 100,
    offset: int = 0,
    db: Session = Depends(get_db)
):
    """Get scan events with optional filtering"""
    query = db.query(ScanEventModel)

    if project_name:
        query = query.filter(ScanEventModel.project_name == project_name)

    events = query.order_by(ScanEventModel.timestamp.desc()).offset(offset).limit(limit).all()

    return [
        ScanEventResponse(
            id=event.id,
            timestamp=event.timestamp.isoformat(),
            project_name=event.project_name,
            file_path=event.file_path,
            secret_type=event.secret_type,
            confidence=event.confidence,
            line_number=event.line_number,
            preview=event.preview,
            created_at=event.created_at.isoformat(),
        )
        for event in events
    ]


@app.get("/api/v1/stats")
async def get_stats(db: Session = Depends(get_db)):
    """Get aggregated statistics about scan events"""
    total_events = db.query(ScanEventModel).count()
    
    # Count by project
    from sqlalchemy import func
    project_counts = db.query(
        ScanEventModel.project_name,
        func.count(ScanEventModel.id).label('count')
    ).group_by(ScanEventModel.project_name).all()

    # Count by secret type
    type_counts = db.query(
        ScanEventModel.secret_type,
        func.count(ScanEventModel.id).label('count')
    ).group_by(ScanEventModel.secret_type).all()

    return {
        "total_events": total_events,
        "by_project": [
            {"project_name": project, "count": count}
            for project, count in project_counts
        ],
        "by_secret_type": [
            {"secret_type": secret_type, "count": count}
            for secret_type, count in type_counts
        ],
    }


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)

