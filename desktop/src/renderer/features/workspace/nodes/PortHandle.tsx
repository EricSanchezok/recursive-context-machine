import type { Port } from '../../../types/graph'

interface PortHandleProps {
  port: Port
  side: 'left' | 'right'
  label?: string
}

export function PortHandle({ port, side, label }: PortHandleProps) {
  return (
    <div
      title={`${port.direction}: ${port.name}`}
      style={{
        position: 'absolute',
        top: 36 + port.index * 22,
        [side]: -6,
        display: 'flex',
        alignItems: 'center',
        gap: 6,
        flexDirection: side === 'left' ? 'row' : 'row-reverse',
        pointerEvents: 'auto',
      }}
    >
      <span
        style={{
          width: 12,
          height: 12,
          borderRadius: 999,
          background: port.channel === 'pulse' ? 'var(--primary)' : '#8b5cf6',
          border: '2px solid white',
          boxShadow: '0 2px 7px rgba(15,23,42,0.18)',
        }}
      />
      {label && (
        <span
          style={{
            fontSize: 10,
            color: 'var(--muted-foreground)',
            background: 'rgba(255,255,255,0.9)',
            padding: '1px 4px',
            borderRadius: 5,
          }}
        >
          {label}
        </span>
      )}
    </div>
  )
}
