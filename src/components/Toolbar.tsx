import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useProjectStore } from '../store/projectStore';
import { Presets } from './Presets';

export function Toolbar() {
  const { isPlaying, setPlaying, project } = useProjectStore();
  const [loading, setLoading] = useState(false);
  
  const handlePlay = async () => {
    try {
      setLoading(true);
      if (isPlaying) {
        await invoke('stop_audio');
      } else {
        await invoke('play_audio');
      }
      setPlaying(!isPlaying);
    } catch (e) {
      console.error('Playback error:', e);
    } finally {
      setLoading(false);
    }
  };
  
  const handleStop = async () => {
    try {
      await invoke('stop_audio');
      setPlaying(false);
    } catch (e) {
      console.error('Stop error:', e);
    }
  };
  
  return (
    <div className="flex items-center gap-2 px-4 py-2 bg-gray-800 border-b border-gray-700">
      <div className="flex items-center gap-1">
        <button
          onClick={handlePlay}
          disabled={loading}
          className="px-3 py-1.5 bg-blue-600 hover:bg-blue-700 disabled:opacity-50 rounded text-white text-sm"
        >
          {isPlaying ? '⏹ Stop' : '▶ Play'}
        </button>
        
        <button
          onClick={handleStop}
          className="px-3 py-1.5 bg-gray-700 hover:bg-gray-600 rounded text-white text-sm"
        >
          ⏹
        </button>
      </div>
      
      <div className="h-6 w-px bg-gray-600 mx-2" />
      
      <div className="flex items-center gap-2">
        <span className="text-gray-400 text-sm">BPM:</span>
        <input
          type="number"
          value={project.bpm}
          className="w-16 px-2 py-1 bg-gray-700 border border-gray-600 rounded text-white text-sm"
          readOnly
        />
      </div>
      
      <Presets />
      
      <div className="flex-1" />
      <span className="text-gray-400 text-sm">{project.name}</span>
    </div>
  );
}
