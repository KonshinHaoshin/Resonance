use std::sync::{Arc, Mutex};
use crate::audio::buffer::AudioBuffer;

/// Core audio engine for playback and rendering
pub struct AudioEngine {
    sample_rate: u32,
    channels: u16,
    buffer: Arc<Mutex<AudioBuffer>>,
    is_playing: bool,
}

impl AudioEngine {
    /// Create a new audio engine with default settings
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
            channels: 2,
            buffer: Arc::new(Mutex::new(AudioBuffer::new(44100, 2))),
            is_playing: false,
        }
    }

    /// Create a new audio engine with custom settings
    pub fn with_settings(sample_rate: u32, channels: u16) -> Self {
        Self {
            sample_rate,
            channels,
            buffer: Arc::new(Mutex::new(AudioBuffer::new(sample_rate, channels))),
            is_playing: false,
        }
    }

    /// Get the sample rate
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Get the number of channels
    pub fn channels(&self) -> u16 {
        self.channels
    }

    /// Check if audio is currently playing
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    /// Start playback
    pub fn play(&mut self) {
        self.is_playing = true;
    }

    /// Stop playback
    pub fn stop(&mut self) {
        self.is_playing = false;
    }

    /// Get the current audio buffer
    pub fn buffer(&self) -> Arc<Mutex<AudioBuffer>> {
        Arc::clone(&self.buffer)
    }

    /// Clear the audio buffer
    pub fn clear(&self) {
        if let Ok(mut buf) = self.buffer.lock() {
            buf.clear();
        }
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_engine_creation() {
        let engine = AudioEngine::new();
        assert_eq!(engine.sample_rate(), 44100);
        assert_eq!(engine.channels(), 2);
        assert!(!engine.is_playing());
    }

    #[test]
    fn test_audio_engine_play_stop() {
        let mut engine = AudioEngine::new();
        engine.play();
        assert!(engine.is_playing());
        engine.stop();
        assert!(!engine.is_playing());
    }
}
