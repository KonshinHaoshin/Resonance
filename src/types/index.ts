// Resonance Types

export interface Note {
  pitch: number;
  velocity: number;
  start: number;
  duration: number;
}

export interface Effect {
  id: string;
  type: 'reverb' | 'delay' | 'eq' | 'chorus';
  enabled: boolean;
  params: Record<string, number>;
}

export interface Track {
  name: string;
  color?: string;
  notes: Note[];
  volume: number;
  pan: number;
  effects: Effect[];
}

export interface Tempo {
  position: number;
  bpm: number;
}

export interface Project {
  name: string;
  bpm: number;
  beatPerBar: number;
  beatUnit: number;
  tempo: Tempo[];
  tracks: Track[];
}
