import { useState, useCallback, useRef } from 'react'
import type { NodeData, Wire } from '../../types/graph'
import { ZoomControls } from '../../workbench/ZoomControls'
import { AcceleratorNode } from './nodes/AcceleratorNode'
import { FluxNode } from './nodes/FluxNode'
import { ConditionNode } from './nodes/ConditionNode'
import { TextNode } from './nodes/TextNode'
import { ConnectionLine } from './ConnectionLine'

interface WorkspaceProps {
  nodes: NodeData[]
  wires: Wire[]
  onNodesChange: (nodes: NodeData[]) => void
}

export function Workspace({ nodes, wires, onNodesChange }: WorkspaceProps) {
  const containerRef = useRef<HTMLDivElement>(null)
  const [zoom, setZoom] = useState(1)
  const [pan, setPan] = useState({ x: 0, y: 0 })
  const dragRef = useRef<{ startX: number; startY: number; panX: number; panY: number } | null>(null)

  const handleMouseDown = useCallback(
    (event: React.MouseEvent) => {
      if ((event.target as HTMLElement).closest('[data-node]')) return
      dragRef.current = { startX: event.clientX, startY: event.clientY, panX: pan.x, panY: pan.y }
    },
    [pan],
  )

  const handleMouseMove = useCallback((event: React.MouseEvent) => {
    if (!dragRef.current) return
    setPan({
      x: dragRef.current.panX + (event.clientX - dragRef.current.startX),
      y: dragRef.current.panY + (event.clientY - dragRef.current.startY),
    })
  }, [])

  const handleMouseUp = useCallback(() => { dragRef.current = null }, [])

  const handleWheel = useCallback((event: React.WheelEvent) => {
    const delta = event.deltaY > 0 ? -0.08 : 0.08
    setZoom((prev) => Math.min(2, Math.max(0.25, prev + delta)))
  }, [])

  const moveNode = useCallback(
    (id: string, x: number, y: number) => {
      onNodesChange(nodes.map((node) => (node.id === id ? { ...node, x, y } : node)))
    },
    [nodes, onNodesChange],
  )

  return (
    <div
      ref={containerRef}
      className="w-full h-full overflow-hidden"
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      onMouseLeave={handleMouseUp}
      onWheel={handleWheel}
    >
      <div
        className="relative w-full h-full"
        style={{ transform: `translate(${pan.x}px, ${pan.y}px) scale(${zoom})`, transformOrigin: '0 0' }}
      >
        <svg className="absolute inset-0 pointer-events-none" style={{ width: 5000, height: 3000 }}>
          {wires.map((wire) => {
            const from = portPoint(nodes, wire.from.nodeId, 'out')
            const to = portPoint(nodes, wire.to.nodeId, 'in')
            if (!from || !to) return null
            return <ConnectionLine key={wire.id} from={from} to={to} />
          })}
        </svg>

        {nodes.map((node) => {
          switch (node.kind) {
            case 'accelerator':
              return <AcceleratorNode key={node.id} node={node} onMove={(x, y) => moveNode(node.id, x, y)} />
            case 'flux':
              return <FluxNode key={node.id} node={node} onMove={(x, y) => moveNode(node.id, x, y)} />
            case 'condition':
              return <ConditionNode key={node.id} node={node} onMove={(x, y) => moveNode(node.id, x, y)} />
            case 'text':
              return <TextNode key={node.id} node={node} onMove={(x, y) => moveNode(node.id, x, y)} />
          }
        })}
      </div>

      <ZoomControls
        zoom={zoom}
        onZoomIn={() => setZoom((prev) => Math.min(2, prev + 0.1))}
        onZoomOut={() => setZoom((prev) => Math.max(0.25, prev - 0.1))}
        onResetZoom={() => { setZoom(1); setPan({ x: 0, y: 0 }) }}
      />
    </div>
  )
}

function portPoint(nodes: NodeData[], nodeId: string, side: 'in' | 'out') {
  if (nodeId === 'Input' || nodeId === 'Output' || nodeId === 'input' || nodeId === 'output') return null
  const node = nodes.find((candidate) => candidate.id === nodeId)
  if (!node) return null
  const width = node.kind === 'accelerator' ? 260 : node.kind === 'condition' ? 220 : 200
  const height = node.kind === 'accelerator' ? 118 : node.kind === 'condition' ? 110 : 96
  return {
    x: side === 'out' ? node.x + width : node.x,
    y: node.y + height / 2,
  }
}
