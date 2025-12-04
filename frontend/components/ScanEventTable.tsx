'use client'

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

interface ScanEventTableProps {
  events: ScanEvent[]
}

export default function ScanEventTable({ events }: ScanEventTableProps) {
  if (events.length === 0) {
    return (
      <div className="px-6 py-12 text-center text-gray-500">
        No scan events found. Events will appear here when secrets are detected.
      </div>
    )
  }

  const formatDate = (dateString: string) => {
    try {
      return new Date(dateString).toLocaleString()
    } catch {
      return dateString
    }
  }

  const getConfidenceColor = (confidence: number) => {
    if (confidence >= 0.8) return 'text-red-600 bg-red-50'
    if (confidence >= 0.6) return 'text-orange-600 bg-orange-50'
    return 'text-yellow-600 bg-yellow-50'
  }

  return (
    <div className="overflow-x-auto">
      <table className="min-w-full divide-y divide-gray-200">
        <thead className="bg-gray-50">
          <tr>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Timestamp
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Project
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              File
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Type
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Confidence
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Preview
            </th>
          </tr>
        </thead>
        <tbody className="bg-white divide-y divide-gray-200">
          {events.map((event) => (
            <tr key={event.id} className="hover:bg-gray-50">
              <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                {formatDate(event.timestamp)}
              </td>
              <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                {event.project_name}
              </td>
              <td className="px-6 py-4 text-sm text-gray-500">
                <div className="max-w-xs truncate" title={event.file_path}>
                  {event.file_path}
                </div>
                <div className="text-xs text-gray-400">Line {event.line_number}</div>
              </td>
              <td className="px-6 py-4 whitespace-nowrap">
                <span className="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-100 text-blue-800">
                  {event.secret_type}
                </span>
              </td>
              <td className="px-6 py-4 whitespace-nowrap">
                <span className={`px-2 py-1 text-xs font-semibold rounded-full ${getConfidenceColor(event.confidence)}`}>
                  {(event.confidence * 100).toFixed(1)}%
                </span>
              </td>
              <td className="px-6 py-4 text-sm text-gray-500">
                <code className="bg-gray-100 px-2 py-1 rounded text-xs font-mono max-w-md truncate block">
                  {event.preview}
                </code>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}

