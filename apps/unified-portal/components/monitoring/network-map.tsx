'use client'

import { useEffect, useRef } from 'react'
import { NetworkNode } from '@/lib/hooks/useNetworkStats'

interface NetworkMapProps {
  nodes: NetworkNode[]
}

export function NetworkMap({ nodes }: NetworkMapProps) {
  const mapRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    if (!mapRef.current) return

    // Clear existing markers
    mapRef.current.innerHTML = ''

    // Create markers for each node
    nodes.forEach(node => {
      const marker = document.createElement('div')
      marker.className = `absolute w-3 h-3 rounded-full border-2 border-white transform -translate-x-1/2 -translate-y-1/2 cursor-pointer transition-all hover:w-4 hover:h-4 hover:z-10 ${getMarkerClass(node)}`

      // Convert lat/lon to map coordinates (simplified projection)
      const x = ((node.lon + 180) / 360) * 100
      const y = ((90 - node.lat) / 180) * 100

      marker.style.left = `${x}%`
      marker.style.top = `${y}%`
      marker.title = `${node.name} (${node.location}) - ${node.status}`

      // Add pulse animation
      marker.style.animation = 'pulse 3s infinite'

      mapRef.current?.appendChild(marker)
    })
  }, [nodes])

  return (
    <div className="relative h-[400px] bg-gradient-to-br from-slate-900 to-slate-800 rounded-lg overflow-hidden">
      {/* Simple world map background */}
      <div className="absolute inset-0 opacity-20">
        <svg viewBox="0 0 1000 500" className="w-full h-full">
          <rect fill="currentColor" width="1000" height="500" className="text-slate-700" />
          {/* Simplified continent shapes */}
          <path fill="currentColor" className="text-slate-600" d="M100,100h50v50h-50zM200,150h50v50h-50zM300,100h50v50h-50zM400,200h50v50h-50zM500,150h50v50h-50zM600,100h50v50h-50zM700,200h50v50h-50zM800,150h50v50h-50z" />
        </svg>
      </div>

      {/* Node markers */}
      <div ref={mapRef} className="absolute inset-0" />

      {/* Legend */}
      <div className="absolute bottom-4 right-4 bg-black/50 backdrop-blur-sm rounded-lg p-3 text-xs space-y-2">
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
