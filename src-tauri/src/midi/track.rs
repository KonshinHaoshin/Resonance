use std::collections::VecDeque;
use super::note::Note;

/// MIDI Track representation
#[derive(Debug, Clone, Default)]
pub struct Track {
    /// Track name
    name: String,
    /// Notes in the track
    notes: Vec<Note>,
}

impl Track {
    /// Create a new track
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            notes: Vec::new(),
        }
    }

    /// Get track name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set track name
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// Add a note to the track
    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note);
        self.notes.sort_by_key(|n| n.start);
    }

    /// Get all notes
    pub fn notes(&self) -> &[Note] {
        &self.notes
    }

    /// Get notes in a time range
    pub fn notes_in_range(&self, start: u64, end: u64) -> Vec<&Note> {
        self.notes
            .iter()
            .filter(|n| n.start < end && n.end() > start)
            .collect()
    }

    /// Get the total duration of the track
    pub fn duration(&self) -> u64 {
        self.notes.iter().map(|n| n.end()).max().unwrap_or(0)
    }

    /// Get the number of notes
    pub fn len(&self) -> usize {
        self.notes.len()
    }

    /// Check if track is empty
    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    /// Clear all notes
    pub fn clear(&mut self) {
        self.notes.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_creation() {
        let track = Track::new("Test Track");
        assert_eq!(track.name(), "Test Track");
        assert!(track.is_empty());
    }

    #[test]
    fn test_track_add_note() {
        let mut track = Track::new("Test");
        track.add_note(Note::new(60, 100, 0, 480));
        track.add_note(Note::new(64, 80, 480, 480));
        assert_eq!(track.len(), 2);
    }

    #[test]
    fn test_track_notes_in_range() {
        let mut track = Track::new("Test");
        track.add_note(Note::new(60, 100, 0, 480));
        track.add_note(Note::new(64, 80, 960, 480));
        
        let notes = track.notes_in_range(0, 500);
        assert_eq!(notes.len(), 1);
    }
}
