//! Audio synthesis - waveform generators

use std::f32::consts::PI;

/// Waveform type enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WaveformType {
    Sine,
    Square,
    Sawtooth,
    Triangle,
    Pulse,
    Noise,
    White,
    Pink,
    Brown,
    SoftSquare,
    SoftSawtooth,
    SoftTriangle,
}

/// Waveform generator trait
pub trait WaveformGenerator: Send {
    /// Generate a sample at a given phase (0.0 to 1.0)
    fn sample(&self, phase: f32) -> f32;

    /// Get waveform type
    fn waveform_type(&self) -> WaveformType;
}

/// Sine wave generator
pub struct SineWave {
    pub phase: f32,
}

impl SineWave {
    pub fn new() -> Self {
        Self { phase: 0.0 }
    }

    pub fn with_phase(phase: f32) -> Self {
        Self { phase }
    }
}

impl Default for SineWave {
    fn default() -> Self {
        Self::new()
    }
}

impl WaveformGenerator for SineWave {
    fn sample(&self, phase: f32) -> f32 {
        (phase * 2.0 * PI).sin()
    }

    fn waveform_type(&self) -> WaveformType {
        WaveformType::Sine
    }
}

/// Square wave generator
pub struct SquareWave {
    pub phase: f32,
    pub duty_cycle: f32,
}

impl SquareWave {
    pub fn new() -> Self {
        Self { phase: 0.0, duty_cycle: 0.5 }
    }

    pub fn with_duty_cycle(duty: f32) -> Self {
        Self { phase: 0.0, duty_cycle: duty.clamp(0.1, 0.9) }
    }
}

impl Default for SquareWave {
    fn default() -> Self {
        Self::new()
    }
}

impl WaveformGenerator for SquareWave {
    fn sample(&self, phase: f32) -> f32 {
        if phase < self.duty_cycle {
            1.0
        } else {
            -1.0
        }
    }

    fn waveform_type(&self) -> WaveformType {
        WaveformType::Square
    }
}

/// Sawtooth wave generator
pub struct SawtoothWave {
    pub phase: f32,
}

impl SawtoothWave {
    pub fn new() -> Self {
        Self { phase: 0.0 }
    }
}

impl Default for SawtoothWave {
    fn default() -> Self {
        Self::new()
    }
}

impl WaveformGenerator for SawtoothWave {
    fn sample(&self, phase: f32) -> f32 {
        2.0 * phase - 1.0
    }

    fn waveform_type(&self) -> WaveformType {
        WaveformType::Sawtooth
    }
}

/// Triangle wave generator
pub struct TriangleWave {
    pub phase: f32,
}

impl TriangleWave {
    pub fn new() -> Self {
        Self { phase: 0.0 }
    }
}

impl Default for TriangleWave {
    fn default() -> Self {
        Self::new()
    }
}

impl WaveformGenerator for TriangleWave {
    fn sample(&self, phase: f32) -> f32 {
        if phase < 0.25 {
            4.0 * phase
        } else if phase < 0.75 {
            2.0 - 4.0 * phase
        } else {
            -4.0 + 4.0 * phase
        }
    }

    fn waveform_type(&self) -> WaveformType {
        WaveformType::Triangle
    }
}

/// Pulse wave generator (variable width)
pub struct PulseWave {
    pub phase: f32,
    pub width: f32,
}

impl PulseWave {
    pub fn new() -> Self {
        Self { phase: 0.0, width: 0.5 }
    }

    pub fn with_width(width: f32) -> Self {
        Self { phase: 0.0, width: width.clamp(0.05, 0.95) }
    }
}

impl Default for PulseWave {
    fn default() -> Self {
        Self::new()
    }
}

impl WaveformGenerator for PulseWave {
    fn sample(&self, phase: f32) -> f32 {
        if phase < self.width { 1.0 } else { -1.0 }
    }

    fn waveform_type(&self) -> WaveformType {
        WaveformType::Pulse
    }
}

/// White noise generator
pub struct WhiteNoise {
    pub seed: u32,
}

impl WhiteNoise {
    pub fn new() -> Self {
        use std::time::SystemTime;
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u32;
        Self { seed }
    }

    pub fn with_seed(seed: u32) -> Self {
        Self { seed }
    }
}

impl Default for WhiteNoise {
    fn default() -> Self {
        Self::new()
    }
}

impl WaveformGenerator for WhiteNoise {
    fn sample(&self, _phase: f32) -> f32 {
        // Simple LCG random
        let new_seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        let value = (new_seed >> 16) as f32 / 65536.0;
        2.0 * value - 1.0
    }

    fn waveform_type(&self) -> WaveformType {
        WaveformType::White
    }
}

/// Pink noise generator (approximation using Paul Kellet's algorithm)
pub struct PinkNoise {
    b: [f32; 16],
    seed: u32,
}

impl PinkNoise {
    pub fn new() -> Self {
        Self {
            b: [0.0; 16],
            seed: 12345,
        }
    }
}

impl Default for PinkNoise {
    fn default() -> Self {
        Self::new()
    }
}

impl WaveformGenerator for PinkNoise {
    fn sample(&self, _phase: f32) -> f32 {
        // Simple white noise for now
        let new_seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        let value = ((new_seed >> 16) as f32 / 65536.0) * 0.5 - 0.25;
        self.b[0] * 0.99886 + value
    }

    fn waveform_type(&self) -> WaveformType {
        WaveformType::Pink
    }
}

/// Brown noise (random walk) generator
pub struct BrownNoise {
    last_value: f32,
    seed: u32,
}

impl BrownNoise {
    pub fn new() -> Self {
        Self {
            last_value: 0.0,
            seed: 54321,
        }
    }
}

impl Default for BrownNoise {
    fn default() -> Self {
        Self::new()
    }
}

impl WaveformGenerator for BrownNoise {
    fn sample(&self, _phase: f32) -> f32 {
        let new_seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        let white = ((new_seed >> 16) as f32 / 65536.0) * 2.0 - 1.0;
        (self.last_value + (0.02 * white)).clamp(-1.0, 1.0)
    }

    fn waveform_type(&self) -> WaveformType {
        WaveformType::Brown
    }
}

/// Soft square wave (with slight slope)
pub struct SoftSquareWave {
    pub phase: f32,
    pub slope: f32,
}

impl SoftSquareWave {
    pub fn new() -> Self {
        Self { phase: 0.0, slope: 0.1 }
    }
}

impl Default for SoftSquareWave {
    fn default() -> Self {
        Self::new()
    }
}

impl WaveformGenerator for SoftSquareWave {
    fn sample(&self, phase: f32) -> f32 {
        // Sigmoid function for soft transition
        let x = (phase - 0.5) / self.slope;
        2.0 / (1.0 + (-x).exp()) - 1.0
    }

    fn waveform_type(&self) -> WaveformType {
        WaveformType::SoftSquare
    }
}

/// Soft sawtooth wave (with smooth transition)
pub struct SoftSawtoothWave {
    pub phase: f32,
    pub slope: f32,
}

impl SoftSawtoothWave {
    pub fn new() -> Self {
        Self { phase: 0.0, slope: 0.1 }
    }
}

impl Default for SoftSawtoothWave {
    fn default() -> Self {
        Self::new()
    }
}

impl WaveformGenerator for SoftSawtoothWave {
    fn sample(&self, phase: f32) -> f32 {
        // Smooth interpolation between -1 and 1
        let x = (phase - 0.5) / self.slope;
        (2.0 / (1.0 + (-x).exp()) - 1.0) * 
        (2.0 * (if phase < 0.5 { phase * 2.0 } else { 2.0 - phase * 2.0 }))
    }

    fn waveform_type(&self) -> WaveformType {
        WaveformType::SoftSawtooth
    }
}

/// Soft triangle wave
pub struct SoftTriangleWave {
    pub phase: f32,
    pub smoothness: f32,
}

impl SoftTriangleWave {
    pub fn new() -> Self {
        Self { phase: 0.0, smoothness: 0.1 }
    }
}

impl Default for SoftTriangleWave {
    fn default() -> Self {
        Self::new()
    }
}

impl WaveformGenerator for SoftTriangleWave {
    fn sample(&self, phase: f32) -> f32 {
        // Interpolated triangle
        let t = phase * 2.0;
        if t < 1.0 {
            t - (t.powi(2) / self.smoothness).tanh()
        } else {
            (2.0 - t) - ((t - 1.0).powi(2) / self.smoothness).tanh()
        }
    }

    fn waveform_type(&self) -> WaveformType {
        WaveformType::SoftTriangle
    }
}

/// Create waveform generator by type
pub fn create_waveform(waveform_type: WaveformType) -> Box<dyn WaveformGenerator> {
    match waveform_type {
        WaveformType::Sine => Box::new(SineWave::new()),
        WaveformType::Square => Box::new(SquareWave::new()),
        WaveformType::Sawtooth => Box::new(SawtoothWave::new()),
        WaveformType::Triangle => Box::new(TriangleWave::new()),
        WaveformType::Pulse => Box::new(PulseWave::new()),
        WaveformType::White | WaveformType::Noise => Box::new(WhiteNoise::new()),
        WaveformType::Pink => Box::new(PinkNoise::new()),
        WaveformType::Brown => Box::new(BrownNoise::new()),
        WaveformType::SoftSquare => Box::new(SoftSquareWave::new()),
        WaveformType::SoftSawtooth => Box::new(SoftSawtoothWave::new()),
        WaveformType::SoftTriangle => Box::new(SoftTriangleWave::new()),
    }
}

/// Generate a waveform with ADSR envelope
pub fn generate_adsr_envelope(
    waveform: &dyn WaveformGenerator,
    frequency: f32,
    sample_rate: u32,
    duration_samples: usize,
    attack: usize,
    decay: usize,
    sustain: f32,
    release: usize,
) -> Vec<f32> {
    let total_samples = attack + decay + (duration_samples - attack - decay - release) + release;
    let mut output = Vec::with_capacity(total_samples);
    
    for i in 0..total_samples {
        let phase = (i as f32 * frequency / sample_rate as f32) % 1.0;
        let sample = waveform.sample(phase);
        
        // ADSR envelope
        let envelope = if i < attack {
            i as f32 / attack as f32
        } else if i < attack + decay {
            let decay_progress = (i - attack) as f32 / decay as f32;
            1.0 - (1.0 - sustain) * decay_progress
        } else if i < total_samples - release {
            sustain
        } else {
            let release_progress = (i - (total_samples - release)) as f32 / release as f32;
            sustain * (1.0 - release_progress)
        };
        
        output.push(sample * envelope);
    }
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sine_wave() {
        let wave = SineWave::new();
        assert!((wave.sample(0.0) - 0.0).abs() < 0.001);
        assert!((wave.sample(0.25) - 1.0).abs() < 0.001);
        assert!((wave.sample(0.5) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_square_wave() {
        let wave = SquareWave::new();
        assert!((wave.sample(0.0) - 1.0).abs() < 0.001);
        assert!((wave.sample(0.75) - (-1.0)).abs() < 0.001);
    }

    #[test]
    fn test_sawtooth_wave() {
        let wave = SawtoothWave::new();
        assert!((wave.sample(0.0) - (-1.0)).abs() < 0.001);
        assert!((wave.sample(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_triangle_wave() {
        let wave = TriangleWave::new();
        assert!((wave.sample(0.0) - (-1.0)).abs() < 0.001);
        assert!((wave.sample(0.25) - 1.0).abs() < 0.001);
        assert!((wave.sample(0.5) - (-1.0)).abs() < 0.001);
    }

    #[test]
    fn test_adsr_envelope() {
        let wave = SineWave::new();
        let samples = generate_adsr_envelope(&wave, 440.0, 44100, 4410, 441, 882, 0.7, 882);
        
        // Should start at 0
        assert!(samples[0].abs() < 0.001);
        
        // Attack peak should be close to max
        assert!((samples[440].abs() - 1.0).abs() < 0.1);
        
        // Should have proper length
        assert_eq!(samples.len(), 4410);
    }

    #[test]
    fn test_create_waveform() {
        let wave = create_waveform(WaveformType::Sine);
        assert_eq!(wave.waveform_type(), WaveformType::Sine);
        
        let wave2 = create_waveform(WaveformType::Square);
        assert_eq!(wave2.waveform_type(), WaveformType::Square);
    }
}
