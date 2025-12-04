"""
Database models for Aegis
"""
from sqlalchemy import Column, Integer, String, Float, DateTime, Text
from sqlalchemy.ext.declarative import declarative_base
from datetime import datetime

Base = declarative_base()


class ScanEvent(Base):
    __tablename__ = "scan_events"

    id = Column(Integer, primary_key=True, index=True)
    timestamp = Column(DateTime, nullable=False, index=True)
    project_name = Column(String(255), nullable=False, index=True)
    file_path = Column(String(500), nullable=False)
    secret_type = Column(String(100), nullable=False, index=True)
    confidence = Column(Float, nullable=False)
    line_number = Column(Integer, nullable=False)
    preview = Column(Text, nullable=True)
    created_at = Column(DateTime, default=datetime.utcnow, nullable=False)

