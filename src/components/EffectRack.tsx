import { useState } from 'react';
import { useProjectStore } from '../store/projectStore';
import type { Effect } from '../types';

interface EffectUnitProps {
  effect: Effect;
  onUpdate: (updates: Partial<Effect>) => void;
}

function ReverbEffect({ effect, onUpdate }: EffectUnitProps) {
  return (
    <div className="grid grid-cols-2 gap-2 text-xs">
      <div>
        <label className="text-gray-500">Room Size</label>
        <input
          type="range"
          min="0"
          max="1"
          step="0.01"
          value={effect.params.roomSize ?? 0.5}
          onChange={(e) => onUpdate({ params: { ...effect.params, roomSize: parseFloat(e.target.value) } })}
          className="w-full"
        />
      </div>
      <div>
        <label className="text-gray-500">Decay</label>
        <input
          type="range"
          min="0"
          max="1"
          step="0.01"
          value={effect.params.decay ?? 0.5}
          onChange={(e) => onUpdate({ params: { ...effect.params, decay: parseFloat(e.target.value) } })}
          className="w-full"
        />
      </div>
      <div>
        <label className="text-gray-500">Wet</label>
        <input
          type="range"
          min="0"
          max="1"
          step="0.01"
          value={effect.params.wet ?? 0.3}
          onChange={(e) => onUpdate({ params: { ...effect.params, wet: parseFloat(e.target.value) } })}
          className="w-full"
        />
      </div>
      <div>
        <label className="text-gray-500">Dry</label>
        <input
          type="range"
          min="0"
          max="1"
          step="0.01"
          value={effect.params.dry ?? 0.7}
          onChange={(e) => onUpdate({ params: { ...effect.params, dry: parseFloat(e.target.value) } })}
          className="w-full"
        />
      </div>
    </div>
  );
}

function DelayEffect({ effect, onUpdate }: EffectUnitProps) {
  return (
    <div className="grid grid-cols-2 gap-2 text-xs">
      <div>
        <label className="text-gray-500">Time</label>
        <input
          type="range"
          min="0"
          max="2000"
          step="10"
          value={effect.params.time ?? 250}
          onChange={(e) => onUpdate({ params: { ...effect.params, time: parseFloat(e.target.value) } })}
          className="w-full"
        />
        <span className="text-gray-400">{effect.params.time ?? 250}ms</span>
      </div>
      <div>
        <label className="text-gray-500">Feedback</label>
        <input
          type="range"
          min="0"
          max="0.95"
          step="0.01"
          value={effect.params.feedback ?? 0.3}
          onChange={(e) => onUpdate({ params: { ...effect.params, feedback: parseFloat(e.target.value) } })}
          className="w-full"
        />
      </div>
      <div>
        <label className="text-gray-500">Wet</label>
        <input
          type="range"
          min="0"
          max="1"
          step="0.01"
          value={effect.params.wet ?? 0.3}
          onChange={(e) => onUpdate({ params: { ...effect.params, wet: parseFloat(e.target.value) } })}
          className="w-full"
        />
      </div>
      <div>
        <label className="text-gray-500">Dry</label>
        <input
          type="range"
          min="0"
          max="1"
          step="0.01"
          value={effect.params.dry ?? 0.7}
          onChange={(e) => onUpdate({ params: { ...effect.params, dry: parseFloat(e.target.value) } })}
          className="w-full"
        />
      </div>
    </div>
  );
}

function EQEffect({ effect, onUpdate }: EffectUnitProps) {
  return (
    <div className="grid grid-cols-3 gap-2 text-xs">
      <div>
        <label className="text-gray-500">Low</label>
        <input
          type="range"
          min="-12"
          max="12"
          step="0.5"
          value={effect.params.low ?? 0}
          onChange={(e) => onUpdate({ params: { ...effect.params, low: parseFloat(e.target.value) } })}
          className="w-full"
        />
        <span className="text-gray-400">{effect.params.low ?? 0}dB</span>
      </div>
      <div>
        <label className="text-gray-500">Mid</label>
        <input
          type="range"
          min="-12"
          max="12"
          step="0.5"
          value={effect.params.mid ?? 0}
          onChange={(e) => onUpdate({ params: { ...effect.params, mid: parseFloat(e.target.value) } })}
          className="w-full"
        />
        <span className="text-gray-400">{effect.params.mid ?? 0}dB</span>
      </div>
      <div>
        <label className="text-gray-500">High</label>
        <input
          type="range"
          min="-12"
          max="12"
          step="0.5"
          value={effect.params.high ?? 0}
          onChange={(e) => onUpdate({ params: { ...effect.params, high: parseFloat(e.target.value) } })}
          className="w-full"
        />
        <span className="text-gray-400">{effect.params.high ?? 0}dB</span>
      </div>
    </div>
  );
}

function ChorusEffect({ effect, onUpdate }: EffectUnitProps) {
  return (
    <div className="grid grid-cols-2 gap-2 text-xs">
      <div>
        <label className="text-gray-500">Rate</label>
        <input
          type="range"
          min="0.1"
          max="10"
          step="0.1"
          value={effect.params.rate ?? 1.5}
          onChange={(e) => onUpdate({ params: { ...effect.params, rate: parseFloat(e.target.value) } })}
          className="w-full"
        />
        <span className="text-gray-400">{effect.params.rate ?? 1.5}Hz</span>
      </div>
      <div>
        <label className="text-gray-500">Depth</label>
        <input
          type="range"
          min="0"
          max="1"
          step="0.01"
          value={effect.params.depth ?? 0.5}
          onChange={(e) => onUpdate({ params: { ...effect.params, depth: parseFloat(e.target.value) } })}
          className="w-full"
        />
      </div>
      <div>
        <label className="text-gray-500">Wet</label>
        <input
          type="range"
          min="0"
          max="1"
          step="0.01"
          value={effect.params.wet ?? 0.5}
          onChange={(e) => onUpdate({ params: { ...effect.params, wet: parseFloat(e.target.value) } })}
          className="w-full"
        />
      </div>
      <div>
        <label className="text-gray-500">Dry</label>
        <input
          type="range"
          min="0"
          max="1"
          step="0.01"
          value={effect.params.dry ?? 0.5}
          onChange={(e) => onUpdate({ params: { ...effect.params, dry: parseFloat(e.target.value) } })}
          className="w-full"
        />
      </div>
    </div>
  );
}

function EffectSlot({ effect, onUpdate, onRemove }: { effect: Effect; onUpdate: (u: Partial<Effect>) => void; onRemove: () => void }) {
  return (
    <div className="bg-gray-800 rounded-lg p-3 border border-gray-700">
      <div className="flex items-center justify-between mb-2">
        <div className="flex items-center gap-2">
          <button
            onClick={() => onUpdate({ enabled: !effect.enabled })}
            className={`px-2 py-0.5 text-xs rounded ${
              effect.enabled ? 'bg-green-600 text-white' : 'bg-gray-700 text-gray-500'
            }`}
          >
            {effect.enabled ? 'ON' : 'OFF'}
          </button>
          <span className="text-sm font-medium text-gray-300 capitalize">{effect.type}</span>
        </div>
        <button
          onClick={onRemove}
          className="text-gray-500 hover:text-red-400 text-xs"
        >
          ✕
        </button>
      </div>
      
      {effect.enabled && (
        <div className="mt-2">
          {effect.type === 'reverb' && <ReverbEffect effect={effect} onUpdate={onUpdate} />}
          {effect.type === 'delay' && <DelayEffect effect={effect} onUpdate={onUpdate} />}
          {effect.type === 'eq' && <EQEffect effect={effect} onUpdate={onUpdate} />}
          {effect.type === 'chorus' && <ChorusEffect effect={effect} onUpdate={onUpdate} />}
        </div>
      )}
    </div>
  );
}

export function EffectRack() {
  const { project, currentTrackIndex, updateTrack } = useProjectStore();
  const [isExpanded, setIsExpanded] = useState(false);
  const [selectedTrackIndex, setSelectedTrackIndex] = useState(currentTrackIndex);
  
  const currentTrack = project.tracks[selectedTrackIndex];
  const effects = currentTrack?.effects ?? [];
  
  const addEffect = (type: Effect['type']) => {
    const newEffect: Effect = {
      id: `${type}-${Date.now()}`,
      type,
      enabled: true,
      params: type === 'reverb' ? { roomSize: 0.5, decay: 0.5, wet: 0.3, dry: 0.7 } :
               type === 'delay' ? { time: 250, feedback: 0.3, wet: 0.3, dry: 0.7 } :
               type === 'eq' ? { low: 0, mid: 0, high: 0 } :
               { rate: 1.5, depth: 0.5, wet: 0.5, dry: 0.5 }
    };
    
    updateTrack(selectedTrackIndex, { 
      effects: [...effects, newEffect]
    });
  };
  
  const updateEffect = (effectId: string, updates: Partial<Effect>) => {
    const updatedEffects = effects.map(e => 
      e.id === effectId ? { ...e, ...updates } : e
    );
    updateTrack(selectedTrackIndex, { effects: updatedEffects });
  };
  
  const removeEffect = (effectId: string) => {
    updateTrack(selectedTrackIndex, { 
      effects: effects.filter(e => e.id !== effectId) 
    });
  };
  
  const effectTypes: Effect['type'][] = ['reverb', 'delay', 'eq', 'chorus'];
  
  return (
    <div className="bg-gray-900 border-t border-gray-700">
      {/* Effect Rack Header */}
      <div 
        className="flex items-center justify-between px-4 py-1 bg-gray-800 cursor-pointer"
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <div className="flex items-center gap-2">
          <span className="text-sm font-medium text-gray-300">Effect Rack</span>
          <span className="text-xs text-gray-500">
            {effects.filter(e => e.enabled).length} active
          </span>
        </div>
        <button className="text-gray-400 hover:text-white">
          {isExpanded ? '▼' : '▲'}
        </button>
      </div>
      
      {/* Effect Rack Content */}
      {isExpanded && (
        <div className="p-4">
          {/* Track Selector */}
          <div className="flex items-center gap-2 mb-4">
            <span className="text-xs text-gray-500">Track:</span>
            <select
              value={selectedTrackIndex}
              onChange={(e) => setSelectedTrackIndex(parseInt(e.target.value))}
              className="px-2 py-1 bg-gray-800 border border-gray-600 rounded text-white text-sm"
            >
              {project.tracks.map((track, i) => (
                <option key={i} value={i}>{track.name}</option>
              ))}
            </select>
          </div>
          
          {/* Add Effect Buttons */}
          <div className="flex gap-2 mb-4">
            {effectTypes.map(type => (
              <button
                key={type}
                onClick={() => addEffect(type)}
                className="px-3 py-1.5 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs capitalize flex items-center gap-1"
              >
                <span className="text-gray-400">+</span> {type}
              </button>
            ))}
          </div>
          
          {/* Effect Chain */}
          <div className="space-y-2">
            {effects.length === 0 ? (
              <div className="text-center text-gray-500 text-sm py-4">
                No effects added. Click a button above to add effects.
              </div>
            ) : (
              effects.map(effect => (
                <EffectSlot
                  key={effect.id}
                  effect={effect}
                  onUpdate={(updates) => updateEffect(effect.id, updates)}
                  onRemove={() => removeEffect(effect.id)}
                />
              ))
            )}
          </div>
        </div>
      )}
    </div>
  );
}
