import { useProjectStore } from '../store/projectStore';

type VocalMode = 'CV' | 'VCV' | 'VC';

const MODE_INFO: Record<VocalMode, { label: string; desc: string }> = {
  CV: { label: 'CV', desc: 'Constant Velocity - Fixed velocity' },
  VCV: { label: 'VCV', desc: 'Velocity as Volume - Velocity controls volume' },
  VC: { label: 'VC', desc: 'Velocity Control - Velocity controls synthesis' }
};

export function VocalMode() {
  const { vocalMode, setVocalMode } = useProjectStore();
  
  const modes: VocalMode[] = ['CV', 'VCV', 'VC'];
  
  return (
    <div className="flex items-center gap-1 ml-4" title="Vocal Mode">
      <span className="text-gray-400 text-sm">Vocal:</span>
      <div className="flex gap-0.5">
        {modes.map(mode => (
          <button
            key={mode}
            onClick={() => setVocalMode(mode)}
            className={`px-2 py-1 rounded text-xs font-medium transition-colors ${
              vocalMode === mode
                ? 'bg-blue-600 text-white'
                : 'bg-gray-700 hover:bg-gray-600 text-gray-300'
            }`}
            title={MODE_INFO[mode].desc}
          >
            {MODE_INFO[mode].label}
          </button>
        ))}
      </div>
    </div>
  );
}
