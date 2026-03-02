import { create } from 'zustand';
import type { Project, Note, Track } from '../types';

interface HistoryState {
  past: Project[];
  future: Project[];
}

type VocalMode = 'CV' | 'VCV' | 'VC';

interface ProjectState {
  project: Project;
  currentTrackIndex: number;
  selectedNotes: number[];
  isPlaying: boolean;
  currentTick: number;
  history: HistoryState;
  vocalMode: VocalMode;
  setProject: (project: Project) => void;
  setCurrentTrack: (index: number) => void;
  addTrack: (track: Track) => void;
  updateTrack: (index: number, updates: Partial<Track>) => void;
  addNote: (trackIndex: number, note: Note) => void;
  updateNote: (trackIndex: number, noteIndex: number, note: Partial<Note>) => void;
  deleteNote: (trackIndex: number, noteIndex: number) => void;
  setPlaying: (playing: boolean) => void;
  selectNote: (noteIndex: number) => void;
  clearSelection: () => void;
  setCurrentTick: (tick: number) => void;
  setVocalMode: (mode: VocalMode) => void;
  undo: () => void;
  redo: () => void;
  canUndo: () => boolean;
  canRedo: () => boolean;
}

const defaultProject: Project = {
  name: 'Untitled',
  bpm: 120,
  beatPerBar: 4,
  beatUnit: 4,
  tempo: [{ position: 0, bpm: 120 }],
  tracks: [{ name: 'Track 1', notes: [], volume: 0.8, pan: 0, effects: [] }]
};

const MAX_HISTORY = 50;

export const useProjectStore = create<ProjectState>((set, get) => ({
  project: defaultProject,
  currentTrackIndex: 0,
  selectedNotes: [],
  isPlaying: false,
  currentTick: 0,
  history: { past: [], future: [] },
  vocalMode: 'CV',
  
  setProject: (project) => set({ project }),
  
  setCurrentTrack: (index) => set({ currentTrackIndex: index }),
  
  addTrack: (track) => set((state) => {
    const newProject = { ...state.project, tracks: [...state.project.tracks, track] };
    return {
      project: newProject,
      history: { past: [...state.history.past.slice(-MAX_HISTORY + 1), state.project], future: [] }
    };
  }),
  
  updateTrack: (trackIndex, trackUpdates) => set((state) => {
    const tracks = [...state.project.tracks];
    tracks[trackIndex] = { ...tracks[trackIndex], ...trackUpdates };
    const newProject = { ...state.project, tracks };
    return {
      project: newProject,
      history: { past: [...state.history.past.slice(-MAX_HISTORY + 1), state.project], future: [] }
    };
  }),
  
  addNote: (trackIndex, note) => set((state) => {
    const tracks = [...state.project.tracks];
    tracks[trackIndex] = { 
      ...tracks[trackIndex], 
      notes: [...tracks[trackIndex].notes, note].sort((a, b) => a.start - b.start) 
    };
    const newProject = { ...state.project, tracks };
    return {
      project: newProject,
      history: { past: [...state.history.past.slice(-MAX_HISTORY + 1), state.project], future: [] }
    };
  }),
  
  updateNote: (trackIndex, noteIndex, noteUpdate) => set((state) => {
    const tracks = [...state.project.tracks];
    const notes = [...tracks[trackIndex].notes];
    notes[noteIndex] = { ...notes[noteIndex], ...noteUpdate };
    const newProject = { ...state.project, tracks: tracks.map((t, i) => i === trackIndex ? { ...t, notes } : t) };
    return {
      project: newProject,
      history: { past: [...state.history.past.slice(-MAX_HISTORY + 1), state.project], future: [] }
    };
  }),
  
  deleteNote: (trackIndex, noteIndex) => set((state) => {
    const tracks = [...state.project.tracks];
    tracks[trackIndex] = {
      ...tracks[trackIndex],
      notes: tracks[trackIndex].notes.filter((_, i) => i !== noteIndex)
    };
    const newProject = { ...state.project, tracks };
    return {
      project: newProject,
      history: { past: [...state.history.past.slice(-MAX_HISTORY + 1), state.project], future: [] },
      selectedNotes: []
    };
  }),
  
  setPlaying: (playing) => set({ isPlaying: playing }),
  
  selectNote: (noteIndex) => set({ selectedNotes: [noteIndex] }),
  
  clearSelection: () => set({ selectedNotes: [] }),
  
  setCurrentTick: (tick) => set({ currentTick: tick }),
  
  setVocalMode: (mode) => set({ vocalMode: mode }),
  
  undo: () => set((state) => {
    if (state.history.past.length === 0) return state;
    const previous = state.history.past[state.history.past.length - 1];
    return {
      project: previous,
      history: {
        past: state.history.past.slice(0, -1),
        future: [state.project, ...state.history.future]
      }
    };
  }),
  
  redo: () => set((state) => {
    if (state.history.future.length === 0) return state;
    const next = state.history.future[0];
    return {
      project: next,
      history: {
        past: [...state.history.past, state.project],
        future: state.history.future.slice(1)
      }
    };
  }),
  
  canUndo: () => get().history.past.length > 0,
  
  canRedo: () => get().history.future.length > 0
}));
