export function TimeStretch() {
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Stretch:</span>
      <input type="range" min="50" max="200" defaultValue="100" className="w-16" />
      <span className="text-gray-400 text-xs">100%</span>
    </div>
  );
}
