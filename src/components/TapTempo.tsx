import { useState } from 'react';

export function TapTempo() {
  const [tempo, setTempo] = useState(120);
  const [times, setTimes] = useState<number[]>([]);
  
  const handleTap = () => {
    const now = Date.now();
    const newTimes = [...times, now].slice(-8);
    setTimes(newTimes);
    
    if (newTimes.length >= 2) {
      const intervals = [];
      for (let i = 1; i < newTimes.length; i++) {
        intervals.push(newTimes[i] - newTimes[i-1]);
      }
      const avg = intervals.reduce((a, b) => a + b, 0) / intervals.length;
      const newTempo = Math.round(60000 / avg);
      if (newTempo >= 20 && newTempo <= 300) {
        setTempo(newTempo);
      }
    }
  };
  
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Tap:</span>
      <button 
        onClick={handleTap}
        className="px-3 py-1 bg-blue-700 hover:bg-blue-600 rounded text-white text-sm"
      >
        TAP
      </button>
      <span className="text-gray-400 text-xs">{tempo} BPM</span>
    </div>
  );
}
