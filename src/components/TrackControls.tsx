import { useProjectStore } from '../store/projectStore';

export function TrackControls() {
  const { project, setProject } = useProjectStore();
  
  const toggleMute = (trackIndex: number) => {
    const tracks = [...project.tracks];
    const track = { ...tracks[trackIndex] };
    // Use a flag to mark as muted
    (track as any).muted = !(track as any).muted;
    tracks[trackIndex] = track;
    setProject({ ...project, tracks });
  };
  
  const toggleSolo = (trackIndex: number) => {
    const tracks = [...project.tracks];
    const track = { ...tracks[trackIndex] };
    (track as any).solo = !(track as any).solo;
    tracks[trackIndex] = track;
    setProject({ ...project, tracks });
  };
  
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Track:</span>
      {project.tracks.map((track, i) => (
        <div key={i} className="flex items-center gap-1">
          <button
            onClick={() => toggleMute(i)}
            className={`px-2 py-1 rounded text-xs ${(track as any).muted ? 'bg-red-700' : 'bg-gray-700'}`}
          >
            M
          </button>
          <button
            onClick={() => toggleSolo(i)}
            className={`px-2 py-1 rounded text-xs ${(track as any).solo ? 'bg-yellow-700' : 'bg-gray-700'}`}
          >
            S
          </button>
        </div>
      ))}
    </div>
  );
}
