use serde::{Serialize, Deserialize};

/// MIDI Note representation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Note {
    /// MIDI note number (0-127)
    pub pitch: u8,
    /// Velocity (0-127)
    pub velocity: u8,
    /// Start position in ticks
    pub start: u64,
    /// Duration in ticks
    pub duration: u64,
}

impl Note {
    /// Create a new note
    pub fn new(pitch: u8, velocity: u8, start: u64, duration: u64) -> Self {
        Self {
            pitch: pitch.min(127),
            velocity: velocity.min(127),
            start,
            duration,
        }
    }

    /// Get note name (e.g., "C4", "F#3")
    pub fn name(&self) -> String {
        let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        let octave = (self.pitch / 12) as i32 - 1;
        let note = note_names[(self.pitch % 12) as usize];
        format!("{}{}", note, octave)
    }

    /// Get frequency in Hz
    pub fn frequency(&self) -> f64 {
        440.0 * 2.0_f64.powf((self.pitch as f64 - 69.0) / 12.0)
    }

    /// Get end position in ticks
    pub fn end(&self) -> u64 {
        self.start + self.duration
    }

    /// Check if this note overlaps with another
    pub fn overlaps(&self, other: &Note) -> bool {
        self.start < other.end() && other.start < self.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_creation() {
        let note = Note::new(60, 100, 0, 480);
        assert_eq!(note.pitch, 60);
        assert_eq!(note.velocity, 100);
        assert_eq!(note.start, 0);
        assert_eq!(note.duration, 480);
    }

    #[test]
    fn test_note_name() {
        let note = Note::new(60, 100, 0, 480); // C4
        assert_eq!(note.name(), "C4");
    }

    #[test]
    fn test_note_frequency() {
        let note = Note::new(69, 100, 0, 480); // A4
        assert!((note.frequency() - 440.0).abs() < 0.01);
    }
}
