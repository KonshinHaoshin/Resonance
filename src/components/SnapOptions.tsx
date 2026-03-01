export function SnapOptions() {
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Snap:</span>
      {['Off', 'Bar', 'Beat', '1/4', '1/8', '1/16'].map(s => (
        <button key={s} className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs">
          {s}
        </button>
      ))}
    </div>
  );
}
