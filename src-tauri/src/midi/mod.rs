//! Resonance - MIDI Processing
//! 
//! This module provides MIDI parsing and processing capabilities.

pub mod note;
pub mod track;
pub mod event;

pub use note::Note;
pub use track::Track;
pub use event::{MidiEvent, MidiEventType};
