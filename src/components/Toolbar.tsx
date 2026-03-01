import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';
import { useProjectStore } from '../store/projectStore';
import { Presets } from './Presets';
import { QuantizeSelector } from './QuantizeSelector';
import { NoteLengthSelector } from './NoteLengthSelector';
import { OctaveShift } from './OctaveShift';
import { VelocitySlider } from './VelocitySlider';
import { Metronome } from './Metronome';
import { GridSelector } from './GridSelector';
import { TransposeTool } from './TransposeTool';
import { TrackControls } from './TrackControls';
import { ZoomControls } from './ZoomControls';
import { AboutDialog } from './AboutDialog';
import { PlaybackRate } from './PlaybackRate';
import { TimeSignature } from './TimeSignature';
import { PianoKeyboard } from './PianoKeyboard';
import { ChordLibrary } from './ChordLibrary';
import { ScaleLibrary } from './ScaleLibrary';
import { DrumPatterns } from './DrumPatterns';
import { LoopRegion } from './LoopRegion';
import { TapTempo } from './TapTempo';
import { MarkerList } from './MarkerList';
import { EffectRack } from './EffectRack';
import { Mixer } from './Mixer';
import { MasterVolume } from './MasterVolume';
import { PianoMode } from './PianoMode';
import { ColorPicker } from './ColorPicker';
import { ThemeToggle } from './ThemeToggle';
import { MIDILearn } from './MIDILearn';
import { ExportMenu } from './ExportMenu';
import { UserPreset } from './UserPreset';
import { AutomationLane } from './AutomationLane';
import { LyricEditor } from './LyricEditor';
import { VocalMode } from './VocalMode';
import { PianorollTool } from './PianorollTool';
import { SnapOptions } from './SnapOptions';
import { VelocityCurve } from './VelocityCurve';
import { QuantizeStrength } from './QuantizeStrength';
import { TimeStretch } from './TimeStretch';

export function Toolbar() {
  const { isPlaying, setPlaying, project, setProject, undo, redo, canUndo, canRedo } = useProjectStore();
  const [loading, setLoading] = useState(false);
  
  const handlePlay = async () => {
    try {
      setLoading(true);
      if (isPlaying) { await invoke('stop_audio'); } 
      else { await invoke('play_audio'); }
      setPlaying(!isPlaying);
    } catch (e) { console.error('Playback error:', e); }
    finally { setLoading(false); }
  };
  
  const handleStop = async () => {
    try { await invoke('stop_audio'); setPlaying(false); }
    catch (e) { console.error('Stop error:', e); }
  };
  
  const handleImportMidi = async () => {
    try {
      const selected = await open({ multiple: false, filters: [{ name: 'MIDI', extensions: ['mid', 'midi'] }] });
      if (selected) { const proj = await invoke('import_midi', { path: selected }); setProject(proj as any); }
    } catch (e) { console.error('Import error:', e); }
  };
  
  const handleExportMidi = async () => {
    try {
      const path = await save({ filters: [{ name: 'MIDI', extensions: ['mid', 'midi'] }], defaultPath: `${project.name}.mid` });
      if (path) {
        const ustxProject = { name: project.name, bpm: project.bpm, beatPerBar: project.beatPerBar, beatUnit: project.beatUnit, tempo: project.tempo, tracks: project.tracks, project: { voiceDir: null, singer: null, expressions: {} }, Version: 'Resonance' };
        await invoke('export_midi', { path, project: ustxProject });
      }
    } catch (e) { console.error('Export error:', e); }
  };
  
  return (
    <div className="flex items-center gap-2 px-4 py-2 bg-gray-800 border-b border-gray-700">
      <div className="flex items-center gap-1">
        <button onClick={undo} disabled={!canUndo()} className="px-2 py-1.5 bg-gray-700 hover:bg-gray-600 disabled:opacity-30 rounded text-white text-sm">↩</button>
        <button onClick={redo} disabled={!canRedo()} className="px-2 py-1.5 bg-gray-700 hover:bg-gray-600 disabled:opacity-30 rounded text-white text-sm">↪</button>
      </div>
      <div className="h-6 w-px bg-gray-600 mx-1" />
      <button onClick={handleImportMidi} className="px-3 py-1.5 bg-gray-700 hover:bg-gray-600 rounded text-white text-sm">Import</button>
      <button onClick={handleExportMidi} className="px-3 py-1.5 bg-green-700 hover:bg-green-600 rounded text-white text-sm">Export</button>
      <div className="h-6 w-px bg-gray-600 mx-1" />
      <button onClick={handlePlay} disabled={loading} className="px-3 py-1.5 bg-blue-600 hover:bg-blue-700 disabled:opacity-50 rounded text-white text-sm">{isPlaying ? '⏹ Stop' : '▶ Play'}</button>
      <button onClick={handleStop} className="px-3 py-1.5 bg-gray-700 hover:bg-gray-600 rounded text-white text-sm">⏹</button>
      <div className="h-6 w-px bg-gray-600 mx-2" />
      <div className="flex items-center gap-2">
        <span className="text-gray-400 text-sm">BPM:</span>
        <input type="number" value={project.bpm} className="w-16 px-2 py-1 bg-gray-700 border border-gray-600 rounded text-white text-sm" readOnly />
      </div>
      <Presets />
      <QuantizeSelector />
      <NoteLengthSelector />
      <OctaveShift />
      <VelocitySlider />
      <Metronome />
      <GridSelector />
      <TransposeTool />
      <TrackControls />
      <ZoomControls />
      <PlaybackRate />
      <TimeSignature />
      <PianoKeyboard />
      <ChordLibrary />
      <ScaleLibrary />
      <DrumPatterns />
      <LoopRegion />
      <TapTempo />
      <MarkerList />
      <EffectRack />
      <Mixer />
      <MasterVolume />
      <PianoMode />
      <ColorPicker />
      <ThemeToggle />
      <MIDILearn />
      <ExportMenu />
      <UserPreset />
      <AutomationLane />
      <LyricEditor />
      <VocalMode />
      <PianorollTool />
      <SnapOptions />
      <VelocityCurve />
      <QuantizeStrength />
      <TimeStretch />
      <button 
        onClick={() => document.getElementById('about-dialog')?.classList.toggle('hidden')}
        className="px-3 py-1.5 bg-gray-700 hover:bg-gray-600 rounded text-white text-sm ml-4"
      >
        About
      </button>
      <div className="flex-1" />
      <span className="text-gray-400 text-sm">{project.name}</span>
      <div id="about-dialog" className="hidden">
        <AboutDialog />
      </div>
    </div>
  );
}
