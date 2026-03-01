export function MasterVolume() {
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Master:</span>
      <input 
        type="range" 
        min={0} 
        max={100} 
        defaultValue={80}
        className="w-20" 
      />
      <span className="text-gray-400 text-xs">80%</span>
    </div>
  );
}
