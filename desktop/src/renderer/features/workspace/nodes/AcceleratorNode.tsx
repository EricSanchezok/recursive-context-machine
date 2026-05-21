import { useState, useCallback } from 'react'
import type { NodeData } from '../../../types/graph'

interface AcceleratorNodeProps {
  node: NodeData
  onMove: (x: number, y: number) => void
}

export function AcceleratorNode({ node, onMove }: AcceleratorNodeProps) {
  const [editing, setEditing] = useState(false)
  const [purpose, setPurpose] = useState(node.purpose)

  const startDrag = useCallback(
    (e: React.MouseEvent) => {
      const startX = e.clientX - node.x
      const startY = e.clientY - node.y
      const onMouseMove = (ev: MouseEvent) => onMove(ev.clientX - startX, ev.clientY - startY)
      const onMouseUp = () => {
        document.removeEventListener('mousemove', onMouseMove)
        document.removeEventListener('mouseup', onMouseUp)
      }
      document.addEventListener('mousemove', onMouseMove)
      document.addEventListener('mouseup', onMouseUp)
    },
    [node.x, node.y, onMove],
  )

  return (
    <div
      data-node
      className="absolute select-none"
      style={{ left: node.x, top: node.y, width: 260 }}
    >
      <div
        className="rounded-2xl shadow-lg border"
        style={{
          backgroundColor: 'var(--card)',
          borderColor: 'var(--border)',
        }}
      >
        <div
          onMouseDown={startDrag}
          className="px-4 py-3 border-b cursor-move flex items-center justify-between"
          style={{ borderColor: 'var(--border)' }}
        >
          <span className="font-semibold text-sm truncate" style={{ color: 'var(--foreground)' }}>
            {node.name || 'Accelerator'}
          </span>
        </div>

        <div className="px-4 py-3 space-y-2">
          {editing ? (
            <input
              autoFocus
              value={purpose}
              onChange={(e) => setPurpose(e.target.value)}
              onBlur={() => setEditing(false)}
              onKeyDown={(e) => { if (e.key === 'Enter') setEditing(false) }}
              className="w-full text-xs border rounded px-2 py-1 outline-none"
              style={{ borderColor: 'var(--border)', backgroundColor: 'var(--input-background)' }}
            />
          ) : (
            <p
              onDoubleClick={() => setEditing(true)}
              className="text-xs leading-relaxed cursor-text"
              style={{ color: 'var(--muted-foreground)' }}
            >
              {purpose || 'Double-click to set purpose'}
            </p>
          )}

          {node.model && (
            <div className="flex items-center gap-2 text-xs" style={{ color: 'var(--muted-foreground)' }}>
              <span className="w-2 h-2 rounded-full bg-purple-400" />
              {node.model}
            </div>
          )}
          {node.tools.length > 0 && (
            <div className="flex gap-1 flex-wrap">
              {node.tools.map((tool) => (
                <span
                  key={tool}
                  className="px-2 py-0.5 rounded text-xs"
                  style={{ backgroundColor: 'var(--muted)', color: 'var(--muted-foreground)' }}
                >
                  {tool}
                </span>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
