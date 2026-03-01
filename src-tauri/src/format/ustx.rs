use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// USTX File - OpenUtau's main file format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UstxFile {
    /// File version
    #[serde(rename = "Version")]
    pub version: String,
    /// Project name
    #[serde(rename = "Name")]
    pub name: String,
    /// BPM (beats per minute)
    #[serde(rename = "BPM")]
    pub bpm: f64,
    /// Time signature numerator
    #[serde(rename = "BeatPerBar")]
    pub beat_per_bar: u32,
    /// Time signature denominator
    #[serde(rename = "BeatUnit")]
    pub beat_unit: u32,
    /// Tempo map
    #[serde(rename = "Tempo")]
    pub tempo: Vec<Tempo>,
    /// Tracks
    #[serde(rename = "Tracks")]
    pub tracks: Vec<TrackData>,
    /// Project settings
    #[serde(rename = "Project")]
    pub project: ProjectSettings,
}

/// Tempo point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tempo {
    #[serde(rename = "Position")]
    pub position: u64,
    #[serde(rename = "BPM")]
    pub bpm: f64,
}

/// Track data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrackData {
    /// Track name
    #[serde(rename = "Name")]
    pub name: String,
    /// Track color
    #[serde(rename = "Color")]
    pub color: Option<u32>,
    /// Notes in the track
    #[serde(rename = "Notes")]
    pub notes: Vec<NoteData>,
}

/// Note data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteData {
    /// Note position in ticks
    #[serde(rename = "Position")]
    pub position: u64,
    /// Note duration in ticks
    #[serde(rename = "Duration")]
    pub duration: u64,
    /// Note pitch (MIDI number)
    #[serde(rename = "Pitch")]
    pub pitch: i32,
    /// Note velocity
    #[serde(rename = "Velocity")]
    pub velocity: u32,
    /// Vibrato settings
    #[serde(rename = "Vibrato")]
    pub vibrato: Option<VibratoData>,
}

/// Vibrato data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibratoData {
    /// Start time relative to note start
    #[serde(rename = "Start")]
    pub start: u64,
    /// Vibrato duration
    #[serde(rename = "Length")]
    pub length: u64,
    /// Vibrato period in points
    #[serde(rename = "Period")]
    pub period: u64,
    /// Vibrato depth
    #[serde(rename = "Depth")]
    pub depth: i32,
    /// Fade in length
    #[serde(rename = "Fade")]
    pub fade: u64,
}

/// Project settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    /// vocals directory path
    #[serde(rename = "VoiceDir")]
    pub voice_dir: Option<String>,
    /// Singer name
    #[serde(rename = "Singer")]
    pub singer: Option<String>,
    /// Expression presets
    #[serde(rename = "Expressions")]
    pub expressions: HashMap<String, ExpressionDef>,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            voice_dir: None,
            singer: None,
            expressions: HashMap::new(),
        }
    }
}

/// Expression definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressionDef {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Abbreviation")]
    pub abbreviation: Option<String>,
    #[serde(rename = "Type")]
    pub expr_type: u32,
    #[serde(rename = "Min")]
    pub min: f64,
    #[serde(rename = "Max")]
    pub max: f64,
    #[serde(rename = "DefaultValue")]
    pub default_value: f64,
}

impl Default for UstxFile {
    fn default() -> Self {
        Self {
            version: "OpenUtau".to_string(),
            name: "Untitled".to_string(),
            bpm: 120.0,
            beat_per_bar: 4,
            beat_unit: 4,
            tempo: vec![Tempo { position: 0, bpm: 120.0 }],
            tracks: vec![TrackData::default()],
            project: ProjectSettings::default(),
        }
    }
}

impl UstxFile {
    /// Create a new USTX file with default settings
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ustx_default() {
        let file = UstxFile::default();
        assert_eq!(file.bpm, 120.0);
        assert_eq!(file.tracks.len(), 1);
    }

    #[test]
    fn test_ustx_serialization() {
        let file = UstxFile::new("Test Project");
        let json = serde_json::to_string(&file).unwrap();
        assert!(json.contains("Test Project"));
    }
}
