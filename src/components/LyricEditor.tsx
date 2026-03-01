export function LyricEditor() {
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Lyrics:</span>
      <input 
        type="text" 
        placeholder="Enter lyrics..."
        className="px-2 py-1 bg-gray-700 border border-gray-600 rounded text-white text-xs w-32"
      />
    </div>
  );
}
