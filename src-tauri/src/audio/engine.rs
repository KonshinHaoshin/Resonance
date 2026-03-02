use std::sync::{Arc, Mutex};
use std::path::Path;
use std::time::Duration;
use crate::audio::buffer::AudioBuffer;
use crate::format::UstxFile;
use crate::format::render::{RenderFormat, RenderConfig};
use tracing::{info, error};

/// Audio engine for synthesis with advanced playback control
pub struct AudioEngine {
    sample_rate: u32,
    channels: u16,
    buffer: Arc<Mutex<AudioBuffer>>,
    is_playing: bool,
    is_paused: bool,
    current_position: u64,
    playback_rate: f32,
    loop_enabled: bool,
    loop_start: u64,
    loop_end: u64,
}

impl AudioEngine {
    /// Create a new audio engine
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
            channels: 2,
            buffer: Arc::new(Mutex::new(AudioBuffer::new(44100, 2))),
            is_playing: false,
            is_paused: false,
            current_position: 0,
            playback_rate: 1.0,
            loop_enabled: false,
            loop_start: 0,
            loop_end: u64::MAX,
        }
    }

    /// Create with custom settings
    pub fn with_settings(sample_rate: u32, channels: u16) -> Self {
        Self {
            sample_rate,
            channels,
            buffer: Arc::new(Mutex::new(AudioBuffer::new(sample_rate, channels))),
            is_playing: false,
            is_paused: false,
            current_position: 0,
            playback_rate: 1.0,
            loop_enabled: false,
            loop_start: 0,
            loop_end: u64::MAX,
        }
    }

    /// Play - just sets the playing flag
    /// Actual playback would be handled by the frontend using Web Audio API
    pub fn play(&mut self) {
        self.is_playing = true;
        self.is_paused = false;
        info!("Audio playback started (flag set)");
    }

    /// Pause playback
    pub fn pause(&mut self) {
        if self.is_playing {
            self.is_paused = true;
        }
    }

    /// Resume from pause
    pub fn resume(&mut self) {
        if self.is_paused {
            self.is_paused = false;
        }
    }

    /// Check if paused
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    /// Stop
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.is_paused = false;
        self.current_position = 0;
        info!("Audio playback stopped");
    }

    /// Pause (legacy)
    pub fn pause(&mut self) {
        self.is_playing = false;
        info!("Audio paused");
    }

    /// Resume
    pub fn resume(&mut self) {
        self.is_playing = true;
        info!("Audio resumed");
    }

    /// Seek to specific position (in ticks)
    pub fn seek_to(&mut self, position: u64) {
        self.current_position = position;
        // Clear buffer on seek for clean playback
        if let Ok(mut buf) = self.buffer.lock() {
            buf.clear();
        }
    }

    /// Set playback rate (0.5 - 2.0)
    pub fn set_playback_rate(&mut self, rate: f32) {
        self.playback_rate = rate.clamp(0.5, 2.0);
    }

    /// Get playback rate
    pub fn playback_rate(&self) -> f32 {
        self.playback_rate
    }

    /// Enable/disable loop mode
    pub fn set_loop_enabled(&mut self, enabled: bool) {
        self.loop_enabled = enabled;
    }

    /// Check if loop is enabled
    pub fn is_loop_enabled(&self) -> bool {
        self.loop_enabled
    }

    /// Set loop region
    pub fn set_loop_region(&mut self, start: u64, end: u64) {
        self.loop_start = start;
        self.loop_end = end;
    }

    /// Get loop start position
    pub fn loop_start(&self) -> u64 {
        self.loop_start
    }

    /// Get loop end position
    pub fn loop_end(&self) -> u64 {
        self.loop_end
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

    /// Add mono sample (duplicated to stereo)
    pub fn add_sample(&self, sample: f32) {
        self.add_samples(sample, sample);
    }

    /// Get current position
    pub fn position(&self) -> u64 {
        self.current_position
    }

    /// Set position
    pub fn set_position(&mut self, pos: u64) {
        self.current_position = pos;
    }

    /// Generate test tone (sine wave)
    pub fn generate_test_tone(&self, frequency: f32, duration_secs: f32) {
        let sample_count = (self.sample_rate as f32 * duration_secs) as usize;
        let mut buffer = self.buffer.lock().unwrap();
        
        for i in 0..sample_count {
            let t = i as f32 / self.sample_rate as f32;
            let sample = (2.0 * std::f32::consts::PI * frequency * t).sin() * 0.3;
            buffer.push_stereo(sample, sample);
        }
    }

    /// Generate simple beep
    pub fn beep(&self, frequency: f32, duration_ms: u64) {
        self.generate_test_tone(frequency, duration_ms as f32 / 1000.0);
    }

    /// Clear buffer
    pub fn clear_buffer(&self) {
        if let Ok(mut buf) = self.buffer.lock() {
            buf.clear();
        }
    }

    /// Get buffer samples as Vec<f32>
    pub fn get_samples(&self) -> Vec<f32> {
        if let Ok(buf) = self.buffer.lock() {
            buf.to_vec()
        } else {
            vec![]
        }
    }

    /// Get buffer length in samples
    pub fn buffer_len(&self) -> usize {
        if let Ok(buf) = self.buffer.lock() {
            buf.len()
        } else {
            0
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
    fn test_engine_creation() {
        let engine = AudioEngine::new();
        assert_eq!(engine.sample_rate(), 44100);
        assert_eq!(engine.channels(), 2);
    }

    #[test]
    fn test_play_stop() {
        let mut engine = AudioEngine::new();
        engine.add_sample(0.5);
        engine.play();
        assert!(engine.is_playing());
        engine.stop();
        assert!(!engine.is_playing());
    }
}
