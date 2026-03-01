/// Audio sample representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sample {
    /// Sample value (-1.0 to 1.0)
    pub value: f32,
}

impl Sample {
    /// Create a new sample
    pub fn new(value: f32) -> Self {
        Self { value: value.clamp(-1.0, 1.0) }
    }

    /// Create a silent sample
    pub fn silence() -> Self {
        Self { value: 0.0 }
    }

    /// Get the amplitude (absolute value)
    pub fn amplitude(&self) -> f32 {
        self.value.abs()
    }

    /// Apply gain to the sample
    pub fn with_gain(self, gain: f32) -> Self {
        Self {
            value: (self.value * gain).clamp(-1.0, 1.0),
        }
    }

    /// Mix two samples together
    pub fn mix(self, other: Self) -> Self {
        Self {
            value: (self.value + other.value).clamp(-1.0, 1.0),
        }
    }
}

impl Default for Sample {
    fn default() -> Self {
        Self::silence()
    }
}

impl From<f32> for Sample {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl From<Sample> for f32 {
    fn from(sample: Sample) -> Self {
        sample.value
    }
}
