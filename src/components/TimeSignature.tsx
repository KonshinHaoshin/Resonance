export function TimeSignature() {
  const signatures = ['4/4', '3/4', '6/8', '2/4', '5/4', '7/8'];
  
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Time:</span>
      <select className="px-2 py-1 bg-gray-700 border border-gray-600 rounded text-white text-sm">
        {signatures.map(sig => (
          <option key={sig} value={sig}>{sig}</option>
        ))}
      </select>
    </div>
  );
}
