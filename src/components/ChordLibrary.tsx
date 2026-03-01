export function ChordLibrary() {
  const chords = [
    { name: 'C', notes: [60, 64, 67] },
    { name: 'Dm', notes: [62, 65, 69] },
    { name: 'Em', notes: [64, 67, 71] },
    { name: 'F', notes: [65, 69, 72] },
    { name: 'G', notes: [67, 71, 74] },
    { name: 'Am', notes: [69, 72, 76] },
    { name: 'Bdim', notes: [71, 74, 77] },
  ];
  
  const playChord = (notes: number[]) => {
    const ctx = new (window.AudioContext || (window as any).webkitAudioContext)();
    notes.forEach((note, i) => {
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.connect(gain);
      gain.connect(ctx.destination);
      const freq = 440 * Math.pow(2, (note - 69) / 12);
      osc.frequency.value = freq;
      gain.gain.setValueAtTime(0.2, ctx.currentTime);
      gain.gain.exponentialRampToValueAtTime(0.01, ctx.currentTime + 1);
      osc.start(ctx.currentTime + i * 0.02);
      osc.stop(ctx.currentTime + 1);
    });
  };
  
  return (
    <div className="flex items-center gap-1 ml-4">
      <span className="text-gray-400 text-sm">Chords:</span>
      {chords.map(c => (
        <button
          key={c.name}
          onClick={() => playChord(c.notes)}
          className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs"
        >
          {c.name}
        </button>
      ))}
    </div>
  );
}
