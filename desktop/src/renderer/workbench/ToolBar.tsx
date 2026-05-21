import { Play, Plus, Network, GitMerge, GitFork, ArrowRight } from 'lucide-react'

interface ToolBarProps {
  onAddAccelerator: () => void
  onAddFlux: () => void
  onAddCondition: () => void
  onStartWire: () => void
  onRun: () => void
  isRunning: boolean
}

export function ToolBar({
  onAddAccelerator,
  onAddFlux,
  onAddCondition,
  onStartWire,
  onRun,
  isRunning,
}: ToolBarProps) {
  return (
    <div
      className="h-12 flex items-center px-3 gap-4 border-t"
      style={{ backgroundColor: 'var(--card)', borderColor: 'var(--border)' }}
    >
      {/* 左侧构建工具 */}
      <div className="flex items-center gap-1">
        <ToolButton icon={<Network size={16} />} label="Accelerator" onClick={onAddAccelerator} />
        <ToolButton icon={<GitMerge size={16} />} label="Flux" onClick={onAddFlux} />
        <ToolButton icon={<GitFork size={16} />} label="Condition" onClick={onAddCondition} />
        <div className="w-px h-6 bg-gray-300 mx-2" />
        <ToolButton icon={<ArrowRight size={16} />} label="Wire" onClick={onStartWire} />
      </div>

      <div className="flex-1" />

      {/* 右侧运行 */}
      <button
        onClick={onRun}
        disabled={isRunning}
        className="flex items-center gap-2 px-4 py-1.5 rounded-lg text-white text-sm font-medium
                   transition-colors disabled:opacity-50"
        style={{ backgroundColor: 'var(--primary)' }}
      >
        <Play size={16} />
        {isRunning ? 'Running…' : 'Run'}
      </button>
    </div>
  )
}

function ToolButton({
  icon,
  label,
  onClick,
}: {
  icon: React.ReactNode
  label: string
  onClick: () => void
}) {
  return (
    <button
      onClick={onClick}
      className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-sm
                 hover:bg-gray-100 transition-colors"
      style={{ color: 'var(--foreground)' }}
    >
      {icon}
      {label}
    </button>
  )
}
