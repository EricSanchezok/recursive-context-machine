import { useCallback } from 'react'
import type { AcceleratorGraphNode } from '../../../types/graph'
import { PortHandle } from './PortHandle'

interface AcceleratorNodeProps {
  node: AcceleratorGraphNode
  onMove: (x: number, y: number) => void
}

export function AcceleratorNode({ node, onMove }: AcceleratorNodeProps) {
  const startDrag = useCallback(
    (event: React.MouseEvent) => {
      const startX = event.clientX - node.position.x
      const startY = event.clientY - node.position.y
      const onMouseMove = (ev: MouseEvent) => onMove(ev.clientX - startX, ev.clientY - startY)
      const onMouseUp = () => {
        document.removeEventListener('mousemove', onMouseMove)
        document.removeEventListener('mouseup', onMouseUp)
      }
      document.addEventListener('mousemove', onMouseMove)
      document.addEventListener('mouseup', onMouseUp)
    },
    [node.position.x, node.position.y, onMove],
  )

  return (
    <div data-node className="absolute select-none" style={{ left: node.position.x, top: node.position.y, width: 260 }}>
      <div className="rounded-2xl shadow-lg border relative" style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}>
        {node.ports.filter((port) => port.direction === 'in').map((port) => (
          <PortHandle key={port.id} port={port} side="left" label={port.name} />
        ))}
        {node.ports.filter((port) => port.direction === 'out').map((port) => (
          <PortHandle key={port.id} port={port} side="right" label={port.name} />
        ))}

        <div onMouseDown={startDrag} className="px-4 py-3 border-b cursor-move flex items-center justify-between" style={{ borderColor: 'var(--border)' }}>
          <span className="font-semibold text-sm truncate" style={{ color: 'var(--foreground)' }}>
            {node.name || 'Accelerator'}
          </span>
        </div>

        <div className="px-4 py-3 space-y-2">
          <p className="text-xs leading-relaxed" style={{ color: 'var(--muted-foreground)' }}>
            {node.purpose || 'Connect a text node to purpose'}
          </p>
          {node.model && (
            <div className="flex items-center gap-2 text-xs" style={{ color: 'var(--muted-foreground)' }}>
              <span className="w-2 h-2 rounded-full bg-purple-400" />
              {node.model}
            </div>
          )}
          {node.tools.length > 0 && (
            <div className="flex gap-1 flex-wrap">
              {node.tools.map((tool) => (
                <span key={tool} className="px-2 py-0.5 rounded text-xs" style={{ backgroundColor: 'var(--muted)', color: 'var(--muted-foreground)' }}>
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
