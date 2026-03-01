export function PianorollTool() {
  const tools = ['Select', 'Draw', 'Erase', 'Slice', 'Glide'];
  
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Tool:</span>
      {tools.map(t => (
        <button key={t} className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs">
          {t}
        </button>
      ))}
    </div>
  );
}
