use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};

/// Phonemizer trait - converts lyrics to phonemes
pub trait Phonemizer: Send + Sync {
    /// Get the phonemizer name
    fn name(&self) -> &str;

    /// Get the phonemizer language code
    fn language(&self) -> &str;

    /// Get the phonemizer description
    fn description(&self) -> &str {
        ""
    }

    /// Convert lyrics to phonemes
    fn phonemize(&self, lyrics: &str) -> Vec<Phoneme>;

    /// Convert lyrics to phonemes with timing
    fn phonemize_with_timing(&self, lyrics: &str, start: u64, duration: u64) -> Vec<Phoneme> {
        let phonemes = self.phonemize(lyrics);
        if phonemes.is_empty() {
            return vec![];
        }

        let phoneme_duration = duration / phonemes.len() as u64;
        phonemes
            .into_iter()
            .enumerate()
            .map(|(i, mut p)| {
                p.position = start + (i as u64 * phoneme_duration);
                p.duration = phoneme_duration;
                p
            })
            .collect()
    }

    /// Get phoneme name for a given alias
    fn get_alias_phoneme(&self, _alias: &str) -> Option<Phoneme> {
        None
    }

    /// Get list of supported phonemes
    fn supported_phonemes(&self) -> Vec<&str> {
        vec![]
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
    /// Whether this is a vowel
    pub is_vowel: bool,
}

impl Phoneme {
    /// Create a new phoneme
    pub fn new(symbol: impl Into<String>, position: u64, duration: u64) -> Self {
        let symbol = symbol.into();
        let is_vowel = Self::is_vowel(&symbol);
        Self {
            symbol,
            position,
            duration,
            is_vowel,
        }
    }

    /// Check if phoneme is a vowel
    fn is_vowel(symbol: &str) -> bool {
        matches!(
            symbol.to_lowercase().as_str(),
            "a" | "i" | "u" | "e" | "o" | "aa" | "ii" | "uu" | "ee" | "oo"
                | "ax" | "ah" | "oh" | "ao" | "aw" | "ay" | "ey" | "oy" | "uw" | "ih"
                | "iy" | "eh" | "ae" | "uh"
        )
    }
}

/// Phonemizer manager - manages multiple phonemizers
pub struct PhonemizerManager {
    phonemizers: HashMap<String, Box<dyn Phonemizer>>,
    default_phonemizer: Option<String>,
}

impl PhonemizerManager {
    pub fn new() -> Self {
        Self {
            phonemizers: HashMap::new(),
            default_phonemizer: None,
        }
    }

    /// Register a phonemizer
    pub fn register(&mut self, name: String, phonemizer: Box<dyn Phonemizer>) {
        info!("Registering phonemizer: {}", name);
        self.phonemizers.insert(name.clone(), phonemizer);

        if self.default_phonemizer.is_none() {
            self.default_phonemizer = Some(name);
        }
    }

    /// Get phonemizer by name
    pub fn get(&self, name: &str) -> Option<&dyn Phonemizer> {
        self.phonemizers
            .get(name)
            .map(|p| p.as_ref() as &dyn Phonemizer)
    }

    /// Get default phonemizer
    pub fn get_default(&self) -> Option<&dyn Phonemizer> {
        self.default_phonemizer
            .as_ref()
            .and_then(|name| self.get(name))
    }

    /// List all phonemizers
    pub fn list(&self) -> Vec<&str> {
        self.phonemizers.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for PhonemizerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Built-in phonemizer implementations
pub mod builtin {
    use super::*;

    /// Japanese CV/VCV phonemizer with basic dictionary
    pub struct JapanesePhonemizer {
        dictionary: HashMap<String, Vec<String>>,
    }

    impl JapanesePhonemizer {
        pub fn new() -> Self {
            let mut dictionary = HashMap::new();

            // Basic Japanese phoneme dictionary
            dictionary.insert("あ".to_string(), vec!["a".to_string()]);
            dictionary.insert("い".to_string(), vec!["i".to_string()]);
            dictionary.insert("う".to_string(), vec!["u".to_string()]);
            dictionary.insert("え".to_string(), vec!["e".to_string()]);
            dictionary.insert("お".to_string(), vec!["o".to_string()]);

            dictionary.insert("か".to_string(), vec!["k".to_string(), "a".to_string()]);
            dictionary.insert("き".to_string(), vec!["k".to_string(), "i".to_string()]);
            dictionary.insert("く".to_string(), vec!["k".to_string(), "u".to_string()]);
            dictionary.insert("け".to_string(), vec!["k".to_string(), "e".to_string()]);
            dictionary.insert("こ".to_string(), vec!["k".to_string(), "o".to_string()]);

            dictionary.insert("さ".to_string(), vec!["s".to_string(), "a".to_string()]);
            dictionary.insert("し".to_string(), vec!["sh".to_string(), "i".to_string()]);
            dictionary.insert("す".to_string(), vec!["s".to_string(), "u".to_string()]);
            dictionary.insert("せ".to_string(), vec!["s".to_string(), "e".to_string()]);
            dictionary.insert("そ".to_string(), vec!["s".to_string(), "o".to_string()]);

            dictionary.insert("た".to_string(), vec!["t".to_string(), "a".to_string()]);
            dictionary.insert("ち".to_string(), vec!["ch".to_string(), "i".to_string()]);
            dictionary.insert("つ".to_string(), vec!["ts".to_string(), "u".to_string()]);
            dictionary.insert("て".to_string(), vec!["t".to_string(), "e".to_string()]);
            dictionary.insert("と".to_string(), vec!["t".to_string(), "o".to_string()]);

            dictionary.insert("な".to_string(), vec!["n".to_string(), "a".to_string()]);
            dictionary.insert("に".to_string(), vec!["n".to_string(), "i".to_string()]);
            dictionary.insert("ぬ".to_string(), vec!["n".to_string(), "u".to_string()]);
            dictionary.insert("ね".to_string(), vec!["n".to_string(), "e".to_string()]);
            dictionary.insert("の".to_string(), vec!["n".to_string(), "o".to_string()]);

            dictionary.insert("は".to_string(), vec!["h".to_string(), "a".to_string()]);
            dictionary.insert("ひ".to_string(), vec!["h".to_string(), "i".to_string()]);
            dictionary.insert("ふ".to_string(), vec!["f".to_string(), "u".to_string()]);
            dictionary.insert("へ".to_string(), vec!["h".to_string(), "e".to_string()]);
            dictionary.insert("ほ".to_string(), vec!["h".to_string(), "o".to_string()]);

            dictionary.insert("ま".to_string(), vec!["m".to_string(), "a".to_string()]);
            dictionary.insert("み".to_string(), vec!["m".to_string(), "i".to_string()]);
            dictionary.insert("む".to_string(), vec!["m".to_string(), "u".to_string()]);
            dictionary.insert("め".to_string(), vec!["m".to_string(), "e".to_string()]);
            dictionary.insert("も".to_string(), vec!["m".to_string(), "o".to_string()]);

            dictionary.insert("や".to_string(), vec!["y".to_string(), "a".to_string()]);
            dictionary.insert("ゆ".to_string(), vec!["y".to_string(), "u".to_string()]);
            dictionary.insert("よ".to_string(), vec!["y".to_string(), "o".to_string()]);

            dictionary.insert("ら".to_string(), vec!["r".to_string(), "a".to_string()]);
            dictionary.insert("り".to_string(), vec!["r".to_string(), "i".to_string()]);
            dictionary.insert("る".to_string(), vec!["r".to_string(), "u".to_string()]);
            dictionary.insert("れ".to_string(), vec!["r".to_string(), "e".to_string()]);
            dictionary.insert("ろ".to_string(), vec!["r".to_string(), "o".to_string()]);

            dictionary.insert("わ".to_string(), vec!["w".to_string(), "a".to_string()]);
            dictionary.insert("を".to_string(), vec!["w".to_string(), "o".to_string()]);

            dictionary.insert("ん".to_string(), vec!["N".to_string()]);

            // Voiced consonants
            dictionary.insert("が".to_string(), vec!["g".to_string(), "a".to_string()]);
            dictionary.insert("ぎ".to_string(), vec!["g".to_string(), "i".to_string()]);
            dictionary.insert("ぐ".to_string(), vec!["g".to_string(), "u".to_string()]);
            dictionary.insert("げ".to_string(), vec!["g".to_string(), "e".to_string()]);
            dictionary.insert("ご".to_string(), vec!["g".to_string(), "o".to_string()]);

            dictionary.insert("ざ".to_string(), vec!["z".to_string(), "a".to_string()]);
            dictionary.insert("じ".to_string(), vec!["j".to_string(), "i".to_string()]);
            dictionary.insert("ず".to_string(), vec!["z".to_string(), "u".to_string()]);
            dictionary.insert("ぜ".to_string(), vec!["z".to_string(), "e".to_string()]);
            dictionary.insert("ぞ".to_string(), vec!["z".to_string(), "o".to_string()]);

            dictionary.insert("だ".to_string(), vec!["d".to_string(), "a".to_string()]);
            dictionary.insert("ぢ".to_string(), vec!["j".to_string(), "i".to_string()]);
            dictionary.insert("づ".to_string(), vec!["z".to_string(), "u".to_string()]);
            dictionary.insert("で".to_string(), vec!["d".to_string(), "e".to_string()]);
            dictionary.insert("ど".to_string(), vec!["d".to_string(), "o".to_string()]);

            dictionary.insert("ば".to_string(), vec!["b".to_string(), "a".to_string()]);
            dictionary.insert("び".to_string(), vec!["b".to_string(), "i".to_string()]);
            dictionary.insert("ぶ".to_string(), vec!["b".to_string(), "u".to_string()]);
            dictionary.insert("べ".to_string(), vec!["b".to_string(), "e".to_string()]);
            dictionary.insert("ぼ".to_string(), vec!["b".to_string(), "o".to_string()]);

            dictionary.insert("ぱ".to_string(), vec!["p".to_string(), "a".to_string()]);
            dictionary.insert("ぴ".to_string(), vec!["p".to_string(), "i".to_string()]);
            dictionary.insert("ぷ".to_string(), vec!["p".to_string(), "u".to_string()]);
            dictionary.insert("ぺ".to_string(), vec!["p".to_string(), "e".to_string()]);
            dictionary.insert("ぽ".to_string(), vec!["p".to_string(), "o".to_string()]);

            Self { dictionary }
        }
    }

    impl Phonemizer for JapanesePhonemizer {
        fn name(&self) -> &str {
            "Japanese CV/VCV"
        }

        fn language(&self) -> &str {
            "ja"
        }

        fn description(&self) -> &str {
            "Japanese phonemizer with basic dictionary"
        }

        fn phonemize(&self, lyrics: &str) -> Vec<Phoneme> {
            let mut result = Vec::new();
            let mut position = 0u64;

            for char in lyrics.chars() {
                let char_str = char.to_string();

                if let Some(phonemes) = self.dictionary.get(&char_str) {
                    for p in phonemes {
                        result.push(Phoneme::new(p.clone(), position, 100));
                        position += 100;
                    }
                } else if char.is_whitespace() {
                    // Skip whitespace
                } else {
                    // Unknown character - just add it
                    result.push(Phoneme::new(char_str, position, 100));
                    position += 100;
                }
            }

            result
        }

        fn get_alias_phoneme(&self, alias: &str) -> Option<Phoneme> {
            // Convert alias to Japanese and look up
            self.dictionary
                .get(alias)
                .and_then(|v| v.first())
                .map(|p| Phoneme::new(p.clone(), 0, 100))
        }

        fn supported_phonemes(&self) -> Vec<&str> {
            vec![
                "a", "i", "u", "e", "o", "N",
                "k", "g", "s", "z", "sh", "j",
                "t", "d", "ts", "ch",
                "n", "h", "b", "p", "m",
                "r", "y", "w", "f",
            ]
        }
    }

    /// English ARPABET phonemizer with basic dictionary
    pub struct EnglishArpabetPhonemizer {
        dictionary: HashMap<String, Vec<String>>,
    }

    impl EnglishArpabetPhonemizer {
        pub fn new() -> Self {
            let mut dictionary = HashMap::new();

            // Basic English phoneme dictionary (simplified)
            let words = vec![
                ("a", vec!["AE".to_string()]),
                ("i", vec!["IH".to_string()]),
                ("u", vec!["UW".to_string()]),
                ("e", vec!["EH".to_string()]),
                ("o", vec!["OW".to_string()]),
                ("the", vec!["DH".to_string(), "AH".to_string()]),
                ("and", vec!["AE".to_string(), "N".to_string(), "D".to_string()]),
                ("is", vec!["IH".to_string(), "Z".to_string()]),
                ("to", vec!["T".to_string(), "UW".to_string()]),
                ("in", vec!["IH".to_string(), "N".to_string()]),
                ("it", vec!["IH".to_string(), "T".to_string()]),
                ("you", vec!["Y".to_string(), "UW".to_string()]),
                ("that", vec!["DH".to_string(), "AE".to_string(), "T".to_string()]),
                ("he", vec!["HH".to_string(), "IY".to_string()]),
                ("was", vec!["W".to_string(), "AA".to_string(), "Z".to_string()]),
                ("for", vec!["F".to_string(), "AO".to_string(), "R".to_string()]),
                ("on", vec!["AA".to_string(), "N".to_string()]),
                ("are", vec!["AA".to_string(), "R".to_string()]),
                ("with", vec!["W".to_string(), "IH".to_string(), "DH".to_string()]),
                ("as", vec!["AE".to_string(), "Z".to_string()]),
            ];

            for (word, phonemes) in words {
                dictionary.insert(word.to_string(), phonemes);
            }

            Self { dictionary }
        }
    }

    impl Phonemizer for EnglishArpabetPhonemizer {
        fn name(&self) -> &str {
            "English ARPA"
        }

        fn language(&self) -> &str {
            "en"
        }

        fn description(&self) -> &str {
            "English phonemizer with ARPABET"
        }

        fn phonemize(&self, lyrics: &str) -> Vec<Phoneme> {
            let mut result = Vec::new();
            let mut position = 0u64;

            for word in lyrics.split_whitespace() {
                let word_lower = word.to_lowercase();

                if let Some(phonemes) = self.dictionary.get(&word_lower) {
                    for p in phonemes {
                        result.push(Phoneme::new(p.clone(), position, 100));
                        position += 100;
                    }
                } else {
                    // Unknown word - just add each letter
                    for c in word.chars() {
                        result.push(Phoneme::new(c.to_string(), position, 50));
                        position += 50;
                    }
                }

                // Add a small gap between words
                position += 50;
            }

            result
        }

        fn supported_phonemes(&self) -> Vec<&str> {
            vec![
                "AA", "AE", "AH", "AO", "AW", "AY",
                "B", "CH", "D", "DH",
                "EH", "ER", "EY",
                "F", "G",
                "HH",
                "IH", "IY",
                "JH",
                "K", "L", "M",
                "N", "NG",
                "OW", "OY",
                "P", "R",
                "S", "SH",
                "T", "TH",
                "UH", "UW",
                "V",
                "W",
                "Y", "Z", "ZH",
            ]
        }
    }

    /// Simple pass-through phonemizer (for testing)
    pub struct SimplePhonemizer;

    impl Phonemizer for SimplePhonemizer {
        fn name(&self) -> &str {
            "Simple"
        }

        fn language(&self) -> &str {
            "xx"
        }

        fn description(&self) -> &str {
            "Simple pass-through phonemizer"
        }

        fn phonemize(&self, lyrics: &str) -> Vec<Phoneme> {
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
        let phonemizer = builtin::JapanesePhonemizer::new();
        let result = phonemizer.phonemize("あいうえお");
        assert!(!result.is_empty());
        println!("Japanese phonemes: {:?}", result);
    }

    #[test]
    fn test_english_phonemizer() {
        let phonemizer = builtin::EnglishArpabetPhonemizer::new();
        let result = phonemizer.phonemize("hello world");
        assert!(!result.is_empty());
        println!("English phonemes: {:?}", result);
    }

    #[test]
    fn test_phonemizer_manager() {
        let mut manager = PhonemizerManager::new();
        manager.register("ja".to_string(), Box::new(builtin::JapanesePhonemizer::new()));

        assert!(manager.get("ja").is_some());
        assert!(manager.get_default().is_some());
    }
}
