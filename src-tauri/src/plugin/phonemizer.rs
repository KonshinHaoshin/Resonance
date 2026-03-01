use std::collections::HashMap;

/// Phonemizer trait - converts lyrics to phonemes
pub trait Phonemizer {
    /// Get the phonemizer name
    fn name(&self) -> &str;

    /// Get the phonemizer language code
    fn language(&self) -> &str;

    /// Convert lyrics to phonemes
    fn phonemize(&self, lyrics: &str) -> Vec<Phoneme>;

    /// Get phoneme name for a given alias
    fn get_alias_phoneme(&self, alias: &str) -> Option<Phoneme> {
        None
    }
}

/// Phoneme representation
#[derive(Debug, Clone, PartialEq)]
pub struct Phoneme {
    /// Phoneme symbol
    pub symbol: String,
    /// Phoneme start position (in ticks)
    pub position: u64,
    /// Phoneme duration (in ticks)
    pub duration: u64,
}

impl Phoneme {
    /// Create a new phoneme
    pub fn new(symbol: impl Into<String>, position: u64, duration: u64) -> Self {
        Self {
            symbol: symbol.into(),
            position,
            duration,
        }
    }
}

/// Built-in phonemizer implementations
pub mod builtin {
    use super::*;

    /// Japanese CV/VCV phonemizer
    pub struct JapanesePhonemizer;

    impl Phonemizer for JapanesePhonemizer {
        fn name(&self) -> &str {
            "Japanese CV/VCV"
        }

        fn language(&self) -> &str {
            "ja"
        }

        fn phonemize(&self, lyrics: &str) -> Vec<Phoneme> {
            // Simplified: just return each character as a phoneme
            lyrics
                .chars()
                .enumerate()
                .map(|(i, c)| Phoneme::new(c.to_string(), i as u64 * 100, 100))
                .collect()
        }
    }

    /// English ARPABET phonemizer
    pub struct EnglishArpabetPhonemizer;

    impl Phonemizer for EnglishArpabetPhonemizer {
        fn name(&self) -> &str {
            "English ARPA"
        }

        fn language(&self) -> &str {
            "en"
        }

        fn phonemize(&self, lyrics: &str) -> Vec<Phoneme> {
            // Simplified: return each word as a phoneme
            lyrics
                .split_whitespace()
                .enumerate()
                .map(|(i, w)| Phoneme::new(w.to_string(), i as u64 * 200, 200))
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_japanese_phonemizer() {
        let phonemizer = builtin::JapanesePhonemizer;
        let result = phonemizer.phonemize("あいうえお");
        assert_eq!(result.len(), 5);
    }
}
