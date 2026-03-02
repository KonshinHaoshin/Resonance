import { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useProjectStore } from '../store/projectStore';

export function AudioPlayer() {
  const audioContextRef = useRef<AudioContext | null>(null);
  const audioBufferRef = useRef<AudioBuffer | null>(null);
  const audioSourceRef = useRef<AudioBufferSourceNode | null>(null);
  const gainNodeRef = useRef<GainNode | null>(null);
  const analyserRef = useRef<AnalyserNode | null>(null);
  
  const { 
    isPlaying,
    setPlaying,
    project
  } = useProjectStore();
  
  const [volume, setVolume] = useState(0.8);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Initialize audio context
  useEffect(() => {
    if (!audioContextRef.current) {
      try {
        audioContextRef.current = new (window.AudioContext || (window as any).webkitAudioContext)();
        
        // Create audio nodes
        if (audioContextRef.current) {
          gainNodeRef.current = audioContextRef.current.createGain();
          analyserRef.current = audioContextRef.current.createAnalyser();
          
          // Connect nodes
          if (gainNodeRef.current && analyserRef.current) {
            gainNodeRef.current.connect(analyserRef.current);
            analyserRef.current.connect(audioContextRef.current.destination);
          }
          
          // Set initial volume
          if (gainNodeRef.current) {
            gainNodeRef.current.gain.value = volume;
          }
        }
      } catch (err) {
        setError(`Audio context error: ${err}`);
        console.error('Audio context error:', err);
      }
    }

    return () => {
      // Cleanup
      if (audioSourceRef.current) {
        audioSourceRef.current.stop();
        audioSourceRef.current = null;
      }
    };
  }, [volume]);

  // Handle play/pause
  useEffect(() => {
    if (!audioContextRef.current || !audioBufferRef.current) return;

    if (isPlaying) {
      playAudio();
    } else {
      stopAudio();
    }
  }, [isPlaying]);

  // Load audio samples from backend
  const loadAudioSamples = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      // Get audio samples from backend
      const samples = await invoke<number[]>('get_audio_samples');
      
      if (!audioContextRef.current || samples.length === 0) {
        setError('No audio samples available');
        setIsLoading(false);
        return;
      }

      // Convert Float32Array to AudioBuffer
      const audioContext = audioContextRef.current;
      const buffer = audioContext.createBuffer(2, samples.length / 2, audioContext.sampleRate);
      
      // Copy data to channels
      const leftChannel = buffer.getChannelData(0);
      const rightChannel = buffer.getChannelData(1);
      
      for (let i = 0; i < samples.length; i += 2) {
        const sampleIndex = i / 2;
        if (i < samples.length) leftChannel[sampleIndex] = samples[i];
        if (i + 1 < samples.length) rightChannel[sampleIndex] = samples[i + 1];
      }
      
      audioBufferRef.current = buffer;
      setIsLoading(false);
      
    } catch (err) {
      setError(`Failed to load audio: ${err}`);
      setIsLoading(false);
      console.error('Audio load error:', err);
    }
  };

  // Render project to audio
  const renderProject = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      const sampleCount = await invoke<number>('render_project', { project });
      console.log(`Rendered ${sampleCount} samples`);
      
      // Load the rendered samples
      await loadAudioSamples();
      
    } catch (err) {
      setError(`Render failed: ${err}`);
      setIsLoading(false);
    }
  };

  // Play audio
  const playAudio = () => {
    if (!audioContextRef.current || !audioBufferRef.current || !gainNodeRef.current) {
      setError('Audio not initialized');
      return;
    }

    if (audioSourceRef.current) {
      // Already playing
      return;
    }

    try {
      const audioContext = audioContextRef.current;
      const source = audioContext.createBufferSource();
      source.buffer = audioBufferRef.current;
      source.connect(gainNodeRef.current);
      
      // Set playback rate (for BPM adjustment)
      source.playbackRate.value = 1.0;
      
      // Handle playback end
      source.onended = () => {
        audioSourceRef.current = null;
        setPlaying(false);
      };
      
      // Start playback
      source.start();
      audioSourceRef.current = source;
      
    } catch (err) {
      setError(`Playback error: ${err}`);
      setPlaying(false);
    }
  };

  // Stop audio
  const stopAudio = () => {
    if (audioSourceRef.current) {
      try {
        audioSourceRef.current.stop();
      } catch (err) {
        // Ignore "already stopped" errors
      }
      audioSourceRef.current = null;
    }
  };

  // Generate test tone
  const generateTestTone = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      // Generate a simple test tone
      await invoke('play_audio');
      
      // Load the generated tone
      await loadAudioSamples();
      
    } catch (err) {
      setError(`Test tone failed: ${err}`);
      setIsLoading(false);
    }
  };

  return (
    <div className="flex flex-col gap-4 p-4 bg-gray-800 rounded-lg">
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-semibold text-white">Audio Player</h2>
        
        <div className="flex items-center gap-2">
          <span className="text-sm text-gray-400">
            {audioBufferRef.current 
              ? `${(audioBufferRef.current.duration).toFixed(1)}s` 
              : 'No audio'}
          </span>
        </div>
      </div>

      {error && (
        <div className="p-2 bg-red-900/50 text-red-300 text-sm rounded">
          {error}
        </div>
      )}

      <div className="flex flex-wrap gap-2">
        <button
          onClick={() => renderProject()}
          disabled={isLoading}
          className="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-700 disabled:cursor-not-allowed text-white rounded transition-colors"
        >
          {isLoading ? 'Rendering...' : 'Render Project'}
        </button>
        
        <button
          onClick={() => generateTestTone()}
          disabled={isLoading}
          className="px-4 py-2 bg-purple-600 hover:bg-purple-700 disabled:bg-gray-700 disabled:cursor-not-allowed text-white rounded transition-colors"
        >
          {isLoading ? 'Generating...' : 'Test Tone'}
        </button>
        
        <button
          onClick={() => loadAudioSamples()}
          disabled={isLoading}
          className="px-4 py-2 bg-gray-700 hover:bg-gray-600 disabled:bg-gray-800 disabled:cursor-not-allowed text-white rounded transition-colors"
        >
          {isLoading ? 'Loading...' : 'Reload Audio'}
        </button>
      </div>

      <div className="flex items-center gap-4">
        <div className="flex-1">
          <label className="block text-sm text-gray-400 mb-1">Volume</label>
          <input
            type="range"
            min="0"
            max="1"
            step="0.01"
            value={volume}
            onChange={(e) => {
              const newVolume = parseFloat(e.target.value);
              setVolume(newVolume);
              if (gainNodeRef.current) {
                gainNodeRef.current.gain.value = newVolume;
              }
            }}
            className="w-full"
          />
          <div className="text-xs text-gray-500 mt-1">{Math.round(volume * 100)}%</div>
        </div>
      </div>

      <div className="h-24 bg-gray-900 rounded p-2">
        <div className="text-xs text-gray-500 mb-1">Audio Visualization</div>
        <div className="w-full h-full flex items-center justify-center text-gray-600 text-sm">
          {audioBufferRef.current ? (
            <div className="text-center">
              <div className="text-gray-300">Audio loaded</div>
              <div className="text-xs text-gray-500">
                {audioBufferRef.current.sampleRate.toLocaleString()}Hz, {audioBufferRef.current.length.toLocaleString()} samples
              </div>
            </div>
          ) : (
            <div className="text-center">
              <div className="text-gray-400">No audio data</div>
              <div className="text-xs text-gray-600">Render or load audio to preview</div>
            </div>
          )}
        </div>
      </div>

      <div className="text-xs text-gray-500">
        <div>Audio Context: {audioContextRef.current ? 'Ready' : 'Not ready'}</div>
        <div>Buffer: {audioBufferRef.current ? 'Loaded' : 'Empty'}</div>
        <div>Playing: {isPlaying ? 'Yes' : 'No'}</div>
      </div>
    </div>
  );
}