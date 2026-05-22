import { useCallback } from 'react'
import type { NodeData } from '../../../types/graph'
import { PortHandle } from './PortHandle'

interface TextNodeProps {
  node: NodeData
  onMove: (x: number, y: number) => void
}

export function TextNode({ node, onMove }: TextNodeProps) {
  const startDrag = useCallback(
    (event: React.MouseEvent) => {
      const startX = event.clientX - node.x
      const startY = event.clientY - node.y
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
    <div data-node className="absolute select-none" style={{ left: node.x, top: node.y, width: 220 }}>
      <div className="rounded-2xl shadow-lg border" style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)', position: 'relative' }}>
        <PortHandle
          port={{ id: `${node.id}:value`, nodeId: node.id, direction: 'out', name: 'value', channel: 'purpose', index: 0 }}
          side="right"
          label="text"
        />
        <div onMouseDown={startDrag} className="px-4 py-3 border-b cursor-move" style={{ borderColor: 'var(--border)' }}>
          <span className="font-semibold text-sm" style={{ color: 'var(--foreground)' }}>{node.name || 'Text'}</span>
        </div>
        <div className="px-4 py-3 text-xs leading-relaxed" style={{ color: 'var(--muted-foreground)' }}>
          {node.text || node.purpose || 'Text value'}
        </div>
      </div>
    </div>
  )
}
