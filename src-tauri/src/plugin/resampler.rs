use crate::audio::buffer::AudioBuffer;

/// Resampler trait - generates audio from phonemes
pub trait Resampler {
    /// Get the resampler name
    fn name(&self) -> &str;

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
}

impl Default for ResamplerConfig {
    fn default() -> Self {
        Self {
            path: String::new(),
            args: vec![],
            is_default: false,
        }
    }
}

/// Built-in WORLDLINE-R resampler
pub mod builtin {
    use super::*;

    /// WORLDLINE-R compatible resampler placeholder
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

        fn resample(
            &self,
            phoneme: &str,
            note_pitch: u8,
            velocity: u8,
            duration: u64,
        ) -> AudioBuffer {
            let mut buffer = AudioBuffer::new(self.sample_rate, 2);
            
            // Simplified: generate a sine wave based on pitch
            // In real implementation, this would use WORLD/FFT algorithms
            let freq = 440.0 * 2.0_f64.powf((note_pitch as f64 - 69.0) / 12.0);
            let samples = (self.sample_rate as f64 * duration as f64 / 480.0) as usize;
            let velocity_factor = velocity as f32 / 127.0;

            for i in 0..samples {
                let t = i as f64 / self.sample_rate as f64;
                let sample = (2.0 * std::f64::consts::PI * freq * t).sin() * velocity_factor as f64;
                buffer.push_mono(sample as f32);
            }

            buffer
        }

        fn supports_phoneme(&self, _phoneme: &str) -> bool {
            true // Accept all phonemes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worldline_resampler() {
        let resampler = builtin::WorldlineResampler::new(44100);
        let buffer = resampler.resample("a", 60, 100, 480);
        assert!(!buffer.is_empty());
    }
}
