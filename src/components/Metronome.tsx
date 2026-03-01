import { useProjectStore } from '../store/projectStore';

export function Metronome() {
  const { isPlaying } = useProjectStore();
  
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Metronome:</span>
      <button 
        className={`px-2 py-1 rounded text-sm ${isPlaying ? 'bg-green-700' : 'bg-gray-700'}`}
        onClick={() => {
          // Metronome click using Web Audio API
          if (isPlaying) {
            const ctx = new (window.AudioContext || (window as any).webkitAudioContext)();
            const osc = ctx.createOscillator();
            const gain = ctx.createGain();
            osc.connect(gain);
            gain.connect(ctx.destination);
            osc.frequency.value = 1000;
            gain.gain.setValueAtTime(0.1, ctx.currentTime);
            gain.gain.exponentialRampToValueAtTime(0.01, ctx.currentTime + 0.05);
            osc.start(ctx.currentTime);
            osc.stop(ctx.currentTime + 0.05);
          }
        }}
      >
        🔔
      </button>
    </div>
  );
}
