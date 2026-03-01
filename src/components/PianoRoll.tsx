import { useRef, useEffect, useState, useCallback } from 'react';
import { useProjectStore } from '../store/projectStore';

const NOTE_HEIGHT = 16;
const MIN_PITCH = 36;
const MAX_PITCH = 84;

export function PianoRoll() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { project, currentTrackIndex, selectedNotes, selectNote, addNote, clearSelection } = useProjectStore();
  const track = project.tracks[currentTrackIndex];
  const [scrollX, setScrollX] = useState(0);
  const [tickWidth, setTickWidth] = useState(0.5);
  
  const render = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    
    const width = 1400;
    const height = (MAX_PITCH - MIN_PITCH + 1) * NOTE_HEIGHT + 24;
    
    ctx.fillStyle = '#1e1e1e';
    ctx.fillRect(0, 0, width, height);
    
    // Pitch lines
    const noteNames = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
    for (let p = MIN_PITCH; p <= MAX_PITCH; p++) {
      const y = 24 + (MAX_PITCH - p) * NOTE_HEIGHT;
      const isBlack = [1, 3, 6, 8, 10].includes(p % 12);
      ctx.fillStyle = isBlack ? '#2a2a2a' : '#252525';
      ctx.fillRect(0, y, width, NOTE_HEIGHT);
      
      if (p % 12 === 0 || p % 12 === 5) {
        ctx.fillStyle = '#666';
        ctx.font = '10px monospace';
        ctx.fillText(`${noteNames[p % 12]}${Math.floor(p / 12) - 1}`, 4, y + NOTE_HEIGHT - 4);
      }
    }
    
    // Beat lines
    const ticksPerBeat = 480;
    const ticksPerBar = ticksPerBeat * project.beatPerBar;
    const totalTicks = Math.max(2000, ...track.notes.map(n => n.start + n.duration));
    
    for (let t = 0; t <= totalTicks; t += ticksPerBeat) {
      const x = t * tickWidth - scrollX;
      if (x < 0 || x > width) continue;
      ctx.strokeStyle = t % ticksPerBar === 0 ? '#444' : '#333';
      ctx.beginPath();
      ctx.moveTo(x, 24);
      ctx.lineTo(x, height);
      ctx.stroke();
    }
    
    // Notes
    track.notes.forEach((note, i) => {
      const x = note.start * tickWidth - scrollX;
      const y = 24 + (MAX_PITCH - note.pitch) * NOTE_HEIGHT;
      const w = note.duration * tickWidth;
      
      if (x + w < 0 || x > width) return;
      
      const isSelected = selectedNotes.includes(i);
      const color = track.color || '#3b82f6';
      
      ctx.fillStyle = isSelected ? '#60a5fa' : color;
      ctx.beginPath();
      ctx.roundRect(Math.max(0, x) + 1, y + 1, Math.min(w - 2, width - x - 2), NOTE_HEIGHT - 2, 2);
      ctx.fill();
      
      if (isSelected) {
        ctx.strokeStyle = '#fff';
        ctx.lineWidth = 1;
        ctx.stroke();
      }
    });
  }, [track, selectedNotes, scrollX, tickWidth, project.beatPerBar]);
  
  useEffect(() => {
    render();
  }, [render]);
  
  const handleClick = (e: React.MouseEvent) => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return;
    
    const x = e.clientX - rect.left + scrollX;
    const y = e.clientY - rect.top - 24;
    
    if (y < 0) return;
    
    const pitch = MAX_PITCH - Math.floor(y / NOTE_HEIGHT);
    const start = Math.floor(x / tickWidth / 10) * 10;
    
    const clickedIndex = track.notes.findIndex(
      n => pitch === n.pitch && start >= n.start && start < n.start + n.duration
    );
    
    if (clickedIndex >= 0) {
      selectNote(clickedIndex);
    } else if (pitch >= MIN_PITCH && pitch <= MAX_PITCH && start >= 0) {
      clearSelection();
    }
  };
  
  const handleDoubleClick = (e: React.MouseEvent) => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return;
    
    const x = e.clientX - rect.left + scrollX;
    const y = e.clientY - rect.top - 24;
    
    if (y < 0) return;
    
    const pitch = MAX_PITCH - Math.floor(y / NOTE_HEIGHT);
    const start = Math.floor(x / tickWidth / 10) * 10;
    
    if (pitch >= MIN_PITCH && pitch <= MAX_PITCH && start >= 0) {
      addNote(currentTrackIndex, { pitch, velocity: 100, start, duration: 480 });
    }
  };
  
  const handleWheel = (e: React.WheelEvent) => {
    if (e.ctrlKey) {
      e.preventDefault();
      // Zoom
      const delta = e.deltaY > 0 ? -0.1 : 0.1;
      setTickWidth(prev => Math.max(0.1, Math.min(2, prev + delta)));
    } else if (e.shiftKey) {
      setScrollX(prev => Math.max(0, prev - e.deltaY));
    }
  };
  
  return (
    <div className="flex-1 overflow-hidden bg-gray-800 flex flex-col">
      <div className="h-6 bg-gray-700 border-b border-gray-600 flex items-center justify-between px-2">
        <span className="text-xs text-gray-400">Piano Roll - {track.name}</span>
        <span className="text-xs text-gray-500">Zoom: {Math.round(tickWidth * 100)}% | Scroll: Shift+Wheel</span>
      </div>
      <div className="overflow-auto flex-1">
        <canvas
          ref={canvasRef}
          width={1400}
          height={(MAX_PITCH - MIN_PITCH + 1) * NOTE_HEIGHT + 24}
          onClick={handleClick}
          onDoubleClick={handleDoubleClick}
          onWheel={handleWheel}
          className="cursor-crosshair"
        />
      </div>
    </div>
  );
}
