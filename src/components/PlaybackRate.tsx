export function PlaybackRate() {
  const rates = [0.25, 0.5, 0.75, 1, 1.25, 1.5, 2];
  
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Speed:</span>
      {rates.map(rate => (
        <button 
          key={rate}
          className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs"
        >
          {rate}x
        </button>
      ))}
    </div>
  );
}
