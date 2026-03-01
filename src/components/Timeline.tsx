import { useProjectStore } from '../store/projectStore';

export function Timeline() {
  const { project, currentTick, setCurrentTick } = useProjectStore();
  
  const ticksPerSecond = (project.bpm * 480) / 60;
  const totalTicks = 10000;
  const width = 1200;
  
  const handleClick = (e: React.MouseEvent<HTMLDivElement>) => {
    const rect = e.currentTarget.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const tick = Math.floor((x / width) * totalTicks);
    setCurrentTick(Math.max(0, tick));
  };
  
  return (
    <div className="h-8 bg-gray-800 border-b border-gray-700 flex items-center relative">
      {/* Time markers */}
      <div className="absolute inset-0 flex items-end pointer-events-none">
        {Array.from({ length: 21 }).map((_, i) => (
          <div
            key={i}
            className="absolute bottom-0 text-[10px] text-gray-500"
            style={{ left: `${i * 5}%` }}
          >
            {Math.floor((i * totalTicks) / ticksPerSecond)}s
          </div>
        ))}
      </div>
      
      {/* Playhead */}
      <div
        className="absolute top-0 w-0.5 h-full bg-red-500 z-10 pointer-events-none"
        style={{ left: `${(currentTick / totalTicks) * 100}%` }}
      />
      
      {/* Clickable area */}
      <div
        className="absolute inset-0 cursor-pointer"
        onClick={handleClick}
      />
    </div>
  );
}
