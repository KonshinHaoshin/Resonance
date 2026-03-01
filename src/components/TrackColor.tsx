export function TrackColor() {
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Track:</span>
      {['Red', 'Orange', 'Yellow', 'Green', 'Blue', 'Purple'].map(c => (
        <button key={c} className="w-5 h-5 rounded" style={{ backgroundColor: c.toLowerCase() }} />
      ))}
    </div>
  );
}
