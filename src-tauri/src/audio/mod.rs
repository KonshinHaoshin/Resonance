//! Resonance - Core Audio Engine
//! 
//! This module provides the core audio processing capabilities.

pub mod engine;
pub mod buffer;
pub mod sample;
pub mod waveform;

pub use engine::AudioEngine;
pub use buffer::AudioBuffer;
pub use sample::Sample;
pub use waveform::*;
