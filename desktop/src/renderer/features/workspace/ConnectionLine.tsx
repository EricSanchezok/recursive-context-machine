interface ConnectionLineProps {
  from: { x: number; y: number }
  to: { x: number; y: number }
}

export function ConnectionLine({ from, to }: ConnectionLineProps) {
  const dx = Math.max(60, Math.abs(to.x - from.x) * 0.45)
  const d = `M ${from.x} ${from.y} C ${from.x + dx} ${from.y}, ${to.x - dx} ${to.y}, ${to.x} ${to.y}`
  return (
    <path
      d={d}
      fill="none"
      stroke="var(--primary)"
      strokeWidth={2}
      strokeLinecap="round"
      opacity={0.55}
    />
  )
}
