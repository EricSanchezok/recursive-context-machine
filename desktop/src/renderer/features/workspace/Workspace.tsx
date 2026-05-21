import { useRef, useState, useCallback } from 'react'
import type { NodeData, Wire } from '../../types/graph'
import { ZoomControls } from '../../workbench/ZoomControls'
import { AcceleratorNode } from './nodes/AcceleratorNode'
import { FluxNode } from './nodes/FluxNode'
import { ConditionNode } from './nodes/ConditionNode'

interface WorkspaceProps {
  nodes: NodeData[]
  wires: Wire[]
  onNodesChange: (nodes: NodeData[]) => void
  onWiresChange: (wires: Wire[]) => void
}

export function Workspace({ nodes, wires, onNodesChange, onWiresChange }: WorkspaceProps) {
  const containerRef = useRef<HTMLDivElement>(null)
  const [zoom, setZoom] = useState(1)
  const [pan, setPan] = useState({ x: 0, y: 0 })
  const draggingRef = useRef<{ startX: number; startY: number; panX: number; panY: number } | null>(null)

  const handleMouseDown = useCallback(
    (e: React.MouseEvent) => {
      const target = e.target as HTMLElement
      if (target.closest('[data-node]')) return
      draggingRef.current = { startX: e.clientX, startY: e.clientY, panX: pan.x, panY: pan.y }
    },
    [pan],
  )

  const handleMouseMove = useCallback(
    (e: React.MouseEvent) => {
      if (!draggingRef.current) return
      const dx = e.clientX - draggingRef.current.startX
      const dy = e.clientY - draggingRef.current.startY
      setPan({ x: draggingRef.current.panX + dx, y: draggingRef.current.panY + dy })
    },
    [],
  )

  const handleMouseUp = useCallback(() => {
    draggingRef.current = null
  }, [])

  const handleWheel = useCallback((e: React.WheelEvent) => {
    const delta = e.deltaY > 0 ? -0.1 : 0.1
    setZoom((z) => Math.min(2, Math.max(0.2, z + delta)))
  }, [])

  const moveNode = useCallback(
    (id: string, x: number, y: number) => {
      onNodesChange(nodes.map((n) => (n.id === id ? { ...n, x, y } : n)))
    },
    [nodes, onNodesChange],
  )

  return (
    <div
      ref={containerRef}
      className="w-full h-full overflow-hidden"
      style={{ backgroundColor: 'var(--workspace-bg)' }}
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
        {nodes.map((node) => {
          switch (node.kind) {
            case 'accelerator':
              return (
                <AcceleratorNode
                  key={node.id}
                  node={node}
                  onMove={(x, y) => moveNode(node.id, x, y)}
                />
              )
            case 'flux':
              return (
                <FluxNode
                  key={node.id}
                  node={node}
                  onMove={(x, y) => moveNode(node.id, x, y)}
                />
              )
            case 'condition':
              return (
                <ConditionNode
                  key={node.id}
                  node={node}
                  onMove={(x, y) => moveNode(node.id, x, y)}
                />
              )
            default:
              return null
          }
        })}
      </div>
      <ZoomControls
        zoom={zoom}
        onZoomIn={() => setZoom((z) => Math.min(2, z + 0.2))}
        onZoomOut={() => setZoom((z) => Math.max(0.2, z - 0.2))}
        onResetZoom={() => { setZoom(1); setPan({ x: 0, y: 0 }) }}
      />
    </div>
  )
}
