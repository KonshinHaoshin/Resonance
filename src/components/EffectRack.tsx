export function EffectRack() {
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">FX:</span>
      {['Reverb', 'Delay', 'Chorus', 'EQ'].map(fx => (
        <button
          key={fx}
          className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs"
        >
          {fx}
        </button>
      ))}
    </div>
  );
}
