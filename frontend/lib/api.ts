import axios from 'axios'

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8000'

const api = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
})

export interface ScanEvent {
  id: number
  timestamp: string
  project_name: string
  file_path: string
  secret_type: string
  confidence: number
  line_number: number
  preview: string
  created_at: string
}

export interface Stats {
  total_events: number
  by_project: Array<{ project_name: string; count: number }>
  by_secret_type: Array<{ secret_type: string; count: number }>
}

export async function fetchEvents(projectName?: string): Promise<ScanEvent[]> {
  try {
    const params = projectName ? { project_name: projectName } : {}
    const response = await api.get<ScanEvent[]>('/api/v1/events', { params })
    return response.data
  } catch (error) {
    console.error('Error fetching events:', error)
    throw new Error('Failed to fetch scan events')
  }
}

export async function fetchStats(): Promise<Stats> {
  try {
    const response = await api.get<Stats>('/api/v1/stats')
    return response.data
  } catch (error) {
    console.error('Error fetching stats:', error)
    throw new Error('Failed to fetch statistics')
  }
}

