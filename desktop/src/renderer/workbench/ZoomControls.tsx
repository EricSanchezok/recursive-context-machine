import { ZoomIn, ZoomOut, Maximize2 } from 'lucide-react'

interface ZoomControlsProps {
  zoom: number
  onZoomIn: () => void
  onZoomOut: () => void
  onResetZoom: () => void
}

export function ZoomControls({ zoom, onZoomIn, onZoomOut, onResetZoom }: ZoomControlsProps) {
  return (
    <div className="absolute bottom-4 right-4 bg-white/95 backdrop-blur-sm rounded-xl shadow-lg flex items-center gap-2 px-2 py-1.5">
      <button className="p-1.5 hover:bg-gray-100 rounded-lg transition-colors" onClick={onZoomOut}>
        <ZoomOut size={18} className="text-gray-700" />
      </button>
      <button
        onClick={onResetZoom}
        className="px-3 py-1 hover:bg-gray-100 rounded-lg transition-colors text-sm font-medium text-gray-700 min-w-[50px]"
      >
        {Math.round(zoom * 100)}%
      </button>
      <button className="p-1.5 hover:bg-gray-100 rounded-lg transition-colors" onClick={onZoomIn}>
        <ZoomIn size={18} className="text-gray-700" />
      </button>
      <div className="w-px h-6 bg-gray-300 mx-1" />
      <button className="p-1.5 hover:bg-gray-100 rounded-lg transition-colors" onClick={onResetZoom}>
        <Maximize2 size={18} className="text-gray-700" />
      </button>
    </div>
  )
}
