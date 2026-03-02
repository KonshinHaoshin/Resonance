import { useRef, useEffect, useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useProjectStore } from '../store/projectStore';

const NOTE_HEIGHT = 16;
const MIN_PITCH = 36;
const MAX_PITCH = 84;
const HEADER_HEIGHT = 24;
const PIANO_KEY_WIDTH = 40;

interface DragState {
  type: 'move' | 'resize-left' | 'resize-right' | 'velocity' | null;
  noteIndex: number;
  startX: number;
  startY: number;
  startTick: number;
  startPitch: number;
  originalStart: number;
  originalDuration: number;
  originalVelocity: number;
}

export function PianoRoll() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const minimapRef = useRef<HTMLCanvasElement>(null);
  const { 
    project, 
    currentTrackIndex, 
    selectedNotes, 
    selectNote, 
    addNote, 
    clearSelection,
    updateNote,
    deleteNote,
    currentTick,
    isPlaying
  } = useProjectStore();
  const track = project.tracks[currentTrackIndex];
  const [scrollX, setScrollX] = useState(0);
  const [scrollY, setScrollY] = useState(0);
  const [tickWidth, setTickWidth] = useState(0.5);
  const [snapEnabled, setSnapEnabled] = useState(true);
  const [dragState, setDragState] = useState<DragState>({
    type: null,
    noteIndex: -1,
    startX: 0,
    startY: 0,
    startTick: 0,
    startPitch: 0,
    originalStart: 0,
    originalDuration: 0,
    originalVelocity: 100
  });
  const [mousePos, setMousePos] = useState({ x: 0, y: 0 });
  const [autoScroll, setAutoScroll] = useState(true);
  const [copiedNotes, setCopiedNotes] = useState<{pitch: number; velocity: number; start: number; duration: number}[]>([]);

  // Calculate total width and height
  const totalTicks = Math.max(4000, ...track.notes.map(n => n.start + n.duration));
  const totalHeight = (MAX_PITCH - MIN_PITCH + 1) * NOTE_HEIGHT + HEADER_HEIGHT;

  // Auto-scroll to follow playhead during playback
  useEffect(() => {
    if (isPlaying && autoScroll && canvasRef.current) {
      const playheadX = PIANO_KEY_WIDTH + currentTick * tickWidth;
      const containerWidth = containerRef.current?.clientWidth || 800;
      const currentScroll = scrollX;
      
      // If playhead is near the right edge, scroll to keep it visible
      if (playheadX > currentScroll + containerWidth - 100) {
        setScrollX(Math.max(0, playheadX - 100));
      }
    }
  }, [currentTick, isPlaying, autoScroll, tickWidth, scrollX]);

  // Render minimap
  const renderMinimap = useCallback(() => {
    const canvas = minimapRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    
    const width = canvas.width;
    const height = canvas.height;
    
    // Clear
    ctx.fillStyle = '#1a1a1a';
    ctx.fillRect(0, 0, width, height);
    
    // Calculate scale
    const scaleX = width / totalTicks;
    const scaleY = height / (MAX_PITCH - MIN_PITCH + 1);
    
    // Draw notes as small rectangles
    const color = track.color || '#3b82f6';
    track.notes.forEach(note => {
      const x = note.start * scaleX;
      const y = (MAX_PITCH - note.pitch - MIN_PITCH) * scaleY;
      const w = Math.max(1, note.duration * scaleX);
      const h = Math.max(1, NOTE_HEIGHT * scaleY - 1);
      
      ctx.fillStyle = color;
      ctx.fillRect(x, y, w, h);
    });
    
    // Draw playhead position
    const playheadX = currentTick * scaleX;
    ctx.fillStyle = '#ef4444';
    ctx.fillRect(playheadX - 1, 0, 2, height);
    
    // Draw viewport rectangle
    const viewportStart = scrollX / tickWidth * scaleX;
    const viewportWidth = (containerRef.current?.clientWidth || 800) / tickWidth * scaleX;
    ctx.strokeStyle = '#666';
    ctx.lineWidth = 1;
    ctx.strokeRect(viewportStart, 0, viewportWidth, height);
  }, [track, totalTicks, currentTick, scrollX, tickWidth]);

  useEffect(() => {
    renderMinimap();
  }, [renderMinimap]);

  // Load project info from backend
  useEffect(() => {
    invoke('get_project_info')
      .then((info) => {
        console.log('Project info:', info);
      })
      .catch((err) => {
        console.log('Backend not ready or error:', err);
      });
  }, []);

  // Snap to grid
  const snapToGrid = useCallback((tick: number) => {
    if (!snapEnabled) return tick;
    const snapTicks = 120; // 16th note
    return Math.round(tick / snapTicks) * snapTicks;
  }, [snapEnabled]);

  // Convert position to grid coordinates
  const getGridPosition = useCallback((clientX: number, clientY: number) => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return null;
    
    const x = clientX - rect.left;
    const y = clientY - rect.top;
    
    if (y < HEADER_HEIGHT) return null;
    
    const pitch = MAX_PITCH - Math.floor((y - HEADER_HEIGHT) / NOTE_HEIGHT);
    const tick = Math.round((x - PIANO_KEY_WIDTH + scrollX) / tickWidth);
    
    if (pitch < MIN_PITCH || pitch > MAX_PITCH || tick < 0) return null;
    
    return { pitch, tick };
  }, [scrollX, tickWidth]);

  // Find note at position
  const getNoteAtPosition = useCallback((x: number, y: number): { noteIndex: number; type: 'move' | 'resize-left' | 'resize-right' | 'velocity' } | null => {
    const gridPos = getGridPosition(x, y);
    if (!gridPos) return null;
    
    for (let i = track.notes.length - 1; i >= 0; i--) {
      const note = track.notes[i];
      const noteX = PIANO_KEY_WIDTH + note.start * tickWidth - scrollX;
      const noteW = note.duration * tickWidth;
      
      if (gridPos.tick >= note.start && gridPos.tick < note.start + note.duration &&
          gridPos.pitch === note.pitch) {
        const relX = x - noteX;
        if (relX < 8) {
          return { noteIndex: i, type: 'resize-left' };
        } else if (relX > noteW - 8) {
          return { noteIndex: i, type: 'resize-right' };
        }
        return { noteIndex: i, type: 'move' };
      }
    }
    return null;
  }, [track.notes, scrollX, tickWidth, getGridPosition]);

  const render = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    
    const width = canvas.width;
    const height = canvas.height;
    
    // Clear
    ctx.fillStyle = '#1e1e1e';
    ctx.fillRect(0, 0, width, height);
    
    const noteNames = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
    const blackNotes = [1, 3, 6, 8, 10];
    
    // Draw piano key area
    ctx.fillStyle = '#252525';
    ctx.fillRect(0, HEADER_HEIGHT, PIANO_KEY_WIDTH, height - HEADER_HEIGHT);
    
    for (let p = MIN_PITCH; p <= MAX_PITCH; p++) {
      const y = HEADER_HEIGHT + (MAX_PITCH - p) * NOTE_HEIGHT;
      const noteInPitch = p % 12;
      const isBlack = blackNotes.includes(noteInPitch);
      
      ctx.fillStyle = isBlack ? '#2a2a2a' : '#252525';
      ctx.fillRect(PIANO_KEY_WIDTH, y, width - PIANO_KEY_WIDTH, NOTE_HEIGHT);
      
      ctx.fillStyle = isBlack ? '#3a3a3a' : '#2a2a2a';
      ctx.fillRect(0, y, PIANO_KEY_WIDTH, NOTE_HEIGHT);
      
      if (noteInPitch === 0 || noteInPitch === 5) {
        ctx.fillStyle = isBlack ? '#888' : '#aaa';
        ctx.font = '10px monospace';
        ctx.fillText(`${noteNames[noteInPitch]}${Math.floor(p / 12) - 1}`, 4, y + NOTE_HEIGHT - 4);
      }
    }
    
    // Beat lines
    const ticksPerBeat = 480;
    const ticksPerBar = ticksPerBeat * project.beatPerBar;
    
    for (let t = 0; t <= totalTicks; t += ticksPerBeat) {
      const x = PIANO_KEY_WIDTH + t * tickWidth - scrollX;
      if (x < PIANO_KEY_WIDTH || x > width) continue;
      
      ctx.strokeStyle = t % ticksPerBar === 0 ? '#444' : '#333';
      ctx.lineWidth = t % ticksPerBar === 0 ? 1 : 0.5;
      ctx.beginPath();
      ctx.moveTo(x, HEADER_HEIGHT);
      ctx.lineTo(x, height);
      ctx.stroke();
    }
    
    // Playhead
    if (isPlaying || currentTick > 0) {
      const playheadX = PIANO_KEY_WIDTH + currentTick * tickWidth - scrollX;
      if (playheadX >= PIANO_KEY_WIDTH && playheadX <= width) {
        ctx.strokeStyle = '#ef4444';
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.moveTo(playheadX, 0);
        ctx.lineTo(playheadX, height);
        ctx.stroke();
        
        ctx.fillStyle = '#ef4444';
        ctx.beginPath();
        ctx.moveTo(playheadX - 6, 0);
        ctx.lineTo(playheadX + 6, 0);
        ctx.lineTo(playheadX, 10);
        ctx.closePath();
        ctx.fill();
      }
    }
    
    // Notes
    track.notes.forEach((note, i) => {
      const x = PIANO_KEY_WIDTH + note.start * tickWidth - scrollX;
      const y = HEADER_HEIGHT + (MAX_PITCH - note.pitch) * NOTE_HEIGHT;
      const w = note.duration * tickWidth;
      
      if (x + w < PIANO_KEY_WIDTH || x > width) return;
      
      const isSelected = selectedNotes.includes(i);
      const color = track.color || '#3b82f6';
      
      // Velocity-based brightness
      const velocityFactor = 0.5 + (note.velocity / 127) * 0.5;
      ctx.fillStyle = isSelected ? '#60a5fa' : color;
      ctx.globalAlpha = velocityFactor;
      ctx.beginPath();
      const noteX = Math.max(PIANO_KEY_WIDTH + 1, x);
      const noteW = Math.min(w - 2, width - noteX - 2);
      ctx.roundRect(noteX + 1, y + 1, noteW - 2, NOTE_HEIGHT - 2, 2);
      ctx.fill();
      ctx.globalAlpha = 1;
      
      if (isSelected) {
        ctx.strokeStyle = '#fff';
        ctx.lineWidth = 2;
        ctx.stroke();
        
        ctx.fillStyle = '#fff';
        if (x >= PIANO_KEY_WIDTH) {
          ctx.fillRect(x + 2, y + 2, 4, NOTE_HEIGHT - 4);
        }
        if (x + w <= width) {
          ctx.fillRect(x + w - 6, y + 2, 4, NOTE_HEIGHT - 4);
        }
      }
      
      // Velocity indicator bar at bottom of note
      if (!isSelected) {
        const velHeight = Math.max(2, (note.velocity / 127) * NOTE_HEIGHT * 0.3);
        ctx.fillStyle = 'rgba(255, 255, 255, 0.4)';
        ctx.fillRect(noteX + 1, y + NOTE_HEIGHT - velHeight - 1, noteW - 2, velHeight);
      }
    });
    
    // Current mouse position highlight
    const mouseY = mousePos.y;
    if (mouseY >= HEADER_HEIGHT && mouseY < height) {
      const hoverPitch = MAX_PITCH - Math.floor((mouseY - HEADER_HEIGHT) / NOTE_HEIGHT);
      if (hoverPitch >= MIN_PITCH && hoverPitch <= MAX_PITCH) {
        const rowY = HEADER_HEIGHT + (MAX_PITCH - hoverPitch) * NOTE_HEIGHT;
        ctx.fillStyle = 'rgba(255, 255, 255, 0.05)';
        ctx.fillRect(PIANO_KEY_WIDTH, rowY, width - PIANO_KEY_WIDTH, NOTE_HEIGHT);
      }
    }
  }, [track, selectedNotes, scrollX, tickWidth, project.beatPerBar, currentTick, isPlaying, mousePos, totalTicks]);
  
  useEffect(() => {
    render();
  }, [render]);

  const handleMouseDown = (e: React.MouseEvent) => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return;
    
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    
    const hitResult = getNoteAtPosition(x, y);
    
    if (hitResult) {
      // Multi-select with shift
      if (e.shiftKey && !selectedNotes.includes(hitResult.noteIndex)) {
        selectNote(hitResult.noteIndex);
      } else if (!e.shiftKey && !selectedNotes.includes(hitResult.noteIndex)) {
        selectNote(hitResult.noteIndex);
      }
      
      const note = track.notes[hitResult.noteIndex];
      setDragState({
        type: hitResult.type,
        noteIndex: hitResult.noteIndex,
        startX: e.clientX,
        startY: e.clientY,
        startTick: Math.round((x - PIANO_KEY_WIDTH + scrollX) / tickWidth),
        startPitch: note.pitch,
        originalStart: note.start,
        originalDuration: note.duration,
        originalVelocity: note.velocity
      });
    } else if (x > PIANO_KEY_WIDTH) {
      clearSelection();
    }
  };

  const handleMouseMove = useCallback((e: React.MouseEvent) => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return;
    
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    setMousePos({ x, y });
    
    const hitResult = getNoteAtPosition(x, y);
    const canvas = canvasRef.current;
    if (canvas) {
      if (hitResult) {
        if (hitResult.type === 'resize-left' || hitResult.type === 'resize-right') {
          canvas.style.cursor = 'ew-resize';
        } else {
          canvas.style.cursor = 'move';
        }
      } else if (x > PIANO_KEY_WIDTH && y > HEADER_HEIGHT) {
        canvas.style.cursor = 'crosshair';
      } else {
        canvas.style.cursor = 'default';
      }
    }
    
    if (dragState.type && dragState.noteIndex >= 0) {
      const deltaX = e.clientX - dragState.startX;
      const deltaY = e.clientY - dragState.startY;
      const deltaTick = Math.round(deltaX / tickWidth);
      
      if (dragState.type === 'move') {
        const newStart = Math.max(0, snapToGrid(dragState.originalStart + deltaTick));
        const deltaPitch = -Math.round(deltaY / NOTE_HEIGHT);
        const newPitch = Math.max(MIN_PITCH, Math.min(MAX_PITCH, dragState.startPitch + deltaPitch));
        updateNote(currentTrackIndex, dragState.noteIndex, { 
          start: newStart,
          pitch: newPitch
        });
      } else if (dragState.type === 'resize-left') {
        const newStart = Math.max(0, dragState.originalStart + deltaTick);
        const newDuration = dragState.originalDuration - deltaTick;
        if (newDuration > 10) {
          updateNote(currentTrackIndex, dragState.noteIndex, { 
            start: snapToGrid(newStart), 
            duration: newDuration 
          });
        }
      } else if (dragState.type === 'resize-right') {
        const newDuration = Math.max(10, dragState.originalDuration + deltaTick);
        updateNote(currentTrackIndex, dragState.noteIndex, { duration: newDuration });
      }
    }
  }, [dragState, tickWidth, scrollX, getNoteAtPosition, updateNote, currentTrackIndex, snapToGrid]);

  const handleMouseUp = useCallback(() => {
    if (dragState.type && dragState.noteIndex >= 0) {
      const note = track.notes[dragState.noteIndex];
      invoke('create_note', {
        pitch: note.pitch,
        velocity: note.velocity,
        start: note.start,
        duration: note.duration
      }).catch(() => {});
    }
    setDragState({ type: null, noteIndex: -1, startX: 0, startY: 0, startTick: 0, startPitch: 0, originalStart: 0, originalDuration: 0, originalVelocity: 100 });
  }, [dragState, track.notes]);

  const handleDoubleClick = (e: React.MouseEvent) => {
    const gridPos = getGridPosition(e.clientX, e.clientY);
    if (!gridPos) return;
    
    const start = snapToGrid(gridPos.tick);
    
    addNote(currentTrackIndex, { 
      pitch: gridPos.pitch, 
      velocity: 100, 
      start, 
      duration: 480 
    });
    
    invoke('create_note', {
      pitch: gridPos.pitch,
      velocity: 100,
      start,
      duration: 480
    }).catch(() => {});
  };
  
  const handleWheel = (e: React.WheelEvent) => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return;
    
    if (e.ctrlKey) {
      e.preventDefault();
      const delta = e.deltaY > 0 ? -0.05 : 0.05;
      setTickWidth(prev => Math.max(0.1, Math.min(4, prev + delta)));
    } else if (e.shiftKey) {
      setScrollX(prev => Math.max(0, prev - e.deltaY));
    } else {
      setScrollY(prev => Math.max(0, prev - e.deltaY));
    }
  };

  // Minimap click to seek
  const handleMinimapClick = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const rect = minimapRef.current?.getBoundingClientRect();
    if (!rect) return;
    
    const x = e.clientX - rect.left;
    const canvas = minimapRef.current;
    if (!canvas) return;
    
    const tick = Math.floor((x / canvas.width) * totalTicks);
    const { setCurrentTick } = useProjectStore.getState();
    setCurrentTick(Math.max(0, tick));
  };

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Delete selected notes
      if (selectedNotes.length > 0 && (e.key === 'Delete' || e.key === 'Backspace')) {
        selectedNotes.forEach(noteIndex => {
          deleteNote(currentTrackIndex, noteIndex);
        });
        clearSelection();
      }
      
      // Copy notes: Ctrl+C
      if (e.code === 'KeyC' && (e.ctrlKey || e.metaKey)) {
        const notesToCopy = selectedNotes.map(i => track.notes[i]);
        setCopiedNotes(notesToCopy);
      }
      
      // Paste notes: Ctrl+V
      if (e.code === 'KeyV' && (e.ctrlKey || e.metaKey) && copiedNotes.length > 0) {
        const gridPos = getGridPosition(mousePos.x + PIANO_KEY_WIDTH - scrollX, mousePos.y + HEADER_HEIGHT - scrollY);
        if (gridPos) {
          copiedNotes.forEach((note, i) => {
            const offset = i === 0 ? 0 : note.start - copiedNotes[0].start;
            addNote(currentTrackIndex, {
              ...note,
              start: snapToGrid(gridPos.tick + offset),
              pitch: gridPos.pitch + (note.pitch - copiedNotes[0].pitch)
            });
          });
        }
      }
      
      // Select all: Ctrl+A
      if (e.code === 'KeyA' && (e.ctrlKey || e.metaKey)) {
        e.preventDefault();
        const allIndices = track.notes.map((_, i) => i);
        useProjectStore.setState({ selectedNotes: allIndices });
      }
    };
    
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [selectedNotes, currentTrackIndex, deleteNote, clearSelection, copiedNotes, track.notes, mousePos, scrollX, scrollY, getGridPosition, addNote, snapToGrid]);

  return (
    <div className="flex-1 overflow-hidden bg-gray-800 flex flex-col">
      <div className="h-6 bg-gray-700 border-b border-gray-600 flex items-center justify-between px-2">
        <span className="text-xs text-gray-400">Piano Roll - {track.name}</span>
        <div className="flex items-center gap-3 text-xs text-gray-500">
          <label className="flex items-center gap-1 cursor-pointer">
            <input
              type="checkbox"
              checked={autoScroll}
              onChange={(e) => setAutoScroll(e.target.checked)}
              className="w-3 h-3"
            />
            <span>Auto-scroll</span>
          </label>
          <button 
            onClick={() => setSnapEnabled(!snapEnabled)}
            className={`px-1 rounded ${snapEnabled ? 'text-green-400' : 'text-gray-500'}`}
          >
            Snap: {snapEnabled ? 'ON' : 'OFF'}
          </button>
          <span>|</span>
          <span>Zoom: {Math.round(tickWidth * 200)}%</span>
          <span>|</span>
          <span>Ctrl+C: Copy | Ctrl+V: Paste</span>
        </div>
      </div>
      
      {/* Minimap */}
      <div className="h-12 bg-gray-900 border-b border-gray-700 relative">
        <canvas
          ref={minimapRef}
          width={800}
          height={48}
          onClick={handleMinimapClick}
          className="w-full h-full cursor-pointer"
        />
      </div>
      
      {/* Main piano roll canvas */}
      <div ref={containerRef} className="overflow-auto flex-1 relative">
        <canvas
          ref={canvasRef}
          width={1400}
          height={totalHeight}
          onMouseDown={handleMouseDown}
          onMouseMove={handleMouseMove}
          onMouseUp={handleMouseUp}
          onMouseLeave={handleMouseUp}
          onDoubleClick={handleDoubleClick}
          onWheel={handleWheel}
          className="cursor-crosshair"
        />
      </div>
    </div>
  );
}
