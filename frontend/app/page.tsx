'use client'

import { useEffect, useState } from 'react'
import ScanEventTable from '@/components/ScanEventTable'
import StatsCard from '@/components/StatsCard'
import { fetchEvents, fetchStats } from '@/lib/api'

interface ScanEvent {
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

interface Stats {
  total_events: number
  by_project: Array<{ project_name: string; count: number }>
  by_secret_type: Array<{ secret_type: string; count: number }>
}

export default function Home() {
  const [events, setEvents] = useState<ScanEvent[]>([])
  const [stats, setStats] = useState<Stats | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadData()
    // Refresh data every 30 seconds
    const interval = setInterval(loadData, 30000)
    return () => clearInterval(interval)
  }, [])

  const loadData = async () => {
    try {
      setLoading(true)
      const [eventsData, statsData] = await Promise.all([
        fetchEvents(),
        fetchStats()
      ])
      setEvents(eventsData)
      setStats(statsData)
      setError(null)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load data')
      console.error('Error loading data:', err)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <nav className="bg-white shadow-sm border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            <div className="flex items-center">
              <h1 className="text-2xl font-bold text-gray-900">🛡️ Aegis</h1>
              <span className="ml-3 text-sm text-gray-500">Secret Scanning Dashboard</span>
            </div>
          </div>
        </div>
      </nav>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {error && (
          <div className="mb-4 bg-red-50 border border-red-200 text-red-800 px-4 py-3 rounded">
            Error: {error}
          </div>
        )}

        {loading ? (
          <div className="text-center py-12">
            <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
            <p className="mt-2 text-gray-600">Loading...</p>
          </div>
        ) : (
          <>
            {stats && (
              <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                <StatsCard
                  title="Total Events"
                  value={stats.total_events.toString()}
                  description="Secrets blocked"
                />
                <StatsCard
                  title="Projects"
                  value={stats.by_project.length.toString()}
                  description="Monitored repositories"
                />
                <StatsCard
                  title="Secret Types"
                  value={stats.by_secret_type.length.toString()}
                  description="Unique patterns detected"
                />
              </div>
            )}

            <div className="bg-white shadow rounded-lg">
              <div className="px-6 py-4 border-b border-gray-200">
                <h2 className="text-xl font-semibold text-gray-900">Recent Scan Events</h2>
              </div>
              <ScanEventTable events={events} />
            </div>

            {stats && stats.by_project.length > 0 && (
              <div className="mt-8 bg-white shadow rounded-lg p-6">
                <h2 className="text-xl font-semibold text-gray-900 mb-4">Events by Project</h2>
                <div className="space-y-2">
                  {stats.by_project.map((project) => (
                    <div key={project.project_name} className="flex items-center justify-between">
                      <span className="text-gray-700 font-medium">{project.project_name}</span>
                      <span className="text-gray-900 font-bold">{project.count}</span>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {stats && stats.by_secret_type.length > 0 && (
              <div className="mt-8 bg-white shadow rounded-lg p-6">
                <h2 className="text-xl font-semibold text-gray-900 mb-4">Events by Secret Type</h2>
                <div className="space-y-2">
                  {stats.by_secret_type.map((type) => (
                    <div key={type.secret_type} className="flex items-center justify-between">
                      <span className="text-gray-700 font-medium">{type.secret_type}</span>
                      <span className="text-gray-900 font-bold">{type.count}</span>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </>
        )}
      </main>
    </div>
  )
}

