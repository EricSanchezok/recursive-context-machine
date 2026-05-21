import type { NodeData } from '../../../types/graph'

interface FluxNodeProps {
  node: NodeData
  onMove: (x: number, y: number) => void
}

export function FluxNode({ node, onMove }: FluxNodeProps) {
  return (
    <div
      data-node
      className="absolute select-none"
      style={{ left: node.x, top: node.y, width: 200 }}
    >
      <div
        className="rounded-2xl shadow-lg border"
        style={{ backgroundColor: 'var(--card)', borderColor: 'var(--amber-200, #fde68a)' }}
      >
        <div
          onMouseDown={(e) => { /* drag stub */ }}
          className="px-4 py-3 cursor-move"
          style={{ borderColor: 'var(--border)' }}
        >
          <span className="font-semibold text-sm" style={{ color: 'var(--foreground)' }}>
            {node.name || 'Flux'}
          </span>
        </div>
        <div className="px-4 py-2 text-xs space-y-1" style={{ color: 'var(--muted-foreground)' }}>
          {node.fluxMode && <div>mode: {node.fluxMode}</div>}
          {node.fluxChannel && <div>channel: {node.fluxChannel}</div>}
          {node.fluxArity != null && <div>arity: {node.fluxArity}</div>}
        </div>
      </div>
    </div>
  )
}
