use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader};

/// Represents a single word entry in the BIP39 wordlist.
#[derive(Serialize, Deserialize, Debug)]
pub struct WordEntry {
    pub binary: String,
    pub word: String,
}

/// Represents a BIP39 wordlist loaded from a JSON file.
#[derive(Serialize, Deserialize, Debug)]
pub struct WordList {
    pub wordlist: Vec<WordEntry>,
}

impl WordList {
    /// Loads a BIP39 wordlist from a JSON file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the JSON file containing the wordlist.
    ///
    /// # Returns
    ///
    /// A `WordList` instance on success, or an error if the file could not be loaded or parsed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mnemonic::WordList;
    ///
    /// let word_list = WordList::load("data/bip39list.json").expect("Failed to load the wordlist");
    /// ```
    pub fn load(path: &str) -> Result<Self, io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let wordlist = serde_json::from_reader(reader)?;
        Ok(wordlist)
    }
}

/// Converts a BIP39 word to its binary representation.
///
/// # Arguments
///
/// * `word_list` - A reference to the loaded `WordList`.
/// * `word` - The word to be converted.
///
/// # Returns
///
/// The binary representation of the word as a string, or `None` if the word is not found.
///
/// # Examples
///
/// ```rust
/// let binary = word_to_binary(&word_list, "zoo"); // Some("11111111111")
/// ```
pub fn word_to_binary<'a>(word_list: &'a WordList, word: &str) -> Option<&'a str> {
    word_list
        .wordlist
        .iter()
        .find(|entry| entry.word == word)
        .map(|entry| entry.binary.as_str())
}

/// Converts a BIP39 word to its decimal index.
///
/// # Arguments
///
/// * `word_list` - A reference to the loaded `WordList`.
/// * `word` - The word to be converted.
///
/// # Returns
///
/// The decimal index of the word as a string, or `None` if the word is not found.
///
/// # Examples
///
/// ```rust
/// let binary = word_to_decimal(&word_list, "zoo"); // Some("2047")
/// ```

pub fn word_to_decimal(word_list: &WordList, word: &str) -> Option<usize> {
    word_list
        .wordlist
        .iter()
        .position(|entry| entry.word == word)
}

/// Converts a binary value to its corresponding BIP39 word.
///
/// # Arguments
///
/// * `word_list` - A reference to the loaded `WordList`.
/// * `binary` - The binary value to convert.
///
/// # Returns
///
/// The corresponding word, or `None` if the binary value is invalid.
///
/// # Examples
///
/// ```rust
/// let word = binary_to_word(&word_list, "00000000000"); // Some("abandon")
/// ```
pub fn binary_to_word<'a>(word_list: &'a WordList, binary: &str) -> Option<&'a str> {
    word_list
        .wordlist
        .iter()
        .find(|entry| entry.binary == binary)
        .map(|entry| entry.word.as_str())
}

/// Converts a decimal index to its corresponding BIP39 word.
///
/// # Arguments
///
/// * `word_list` - A reference to the loaded `WordList`.
/// * `decimal` - The decimal index to convert.
///
/// # Returns
///
/// The corresponding word, or `None` if the index is out of range.
///
/// # Examples
///
/// ```rust
/// let word = decimal_to_word(&word_list, 0); // Some("abandon")
/// ```
pub fn decimal_to_word<'a>(word_list: &'a WordList, decimal: usize) -> Option<&'a str> {
    word_list.wordlist.get(decimal).map(|entry| entry.word.as_str())
}

