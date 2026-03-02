import { useState } from 'react';
import { useProjectStore } from '../store/projectStore';

interface ChannelStripProps {
  trackIndex: number;
  name: string;
  volume: number;
  pan: number;
  color?: string;
}

function ChannelStrip({ trackIndex, name, volume, pan, color }: ChannelStripProps) {
  const { updateTrack, currentTrackIndex, setCurrentTrack } = useProjectStore();
  const [isMuted, setIsMuted] = useState(false);
  const [isSolo, setIsSolo] = useState(false);
  
  const isSelected = currentTrackIndex === trackIndex;
  
  const handleVolumeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    updateTrack(trackIndex, { volume: parseFloat(e.target.value) });
  };
  
  const handlePanChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    updateTrack(trackIndex, { pan: parseFloat(e.target.value) });
  };
  
  const meterHeight = volume * 100;
  const panPosition = ((pan + 1) / 2) * 100;
  
  return (
    <div 
      className={`flex flex-col items-center p-2 rounded-lg min-w-[80px] ${
        isSelected ? 'bg-gray-700 ring-2 ring-blue-500' : 'bg-gray-800 hover:bg-gray-750'
      }`}
      onClick={() => setCurrentTrack(trackIndex)}
    >
      {/* Track Name */}
      <div 
        className="text-xs font-medium text-center mb-2 truncate w-full px-1"
        style={{ color: color || '#9ca3af' }}
      >
        {name}
      </div>
      
      {/* Pan Knob */}
      <div className="flex flex-col items-center mb-2">
        <span className="text-[10px] text-gray-500 mb-1">PAN</span>
        <div className="relative w-8 h-8">
          <div 
            className="absolute inset-0 rounded-full border-2 border-gray-600"
            style={{
              background: `conic-gradient(from 225deg, #3b82f6 ${panPosition}%, #6b7280 ${panPosition}%)`
            }}
          />
          <input
            type="range"
            min="-1"
            max="1"
            step="0.01"
            value={pan}
            onChange={handlePanChange}
            className="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
          />
          <div className="absolute inset-1 rounded-full bg-gray-800 flex items-center justify-center">
            <div 
              className="w-0.5 h-3 bg-white rounded-full origin-bottom"
              style={{ transform: `rotate(${pan * 135}deg)`, transformOrigin: 'bottom' }}
            />
          </div>
        </div>
        <span className="text-[10px] text-gray-400 mt-1">
          {pan === 0 ? 'C' : pan < 0 ? `L${Math.abs(Math.round(pan * 100))}` : `R${Math.round(pan * 100)}`}
        </span>
      </div>
      
      {/* Volume Fader & Meter */}
      <div className="flex items-center gap-1 h-32">
        {/* Meter */}
        <div className="w-3 h-full bg-gray-900 rounded relative overflow-hidden">
          <div 
            className="absolute bottom-0 w-full transition-all duration-75"
            style={{ 
              height: `${meterHeight}%`,
              background: meterHeight > 90 ? '#ef4444' : meterHeight > 75 ? '#f59e0b' : '#22c55e'
            }}
          />
          {/* Peak indicator */}
          {meterHeight > 90 && (
            <div className="absolute top-0 w-full h-0.5 bg-red-500" />
          )}
        </div>
        
        {/* Fader */}
        <div className="relative h-full">
          <input
            type="range"
            min="0"
            max="1"
            step="0.01"
            value={volume}
            onChange={handleVolumeChange}
            className="h-full w-6 appearance-none bg-transparent cursor-pointer"
            style={{ writingMode: 'vertical-lr', direction: 'rtl' }}
          />
          <div className="absolute left-1 top-0 bottom-0 w-2 bg-gray-700 rounded">
            <div 
              className="absolute bottom-0 w-full bg-blue-500 rounded"
              style={{ height: `${volume * 100}%` }}
            />
            <div 
              className="absolute left-1/2 -translate-x-1/2 w-4 h-2 bg-gray-300 rounded-sm cursor-grab active:cursor-grabbing"
              style={{ bottom: `calc(${volume * 100}% - 4px)` }}
            />
          </div>
        </div>
      </div>
      
      {/* dB Display */}
      <div className="text-[10px] text-gray-400 mt-1 font-mono">
        {volume === 0 ? '-∞' : `${(20 * Math.log10(volume)).toFixed(1)}dB`}
      </div>
      
      {/* Mute/Solo */}
      <div className="flex gap-1 mt-2">
        <button
          onClick={(e) => { e.stopPropagation(); setIsMuted(!isMuted); }}
          className={`px-2 py-0.5 text-[10px] font-bold rounded ${
            isMuted ? 'bg-red-600 text-white' : 'bg-gray-700 text-gray-400 hover:bg-gray-600'
          }`}
        >
          M
        </button>
        <button
          onClick={(e) => { e.stopPropagation(); setIsSolo(!isSolo); }}
          className={`px-2 py-0.5 text-[10px] font-bold rounded ${
            isSolo ? 'bg-yellow-600 text-white' : 'bg-gray-700 text-gray-400 hover:bg-gray-600'
          }`}
        >
          S
        </button>
      </div>
    </div>
  );
}

export function Mixer() {
  const { project } = useProjectStore();
  const [isExpanded, setIsExpanded] = useState(false);
  
  return (
    <div className="bg-gray-900 border-t border-gray-700">
      {/* Mixer Header */}
      <div 
        className="flex items-center justify-between px-4 py-1 bg-gray-800 cursor-pointer"
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <div className="flex items-center gap-2">
          <span className="text-sm font-medium text-gray-300">Mixer</span>
          <span className="text-xs text-gray-500">
            {project.tracks.length} tracks
          </span>
        </div>
        <button className="text-gray-400 hover:text-white">
          {isExpanded ? '▼' : '▲'}
        </button>
      </div>
      
      {/* Mixer Channels */}
      {isExpanded && (
        <div className="flex items-start gap-2 p-4 overflow-x-auto">
          {project.tracks.map((track, index) => (
            <ChannelStrip
              key={index}
              trackIndex={index}
              name={track.name}
              volume={track.volume ?? 0.8}
              pan={track.pan ?? 0}
              color={track.color}
            />
          ))}
          
          {/* Master Channel */}
          <div className="flex flex-col items-center p-2 rounded-lg min-w-[80px] bg-gradient-to-b from-gray-800 to-gray-900 border border-gray-600 ml-4">
            <div className="text-xs font-medium text-center mb-2 text-purple-400">MASTER</div>
            
            {/* Master Meter */}
            <div className="flex items-center gap-1 h-32">
              <div className="w-3 h-full bg-gray-900 rounded relative overflow-hidden">
                <div className="absolute bottom-0 w-full bg-purple-500" style={{ height: '70%' }} />
              </div>
              <div className="w-3 h-full bg-gray-900 rounded relative overflow-hidden">
                <div className="absolute bottom-0 w-full bg-purple-500" style={{ height: '68%' }} />
              </div>
            </div>
            
            <div className="text-[10px] text-gray-400 mt-1 font-mono">0.0dB</div>
          </div>
        </div>
      )}
    </div>
  );
}
