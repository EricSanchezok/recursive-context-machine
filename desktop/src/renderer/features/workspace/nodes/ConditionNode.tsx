import { useCallback } from 'react'
import type { ConditionGraphNode } from '../../../types/graph'
import { PortHandle } from './PortHandle'

interface ConditionNodeProps {
  node: ConditionGraphNode
  onMove: (x: number, y: number) => void
}

export function ConditionNode({ node, onMove }: ConditionNodeProps) {
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
    <div data-node className="absolute select-none" style={{ left: node.position.x, top: node.position.y, width: 220 }}>
      <div className="rounded-2xl shadow-lg border relative" style={{ backgroundColor: 'var(--card)', borderColor: 'var(--node-condition-border)' }}>
        {node.ports.filter((port) => port.direction === 'in').map((port) => (
          <PortHandle key={port.id} port={port} side="left" label={port.name} />
        ))}
        {node.ports.filter((port) => port.direction === 'out').map((port) => (
          <PortHandle key={port.id} port={port} side="right" label={port.name} />
        ))}
        <div onMouseDown={startDrag} className="px-4 py-3 cursor-move">
          <span className="font-semibold text-sm" style={{ color: 'var(--foreground)' }}>{node.name || 'Condition'}</span>
        </div>
        <div className="px-4 py-2 text-xs" style={{ color: 'var(--muted-foreground)' }}>{node.predicate}</div>
      </div>
    </div>
  )
}
