use crate::audio::buffer::AudioBuffer;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use tracing::{info, warn, error};

/// Resampler trait - generates audio from phonemes
pub trait Resampler: Send + Sync {
    /// Get the resampler name
    fn name(&self) -> &str;

    /// Get the resampler description
    fn description(&self) -> &str {
        ""
    }

    /// Resample a phoneme to audio
    fn resample(
        &self,
        phoneme: &str,
        note_pitch: u8,
        velocity: u8,
        duration: u64,
    ) -> AudioBuffer;

    /// Check if this resampler supports the given phoneme
    fn supports_phoneme(&self, phoneme: &str) -> bool;

    /// Get list of supported phonemes
    fn supported_phonemes(&self) -> Vec<&str> {
        vec!["a", "i", "u", "e", "o", "N", "n", "m", "r", "l", "k", "g", "s", "z", "t", "d", "n", "h", "b", "p", "m", "j", "w", "v", "f", "th", "dh", "sh", "ch", "jh", "ng", "aa", "ae", "ah", "ao", "aw", "ay", "eh", "er", "ey", "ih", "iy", "ow", "oy", "uh", "uw"]
    }
}

/// Resampler configuration
#[derive(Debug, Clone)]
pub struct ResamplerConfig {
    /// Path to resampler executable or DLL
    pub path: String,
    /// Resampler arguments
    pub args: Vec<String>,
    /// Whether to use this resampler as default
    pub is_default: bool,
    /// Phoneme map (alias -> phoneme)
    pub phoneme_map: HashMap<String, String>,
}

impl Default for ResamplerConfig {
    fn default() -> Self {
        Self {
            path: String::new(),
            args: vec![],
            is_default: false,
            phoneme_map: HashMap::new(),
        }
    }
}

/// Resampler manager - manages multiple resamplers
pub struct ResamplerManager {
    resamplers: HashMap<String, Box<dyn Resampler>>,
    default_resampler: Option<String>,
}

impl ResamplerManager {
    pub fn new() -> Self {
        Self {
            resamplers: HashMap::new(),
            default_resampler: None,
        }
    }

    /// Register a resampler
    pub fn register(&mut self, name: String, resampler: Box<dyn Resampler>) {
        info!("Registering resampler: {}", name);
        self.resamplers.insert(name.clone(), resampler);
        
        // Set as default if first one
        if self.default_resampler.is_none() {
            self.default_resampler = Some(name);
        }
    }

    /// Get resampler by name
    pub fn get(&self, name: &str) -> Option<&dyn Resampler> {
        self.resamplers.get(name).map(|r| r.as_ref() as &dyn Resampler)
    }

    /// Get default resampler
    pub fn get_default(&self) -> Option<&dyn Resampler> {
        self.default_resampler.as_ref().and_then(|name| self.get(name))
    }

    /// List all resamplers
    pub fn list(&self) -> Vec<&str> {
        self.resamplers.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for ResamplerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Built-in resamplers
pub mod builtin {
    use super::*;

    /// Simple sine wave resampler (for testing)
    pub struct SineResampler {
        sample_rate: u32,
    }

    impl SineResampler {
        pub fn new(sample_rate: u32) -> Self {
            Self { sample_rate }
        }
    }

    impl Resampler for SineResampler {
        fn name(&self) -> &str {
            "Sine"
        }

        fn description(&self) -> &str {
            "Simple sine wave resampler for testing"
        }

        fn resample(
            &self,
            _phoneme: &str,
            note_pitch: u8,
            velocity: u8,
            duration: u64,
        ) -> AudioBuffer {
            let mut buffer = AudioBuffer::new(self.sample_rate, 2);
            
            let freq = 440.0 * 2.0_f64.powf((note_pitch as f64 - 69.0) / 12.0);
            let samples = (self.sample_rate as f64 * duration as f64 / 480.0) as usize;
            let velocity_factor = (velocity as f32 / 127.0) * 0.5;

            // Apply envelope (ADSR-ish)
            let attack = (samples as f32 * 0.05) as usize;
            let decay = (samples as f32 * 0.1) as usize;
            let sustain = samples - attack - decay;

            for i in 0..samples {
                let t = i as f64 / self.sample_rate as f64;
                let envelope = if i < attack {
                    i as f32 / attack as f32
                } else if i < attack + decay {
                    let decay_progress = (i - attack) as f32 / decay as f32;
                    1.0 - (decay_progress * 0.3)
                } else {
                    1.0
                };
                
                let sample = ((2.0 * std::f64::consts::PI * freq * t).sin() * velocity_factor as f64) * envelope as f64;
                buffer.push_stereo(sample as f32, sample as f32);
            }

            buffer
        }

        fn supports_phoneme(&self, _phoneme: &str) -> bool {
            true
        }
    }

    /// Triangle wave resampler
    pub struct TriangleResampler {
        sample_rate: u32,
    }

    impl TriangleResampler {
        pub fn new(sample_rate: u32) -> Self {
            Self { sample_rate }
        }
    }

    impl Resampler for TriangleResampler {
        fn name(&self) -> &str {
            "Triangle"
        }

        fn description(&self) -> &str {
            "Triangle wave resampler"
        }

        fn resample(
            &self,
            _phoneme: &str,
            note_pitch: u8,
            velocity: u8,
            duration: u64,
        ) -> AudioBuffer {
            let mut buffer = AudioBuffer::new(self.sample_rate, 2);
            
            let freq = 440.0 * 2.0_f64.powf((note_pitch as f64 - 69.0) / 12.0);
            let samples = (self.sample_rate as f64 * duration as f64 / 480.0) as usize;
            let velocity_factor = (velocity as f32 / 127.0) * 0.5;

            for i in 0..samples {
                let t = i as f64 / self.sample_rate as f64;
                let phase = (freq * t) % 1.0;
                let sample = (2.0 * (phase * 2.0 * std::f64::consts::PI).sin().abs() - 1.0) 
                    * velocity_factor as f64;
                buffer.push_stereo(sample as f32, sample as f32);
            }

            buffer
        }

        fn supports_phoneme(&self, _phoneme: &str) -> bool {
            true
        }
    }

    /// Sawtooth wave resampler
    pub struct SawtoothResampler {
        sample_rate: u32,
    }

    impl SawtoothResampler {
        pub fn new(sample_rate: u32) -> Self {
            Self { sample_rate }
        }
    }

    impl Resampler for SawtoothResampler {
        fn name(&self) -> &str {
            "Sawtooth"
        }

        fn description(&self) -> &str {
            "Sawtooth wave resampler"
        }

        fn resample(
            &self,
            _phoneme: &str,
            note_pitch: u8,
            velocity: u8,
            duration: u64,
        ) -> AudioBuffer {
            let mut buffer = AudioBuffer::new(self.sample_rate, 2);
            
            let freq = 440.0 * 2.0_f64.powf((note_pitch as f64 - 69.0) / 12.0);
            let samples = (self.sample_rate as f64 * duration as f64 / 480.0) as usize;
            let velocity_factor = (velocity as f32 / 127.0) * 0.3;

            for i in 0..samples {
                let t = i as f64 / self.sample_rate as f64;
                let phase = (freq * t) % 1.0;
                let sample = (2.0 * phase - 1.0) * velocity_factor as f64;
                buffer.push_stereo(sample as f32, sample as f32);
            }

            buffer
        }

        fn supports_phoneme(&self, _phoneme: &str) -> bool {
            true
        }
    }

    /// WORLDLINE-R compatible resampler placeholder
    /// In a full implementation, this would use actual WORLD analysis/synthesis
    pub struct WorldlineResampler {
        sample_rate: u32,
    }

    impl WorldlineResampler {
        pub fn new(sample_rate: u32) -> Self {
            Self { sample_rate }
        }
    }

    impl Resampler for WorldlineResampler {
        fn name(&self) -> &str {
            "WORLDLINE-R"
        }

        fn description(&self) -> &str {
            "WORLD-based resampler (placeholder - uses sine wave)"
        }

        fn resample(
            &self,
            phoneme: &str,
            note_pitch: u8,
            velocity: u8,
            duration: u64,
        ) -> AudioBuffer {
            let mut buffer = AudioBuffer::new(self.sample_rate, 2);
            
            // Simplified: generate different waveforms based on phoneme
            // In real implementation, would use WORLD F0, spectral envelope, etc.
            let base_freq = 440.0 * 2.0_f64.powf((note_pitch as f64 - 69.0) / 12.0);
            
            // Add slight pitch variation based on phoneme
            let phoneme_hash = phoneme.bytes().fold(0u64, |acc, b| acc.wrapping_add(b as u64));
            let pitch_mod = 1.0 + (phoneme_hash % 50) as f64 / 1000.0;
            let freq = base_freq * pitch_mod;
            
            let samples = (self.sample_rate as f64 * duration as f64 / 480.0) as usize;
            let velocity_factor = (velocity as f32 / 127.0) * 0.4;

            for i in 0..samples {
                let t = i as f64 / self.sample_rate as f64;
                
                // Generate harmonics based on phoneme
                let fundamental = (2.0 * std::f64::consts::PI * freq * t).sin();
                let harmonic2 = (4.0 * std::f64::consts::PI * freq * t).sin() * 0.3;
                let harmonic3 = (6.0 * std::f64::consts::PI * freq * t).sin() * 0.1;
                
                let sample = (fundamental + harmonic2 + harmonic3) * velocity_factor as f64;
                buffer.push_stereo(sample as f32, sample as f32);
            }

            buffer
        }

        fn supports_phoneme(&self, _phoneme: &str) -> bool {
            true
        }
    }
}

/// External resampler - wraps command-line resampler
pub struct ExternalResampler {
    name: String,
    path: String,
    args: Vec<String>,
    sample_rate: u32,
}

impl ExternalResampler {
    pub fn new(name: String, path: String, args: Vec<String>, sample_rate: u32) -> Self {
        Self {
            name,
            path,
            args,
            sample_rate,
        }
    }
}

impl Resampler for ExternalResampler {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        "External resampler"
    }

    fn resample(
        &self,
        phoneme: &str,
        note_pitch: u8,
        velocity: u8,
        duration: u64,
    ) -> AudioBuffer {
        // Build command
        let mut cmd = Command::new(&self.path);
        
        // Add arguments
        for arg in &self.args {
            cmd.arg(arg);
        }
        
        // Add phoneme-specific arguments
        cmd.arg(phoneme);
        cmd.arg(note_pitch.to_string());
        cmd.arg(velocity.to_string());
        cmd.arg(duration.to_string());
        cmd.arg(self.sample_rate.to_string());
        
        // Execute and capture output
        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    // Parse output as f32 samples
                    let samples: Vec<f32> = output.stdout
                        .chunks_exact(4)
                        .map(|chunk| {
                            f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                        })
                        .collect();
                    
                    let mut buffer = AudioBuffer::new(self.sample_rate, 2);
                    for chunk in samples.chunks(2) {
                        if chunk.len() == 2 {
                            buffer.push_stereo(chunk[0], chunk[1]);
                        } else if chunk.len() == 1 {
                            buffer.push_stereo(chunk[0], chunk[0]);
                        }
                    }
                    return buffer;
                } else {
                    warn!("External resampler failed: {:?}", output.stderr);
                }
            }
            Err(e) => {
                error!("Failed to run external resampler: {}", e);
            }
        }
        
        // Fallback to empty buffer
        AudioBuffer::new(self.sample_rate, 2)
    }

    fn supports_phoneme(&self, phoneme: &str) -> bool {
        // Could check with --help or a config file
        // For now, accept all
        !phoneme.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sine_resampler() {
        let resampler = builtin::SineResampler::new(44100);
        let buffer = resampler.resample("a", 60, 100, 480);
        assert!(!buffer.is_empty());
    }

    #[test]
    fn test_worldline_resampler() {
        let resampler = builtin::WorldlineResampler::new(44100);
        let buffer = resampler.resample("a", 60, 100, 480);
        assert!(!buffer.is_empty());
    }

    #[test]
    fn test_resampler_manager() {
        let mut manager = ResamplerManager::new();
        manager.register("sine".to_string(), Box::new(builtin::SineResampler::new(44100)));
        
        assert!(manager.get("sine").is_some());
        assert!(manager.get_default().is_some());
    }
}
