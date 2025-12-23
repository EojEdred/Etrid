'use client'

import { NetworkNode } from '@/hooks/useNetworkStats'

interface NetworkMapProps {
  nodes: NetworkNode[]
}

export function NetworkMap({ nodes }: NetworkMapProps) {
  return (
    <div className="relative h-[400px] bg-gradient-to-br from-slate-900 to-slate-800 rounded-lg overflow-hidden border border-white/10 shadow-xl">
      {/* Simple world map background */}
      <div className="absolute inset-0 opacity-20 pointer-events-none">
        <svg viewBox="0 0 1000 500" className="w-full h-full">
          <rect fill="currentColor" width="1000" height="500" className="text-slate-700" />
          {/* Simplified continent shapes (approximate) */}
          <path fill="currentColor" className="text-slate-500" d="M100,100h50v50h-50zM200,150h50v50h-50zM300,100h50v50h-50zM400,200h50v50h-50zM500,150h50v50h-50zM600,100h50v50h-50zM700,200h50v50h-50zM800,150h50v50h-50z" />
          <text x="500" y="250" textAnchor="middle" fill="white" className="text-6xl font-bold opacity-10">WORLD MAP</text>
        </svg>
      </div>

      {/* Node markers */}
      <div className="absolute inset-0">
        {nodes.map((node, i) => {
          // Convert lat/lon to map coordinates
          // Longitude: -180 to 180 -> 0 to 100%
          // Latitude: 90 to -90 -> 0 to 100% (inverted)
          const x = Math.max(0, Math.min(100, ((node.lon + 180) / 360) * 100))
          const y = Math.max(0, Math.min(100, ((90 - node.lat) / 180) * 100))

          return (
            <div
              key={i}
              className={`absolute w-3 h-3 rounded-full border-2 border-white transform -translate-x-1/2 -translate-y-1/2 cursor-pointer transition-all hover:w-4 hover:h-4 hover:z-10 ${getMarkerClass(node)}`}
              style={{
                left: `${x}%`,
                top: `${y}%`,
                animation: 'pulse 3s infinite'
              }}
              title={`${node.name} (${node.location}) - ${node.status}`}
            />
          )
        })}
      </div>

      {/* Legend */}
      <div className="absolute bottom-4 right-4 bg-black/50 backdrop-blur-sm rounded-lg p-3 text-xs space-y-2 pointer-events-none">
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 rounded-full bg-green-500 border-2 border-white shadow-lg shadow-green-500/50" />
          <span className="text-white">Bootstrap Node</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 rounded-full bg-orange-500 border-2 border-white shadow-lg shadow-orange-500/50" />
          <span className="text-white">Validator</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 rounded-full bg-blue-500 border-2 border-white shadow-lg shadow-blue-500/50" />
          <span className="text-white">Full Node</span>
        </div>
      </div>

      <style jsx>{`
        @keyframes pulse {
          0%, 100% {
            opacity: 1;
            transform: translate(-50%, -50%) scale(1);
          }
          50% {
            opacity: 0.5;
            transform: translate(-50%, -50%) scale(1.2);
          }
        }
      `}</style>
    </div>
  )
}

function getMarkerClass(node: NetworkNode): string {
  const baseClass = 'shadow-lg'

  if (node.type === 'bootstrap') {
    return `${baseClass} bg-green-500 shadow-green-500/50`
  } else if (node.type === 'validator') {
    return `${baseClass} bg-orange-500 shadow-orange-500/50`
  } else {
    return `${baseClass} bg-blue-500 shadow-blue-500/50`
  }
}
