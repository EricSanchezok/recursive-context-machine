import type { NodeData } from '../../../types/graph'

interface ConditionNodeProps {
  node: NodeData
  onMove: (x: number, y: number) => void
}

export function ConditionNode({ node, onMove }: ConditionNodeProps) {
  return (
    <div
      data-node
      className="absolute select-none"
      style={{ left: node.x, top: node.y, width: 220 }}
    >
      <div
        className="rounded-2xl shadow-lg border"
        style={{ backgroundColor: 'var(--card)', borderColor: 'var(--orange-300, #fdba74)' }}
      >
        <div
          onMouseDown={(e) => { /* drag stub */ }}
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
