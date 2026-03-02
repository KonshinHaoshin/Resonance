import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useProjectStore } from '../store/projectStore';

export function TransportBar() {
  const { 
    isPlaying, 
    setPlaying, 
    currentTick, 
    setCurrentTick, 
    project,
    setProject
  } = useProjectStore();
  
  const [loopEnabled, setLoopEnabled] = useState(false);
  const [loopStart, setLoopStart] = useState(0);
  const [loopEnd, setLoopEnd] = useState(1920);
  const [metronomeEnabled, setMetronomeEnabled] = useState(false);
  const [position, setPosition] = useState({ bars: 1, beats: 1, ticks: 0 });
  const animationRef = useRef<number | null>(null);
  const lastTimeRef = useRef<number>(0);
  
  const ticksPerBeat = 480;
  
  // Update position display
  useEffect(() => {
    const totalBeats = Math.floor(currentTick / ticksPerBeat);
    const bars = Math.floor(totalBeats / project.beatPerBar) + 1;
    const beats = (totalBeats % project.beatPerBar) + 1;
    const ticks = currentTick % ticksPerBeat;
    setPosition({ bars, beats, ticks });
  }, [currentTick, project.beatPerBar, ticksPerBeat]);
  
  // Playback animation loop
  useEffect(() => {
    if (isPlaying) {
      const tickRate = (project.bpm * ticksPerBeat) / 60; // ticks per second
      
      const animate = (time: number) => {
        if (lastTimeRef.current === 0) {
          lastTimeRef.current = time;
        }
        
        const delta = (time - lastTimeRef.current) / 1000; // convert to seconds
        lastTimeRef.current = time;
        
        const deltaTicks = Math.round(delta * tickRate);
        let newTick = currentTick + deltaTicks;
        
        // Loop handling
        if (loopEnabled && newTick >= loopEnd) {
          newTick = loopStart;
        }
        
        // Stop at end of project
        const maxTick = Math.max(4000, ...project.tracks.flatMap(t => t.notes.map(n => n.start + n.duration)));
        if (newTick >= maxTick) {
          setPlaying(false);
          setCurrentTick(0);
          lastTimeRef.current = 0;
          return;
        }
        
        setCurrentTick(newTick);
        animationRef.current = requestAnimationFrame(animate);
      };
      
      lastTimeRef.current = 0;
      animationRef.current = requestAnimationFrame(animate);
    } else {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
        animationRef.current = null;
      }
      lastTimeRef.current = 0;
    }
    
    return () => {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
    };
  }, [isPlaying, project.bpm, loopEnabled, loopStart, loopEnd, setPlaying, setCurrentTick, currentTick, ticksPerBeat, project.tracks]);
  
  const handlePlay = async () => {
    try {
      if (isPlaying) {
        await invoke('stop_audio');
      } else {
        await invoke('play_audio');
      }
      setPlaying(!isPlaying);
    } catch (e) {
      console.error('Playback error:', e);
      setPlaying(!isPlaying);
    }
  };
  
  const handleStop = async () => {
    try {
      await invoke('stop_audio');
      setPlaying(false);
      setCurrentTick(0);
    } catch (e) {
      console.error('Stop error:', e);
      setPlaying(false);
      setCurrentTick(0);
    }
  };
  
  const handlePause = async () => {
    try {
      await invoke('stop_audio');
      setPlaying(false);
    } catch (e) {
      console.error('Pause error:', e);
    }
  };
  
  const handleRewind = () => {
    setCurrentTick(0);
  };
  
  const handleForward = () => {
    const maxTick = Math.max(4000, ...project.tracks.flatMap(t => t.notes.map(n => n.start + n.duration)));
    setCurrentTick(maxTick);
  };
  
  const handleBpmChange = (newBpm: number) => {
    setProject({ ...project, bpm: Math.max(20, Math.min(300, newBpm)) });
  };
  
  const formatTime = () => {
    const seconds = Math.round(currentTick / ticksPerBeat / project.bpm * 60);
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };
  
  return (
    <div className="flex items-center gap-4 px-4 py-2 bg-gray-900 border-b border-gray-700">
      {/* Transport Controls */}
      <div className="flex items-center gap-1">
        <button
          onClick={handleRewind}
          className="p-2 hover:bg-gray-700 rounded text-gray-300"
          title="Rewind (Home)"
        >
          ⏮
        </button>
        
        <button
          onClick={handleStop}
          className="p-2 hover:bg-gray-700 rounded text-gray-300"
          title="Stop"
        >
          ⏹
        </button>
        
        <button
          onClick={handlePlay}
          className={`p-2 rounded text-white ${isPlaying ? 'bg-blue-600 hover:bg-blue-700' : 'bg-green-600 hover:bg-green-700'}`}
          title={isPlaying ? 'Pause' : 'Play (Space)'}
        >
          {isPlaying ? '⏸' : '▶'}
        </button>
        
        <button
          onClick={handlePause}
          className="p-2 hover:bg-gray-700 rounded text-gray-300"
          title="Pause"
        >
          ⏸
        </button>
        
        <button
          onClick={handleForward}
          className="p-2 hover:bg-gray-700 rounded text-gray-300"
          title="Forward to End"
        >
          ⏭
        </button>
      </div>
      
      <div className="h-6 w-px bg-gray-600" />
      
      {/* Position Display */}
      <div className="flex items-center gap-2 font-mono text-sm">
        <span className="text-blue-400 min-w-[60px]">
          {position.bars}:{position.beats}:{Math.floor(position.ticks / 48)}
        </span>
        <span className="text-gray-500">|</span>
        <span className="text-gray-400">{formatTime()}</span>
      </div>
      
      <div className="h-6 w-px bg-gray-600" />
      
      {/* BPM Control */}
      <div className="flex items-center gap-2">
        <button
          onClick={() => handleBpmChange(project.bpm - 1)}
          className="p-1 hover:bg-gray-700 rounded text-gray-400"
        >
          -
        </button>
        <div className="flex flex-col items-center">
          <label className="text-[10px] text-gray-500">BPM</label>
          <input
            type="number"
            value={project.bpm}
            onChange={(e) => handleBpmChange(parseInt(e.target.value) || 120)}
            className="w-14 px-1 py-0.5 bg-gray-800 border border-gray-600 rounded text-white text-sm text-center"
            min={20}
            max={300}
          />
        </div>
        <button
          onClick={() => handleBpmChange(project.bpm + 1)}
          className="p-1 hover:bg-gray-700 rounded text-gray-400"
        >
          +
        </button>
      </div>
      
      {/* Time Signature */}
      <div className="flex items-center gap-1 ml-2">
        <span className="text-gray-500 text-xs">Time:</span>
        <select
          value={`${project.beatPerBar}/${project.beatUnit}`}
          onChange={(e) => {
            const [beats, unit] = e.target.value.split('/').map(Number);
            setProject({ ...project, beatPerBar: beats, beatUnit: unit });
          }}
          className="px-2 py-1 bg-gray-800 border border-gray-600 rounded text-white text-sm"
        >
          <option value="4/4">4/4</option>
          <option value="3/4">3/4</option>
          <option value="6/8">6/8</option>
          <option value="5/4">5/4</option>
          <option value="7/8">7/8</option>
        </select>
      </div>
      
      <div className="h-6 w-px bg-gray-600" />
      
      {/* Loop Controls */}
      <div className="flex items-center gap-2">
        <button
          onClick={() => setLoopEnabled(!loopEnabled)}
          className={`px-2 py-1 rounded text-xs ${loopEnabled ? 'bg-green-600 text-white' : 'bg-gray-700 text-gray-400'}`}
          title="Loop"
        >
          Loop
        </button>
        {loopEnabled && (
          <>
            <input
              type="number"
              value={Math.round(loopStart / ticksPerBeat)}
              onChange={(e) => setLoopStart(parseInt(e.target.value) * ticksPerBeat)}
              className="w-12 px-1 py-0.5 bg-gray-800 border border-gray-600 rounded text-white text-xs"
              placeholder="Start"
            />
            <span className="text-gray-500">→</span>
            <input
              type="number"
              value={Math.round(loopEnd / ticksPerBeat)}
              onChange={(e) => setLoopEnd(parseInt(e.target.value) * ticksPerBeat)}
              className="w-12 px-1 py-0.5 bg-gray-800 border border-gray-600 rounded text-white text-xs"
              placeholder="End"
            />
          </>
        )}
      </div>
      
      {/* Metronome */}
      <div className="flex items-center gap-2 ml-auto">
        <button
          onClick={() => setMetronomeEnabled(!metronomeEnabled)}
          className={`px-2 py-1 rounded text-xs ${metronomeEnabled ? 'bg-orange-600 text-white' : 'bg-gray-700 text-gray-400'}`}
          title="Metronome"
        >
          ♩ {metronomeEnabled ? 'ON' : 'OFF'}
        </button>
      </div>
    </div>
  );
}
