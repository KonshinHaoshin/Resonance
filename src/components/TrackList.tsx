import { useProjectStore } from '../store/projectStore';

export function TrackList() {
  const { project, currentTrackIndex, setCurrentTrack, addTrack } = useProjectStore();
  const colors = ['#ef4444', '#f97316', '#eab308', '#22c55e', '#3b82f6', '#8b5cf6'];
  
  return (
    <div className="w-48 bg-gray-900 border-r border-gray-700 flex flex-col">
      <div className="flex items-center justify-between px-3 py-2 border-b border-gray-700">
        <span className="text-gray-300 text-sm font-medium">Tracks</span>
        <button onClick={() => addTrack({ name: `Track ${project.tracks.length + 1}`, notes: [], volume: 0.8, pan: 0, effects: [] })} className="text-gray-400 hover:text-white">+</button>
      </div>
      {project.tracks.map((track, i) => (
        <button
          key={i}
          onClick={() => setCurrentTrack(i)}
          className={`w-full flex items-center gap-2 px-3 py-2 text-left ${i === currentTrackIndex ? 'bg-gray-700 text-white' : 'text-gray-400 hover:bg-gray-800'}`}
        >
          <div className="w-3 h-3 rounded-full" style={{ backgroundColor: colors[i % colors.length] }} />
          <span className="text-sm">{track.name}</span>
        </button>
      ))}
    </div>
  );
}
