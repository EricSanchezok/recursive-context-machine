import { useCallback } from 'react'
import type { NodeData } from '../../../types/graph'

interface ConditionNodeProps {
  node: NodeData
  onMove: (x: number, y: number) => void
}

export function ConditionNode({ node, onMove }: ConditionNodeProps) {
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
      style={{ left: node.x, top: node.y, width: 220 }}
    >
      <div
        className="rounded-2xl shadow-lg border"
        style={{ backgroundColor: 'var(--card)', borderColor: 'var(--node-condition-border)' }}
      >
        <div
          onMouseDown={startDrag}
          className="px-4 py-3 cursor-move"
          style={{ borderColor: 'var(--border)' }}
        >
          <span className="font-semibold text-sm" style={{ color: 'var(--foreground)' }}>
            {node.name || 'Condition'}
          </span>
        </div>
        <div className="px-4 py-2 text-xs" style={{ color: 'var(--muted-foreground)' }}>
          {node.conditionPredicate || 'predicate'}
        </div>
        <div className="flex border-t" style={{ borderColor: 'var(--border)' }}>
          <div className="flex-1 text-center py-1.5 text-xs font-medium text-emerald-600">
            true
          </div>
          <div className="w-px" style={{ backgroundColor: 'var(--border)' }} />
          <div className="flex-1 text-center py-1.5 text-xs font-medium text-rose-500">
            false
          </div>
        </div>
      </div>
    </div>
  )
}
