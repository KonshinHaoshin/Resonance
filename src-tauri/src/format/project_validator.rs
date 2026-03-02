//! Project Validator - Comprehensive USTX file validation

use crate::format::ustx::{UstxFile, TrackData, NoteData, Tempo, VibratoData};

/// Validation result
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, message: impl Into<String>) {
        self.errors.push(message.into());
        self.is_valid = false;
    }

    pub fn add_warning(&mut self, message: impl Into<String>) {
        self.warnings.push(message.into());
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid && self.errors.is_empty()
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Project validator
pub struct ProjectValidator;

impl ProjectValidator {
    /// Create a new validator
    pub fn new() -> Self {
        Self
    }

    /// Validate a complete USTX project
    pub fn validate(&self, project: &UstxFile) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate basic project properties
        self.validate_basic_info(project, &mut result);
        
        // Validate tempo
        self.validate_tempo(project, &mut result);
        
        // Validate time signature
        self.validate_time_signature(project, &mut result);
        
        // Validate tracks
        self.validate_tracks(project, &mut result);
        
        // Validate project integrity
        self.validate_integrity(project, &mut result);
        
        result
    }

    /// Validate basic project information
    fn validate_basic_info(&self, project: &UstxFile, result: &mut ValidationResult) {
        if project.name.is_empty() {
            result.add_error("Project name is empty");
        }

        if project.name.len() > 256 {
            result.add_warning(format!("Project name is very long ({} chars)", project.name.len()));
        }

        if project.version.is_empty() {
            result.add_warning("Project version is not specified");
        }
    }

    /// Validate tempo settings
    fn validate_tempo(&self, project: &UstxFile, result: &mut ValidationResult) {
        if project.bpm <= 0.0 || project.bpm > 1000.0 {
            result.add_error(format!("Invalid BPM: {} (must be between 0.01 and 1000)", project.bpm));
        }

        if project.bpm < 20.0 || project.bpm > 400.0 {
            result.add_warning(format!("Unusual BPM: {} (typical range 40-240)", project.bpm));
        }

        if project.tempo.is_empty() {
            result.add_error("No tempo information in project");
            return;
        }

        // Check tempo consistency
        for (i, tempo) in project.tempo.iter().enumerate() {
            if tempo.bpm <= 0.0 || tempo.bpm > 1000.0 {
                result.add_error(format!("Tempo at index {} has invalid BPM: {}", i, tempo.bpm));
            }
        }
        
        // Check if first tempo matches project BPM
        if let Some(first_tempo) = project.tempo.first() {
            if (first_tempo.bpm - project.bpm).abs() > 0.001 {
                result.add_warning(format!("First tempo BPM ({}) doesn't match project BPM ({})",
                    first_tempo.bpm, project.bpm));
            }
        }
    }

    /// Validate time signature
    fn validate_time_signature(&self, project: &UstxFile, result: &mut ValidationResult) {
        if project.beat_per_bar == 0 {
            result.add_error("Beat per bar cannot be zero");
        }

        if project.beat_per_bar > 32 {
            result.add_warning(format!("Unusually high beat per bar: {}", project.beat_per_bar));
        }

        // Common time signatures use powers of 2
        let valid_beats = vec![1, 2, 4, 8, 16, 32];
        if !valid_beats.contains(&project.beat_unit) {
            result.add_warning(format!("Unusual beat unit: {} (expected 1, 2, 4, 8, 16, or 32)",
                project.beat_unit));
        }

        // Check for reasonable time signatures
        if project.beat_per_bar > 12 {
            result.add_warning(format!("Unusual time signature: {}/{}",
                project.beat_per_bar, project.beat_unit));
        }
    }

    /// Validate tracks
    fn validate_tracks(&self, project: &UstxFile, result: &mut ValidationResult) {
        if project.tracks.is_empty() {
            result.add_error("Project has no tracks");
            return;
        }

        if project.tracks.len() > 100 {
            result.add_warning(format!("Excessive number of tracks: {}", project.tracks.len()));
        }

        for (track_idx, track) in project.tracks.iter().enumerate() {
            self.validate_track(track, track_idx, project, result);
        }
    }

    /// Validate a single track
    fn validate_track(&self, track: &TrackData, track_idx: usize, project: &UstxFile, result: &mut ValidationResult) {
        if track.name.is_empty() {
            result.add_error(format!("Track {} has no name", track_idx));
        }

        if track.notes.is_empty() {
            result.add_warning(format!("Track '{}' has no notes", track.name));
        }

        // Check for overlapping notes
        self.check_note_overlaps(&track.notes, track_idx, &track.name, result);
        
        // Validate individual notes
        for (note_idx, note) in track.notes.iter().enumerate() {
            self.validate_note(note, track_idx, note_idx, result);
        }
    }

    /// Validate a single note
    fn validate_note(&self, note: &NoteData, track_idx: usize, note_idx: usize, result: &mut ValidationResult) {
        if note.pitch < 0 || note.pitch > 127 {
            result.add_error(format!(
                "Track {} note {} has invalid pitch: {} (must be 0-127)",
                track_idx, note_idx, note.pitch));
        }

        if note.duration == 0 {
            result.add_error(format!(
                "Track {} note {} has zero duration",
                track_idx, note_idx));
        }

        if note.velocity == 0 {
            result.add_warning(format!(
                "Track {} note {} has zero velocity",
                track_idx, note_idx));
        }

        if note.velocity > 127 {
            result.add_error(format!(
                "Track {} note {} has invalid velocity: {} (must be 0-127)",
                track_idx, note_idx, note.velocity));
        }

        // Validate vibrato if present
        if let Some(vibrato) = &note.vibrato {
            self.validate_vibrato(vibrato, track_idx, note_idx, result);
        }
    }

    /// Validate vibrato data
    fn validate_vibrato(&self, vibrato: &VibratoData, track_idx: usize, note_idx: usize, result: &mut ValidationResult) {
        if vibrato.length == 0 {
            result.add_warning(format!(
                "Track {} note {} has vibrato with zero length",
                track_idx, note_idx));
        }

        if vibrato.period == 0 {
            result.add_error(format!(
                "Track {} note {} has vibrato with zero period",
                track_idx, note_idx));
        }
    }

    /// Check for overlapping notes
    fn check_note_overlaps(&self, notes: &[NoteData], track_idx: usize, track_name: &str, result: &mut ValidationResult) {
        for i in 0..notes.len() {
            for j in (i + 1)..notes.len() {
                let note1 = &notes[i];
                let note2 = &notes[j];

                // Check if same pitch and overlapping
                if note1.pitch == note2.pitch {
