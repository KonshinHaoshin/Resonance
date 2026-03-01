import { useProjectStore } from '../store/projectStore';

export function Presets() {
  const presets = [
    { name: 'Pop', bpm: 120 },
    { name: 'Ballad', bpm: 70 },
    { name: 'Dance', bpm: 128 },
    { name: 'Rock', bpm: 140 },
  ];
  
  const handleApply = (bpm: number) => {
    const { project, setProject } = useProjectStore.getState();
    setProject({ ...project, bpm });
  };
  
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Preset:</span>
      <select
        onChange={(e) => handleApply(parseInt(e.target.value))}
        className="px-2 py-1 bg-gray-700 border border-gray-600 rounded text-white text-sm"
        defaultValue=""
      >
        <option value="" disabled>Select...</option>
        {presets.map(p => (
          <option key={p.name} value={p.bpm}>{p.name} ({p.bpm})</option>
        ))}
      </select>
    </div>
  );
}
