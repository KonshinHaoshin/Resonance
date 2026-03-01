export function Mixer() {
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Mixer:</span>
      {[1, 2, 3, 4].map(ch => (
        <div key={ch} className="flex flex-col items-center">
          <div className="w-6 h-24 bg-gray-700 rounded relative">
            <div className="absolute bottom-0 w-full bg-blue-500" style={{ height: '60%' }} />
          </div>
          <span className="text-gray-500 text-xs mt-1">ch{ch}</span>
        </div>
      ))}
    </div>
  );
}
