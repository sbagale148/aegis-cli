interface StatsCardProps {
  title: string
  value: string
  description: string
}

export default function StatsCard({ title, value, description }: StatsCardProps) {
  return (
    <div className="bg-white overflow-hidden shadow rounded-lg">
      <div className="p-5">
        <div className="flex items-center">
          <div className="flex-shrink-0">
            <div className="w-8 h-8 bg-blue-500 rounded-md flex items-center justify-center">
              <span className="text-white text-sm font-bold">📊</span>
            </div>
          </div>
          <div className="ml-5 w-0 flex-1">
            <dl>
              <dt className="text-sm font-medium text-gray-500 truncate">{title}</dt>
              <dd className="flex items-baseline">
                <div className="text-2xl font-semibold text-gray-900">{value}</div>
              </dd>
              <dd className="text-xs text-gray-500 mt-1">{description}</dd>
            </dl>
          </div>
        </div>
      </div>
    </div>
  )
}

