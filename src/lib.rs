use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader};

/// Represents an entry in the BIP39 wordlist
#[derive(Serialize, Deserialize, Debug)]
pub struct WordEntry {
    pub binary: String,
    pub word: String,
}

/// Represents the full BIP39 wordlist
#[derive(Serialize, Deserialize, Debug)]
pub struct WordList {
    pub wordlist: Vec<WordEntry>,
}

impl WordList {
    /// Load the wordlist from a JSON file
    pub fn load(path: &str) -> Result<Self, io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let wordlist = serde_json::from_reader(reader)?;
        Ok(wordlist)
    }
}

/// Convert a word to its binary value
pub fn word_to_binary<'a>(word_list: &'a WordList, word: &str) -> Option<&'a str> {
    word_list
        .wordlist
        .iter()
        .find(|entry| entry.word == word)
        .map(|entry| entry.binary.as_str())
}

/// Convert a word to its decimal index
pub fn word_to_decimal(word_list: &WordList, word: &str) -> Option<usize> {
    word_list
        .wordlist
        .iter()
        .position(|entry| entry.word == word)
}

/// Convert a binary value to its word
pub fn binary_to_word<'a>(word_list: &'a WordList, binary: &str) -> Option<&'a str> {
    word_list
        .wordlist
        .iter()
        .find(|entry| entry.binary == binary)
        .map(|entry| entry.word.as_str())
}

/// Convert a decimal index to its word
pub fn decimal_to_word<'a>(word_list: &'a WordList, decimal: usize) -> Option<&'a str> {
    word_list.wordlist.get(decimal).map(|entry| entry.word.as_str())
}

