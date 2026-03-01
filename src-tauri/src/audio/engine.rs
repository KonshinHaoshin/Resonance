use std::sync::{Arc, Mutex};
use crate::audio::buffer::AudioBuffer;

/// Audio engine for synthesis
pub struct AudioEngine {
    sample_rate: u32,
    channels: u16,
    buffer: Arc<Mutex<AudioBuffer>>,
    is_playing: bool,
    current_position: u64,
}

impl AudioEngine {
    /// Create a new audio engine
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
            channels: 2,
            buffer: Arc::new(Mutex::new(AudioBuffer::new(44100, 2))),
            is_playing: false,
            current_position: 0,
        }
    }

    /// Create with custom settings
    pub fn with_settings(sample_rate: u32, channels: u16) -> Self {
        Self {
            sample_rate,
            channels,
            buffer: Arc::new(Mutex::new(AudioBuffer::new(sample_rate, channels))),
            is_playing: false,
            current_position: 0,
        }
    }

    /// Play
    pub fn play(&mut self) {
        self.is_playing = true;
    }

    /// Stop
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.current_position = 0;
    }

    /// Check if playing
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    /// Get sample rate
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Get channels
    pub fn channels(&self) -> u16 {
        self.channels
    }

    /// Get buffer
    pub fn buffer(&self) -> Arc<Mutex<AudioBuffer>> {
        Arc::clone(&self.buffer)
    }

    /// Add stereo samples
    pub fn add_samples(&self, left: f32, right: f32) {
        if let Ok(mut buf) = self.buffer.lock() {
            buf.push_stereo(left, right);
        }
    }

    /// Get current position
    pub fn position(&self) -> u64 {
        self.current_position
    }

    /// Set position
    pub fn set_position(&mut self, pos: u64) {
        self.current_position = pos;
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
    fn test_engine_creation() {
        let engine = AudioEngine::new();
        assert_eq!(engine.sample_rate(), 44100);
    }
}
