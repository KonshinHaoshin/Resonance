use serde::{Deserialize, Serialize};

/// UST File - Original UTAU format
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UstFile {
    /// Project settings
    #[serde(skip)]
    pub settings: UstSettings,
    /// Notes
    pub notes: Vec<UstNote>,
}

/// UST Settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UstSettings {
    pub tempo: f64,
    pub track_name: String,
    pub voice_dir: String,
    pub output_file: String,
}

/// UST Note
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UstNote {
    /// Note number (0 = C3)
    #[serde(rename = "#NOTE")]
    pub note: Option<String>,
    /// Length in ticks
    #[serde(rename = "#LENGTH")]
    pub length: u64,
    /// Lyric
    #[serde(rename = "#LYRIC")]
    pub lyric: String,
    /// Velocity
    #[serde(rename = "#VELOCITY")]
    pub velocity: u32,
    /// Intensity
    #[serde(rename = "#INTENSITY")]
    pub intensity: u32,
    /// Modulation
    #[serde(rename = "#MODULATION")]
    pub modulation: u32,
    /// Pre-phoneme length
    #[serde(rename = "#PREPHONEME")]
    pub pre_phoneme: u32,
    /// Voice overlap
    #[serde(rename = "#VOICEOVERLAP")]
    pub voice_overlap: u32,
    /// Flags (deprecated in UTAU)
    #[serde(rename = "#FLAGS")]
    pub flags: Option<String>,
    /// Pitch bend
    #[serde(rename = "#PBS")]
    pub pitch_bend_start: Option<String>,
    /// Pitch bend duration
    #[serde(rename = "#PBW")]
    pub pitch_bend_width: Vec<u32>,
    /// Pitch bend depth
    #[serde(rename = "#PBY")]
    pub pitch_bend_depth: Vec<i32>,
    /// Vibrato
    #[serde(rename = "#VIBRATO")]
    pub vibrato: Option<String>,
}

impl Default for UstNote {
    fn default() -> Self {
        Self {
            note: None,
            length: 480,
            lyric: "a".to_string(),
            velocity: 100,
            intensity: 100,
            modulation: 0,
            pre_phoneme: 0,
            voice_overlap: 0,
            flags: None,
            pitch_bend_start: None,
            pitch_bend_width: vec![],
            pitch_bend_depth: vec![],
            vibrato: None,
        }
    }
}

impl UstFile {
    /// Create a new UST file
    pub fn new() -> Self {
        Self {
            settings: UstSettings::default(),
            notes: vec![UstNote::default()],
        }
    }

    /// Get total duration in ticks
    pub fn duration(&self) -> u64 {
        self.notes.iter().map(|n| n.length).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ust_creation() {
        let ust = UstFile::new();
        assert_eq!(ust.notes.len(), 1);
    }
}
