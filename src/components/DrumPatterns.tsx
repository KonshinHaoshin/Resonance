export function DrumPatterns() {
  const patterns = [
    { name: 'Kick', sound: 36 },
    { name: 'Snare', sound: 38 },
    { name: 'HiHat', sound: 42 },
    { name: 'Clap', sound: 39 },
  ];
  
  const playDrum = (midiNote: number) => {
    const ctx = new (window.AudioContext || (window as any).webkitAudioContext)();
    const osc = ctx.createOscillator();
    const gain = ctx.createGain();
    osc.connect(gain);
    gain.connect(ctx.destination);
    
    const freq = 440 * Math.pow(2, (midiNote - 69) / 12);
    osc.frequency.value = freq;
    osc.type = 'square';
    gain.gain.setValueAtTime(0.3, ctx.currentTime);
    gain.gain.exponentialRampToValueAtTime(0.01, ctx.currentTime + 0.1);
    osc.start(ctx.currentTime);
    osc.stop(ctx.currentTime + 0.1);
  };
  
  return (
    <div className="flex items-center gap-1 ml-4">
      <span className="text-gray-400 text-sm">Drums:</span>
      {patterns.map(d => (
        <button
          key={d.name}
          onClick={() => playDrum(d.sound)}
          className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs"
        >
          {d.name}
        </button>
      ))}
    </div>
  );
}
