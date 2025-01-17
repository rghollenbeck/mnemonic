use mnemonic::*;
use std::convert::TryInto;

fn load_test_wordlist() -> WordList {
    WordList::load("data/bip39list.json").expect("Failed to load the wordlist")
}

#[test]
fn test_word_to_binary() {
    let word_list = load_test_wordlist();
    assert_eq!(word_to_binary(&word_list, "zoo"), Some("11111111111"));
}

#[test]
fn test_word_to_decimal() {
    let word_list = load_test_wordlist();
    assert_eq!(word_to_decimal(&word_list, "zoo"), Some(2047));
}

#[test]
fn test_binary_to_word() {
    let word_list = load_test_wordlist();
    assert_eq!(binary_to_word(&word_list, "00000000000"), Some("abandon"));
}

#[test]
fn test_decimal_to_word() {
    let word_list = load_test_wordlist();
    assert_eq!(decimal_to_word(&word_list, 0), Some("abandon"));
}

#[test]
fn test_outlier_number_one() {
    let word_list = load_test_wordlist();
    assert_eq!(binary_to_word(&word_list, "101010101010"), None); // Invalid binary
}

#[test]
fn test_outlier_number_two() {
    let word_list = load_test_wordlist();
    assert_eq!(decimal_to_word(&word_list, 2048), None); // Out of range
}

#[test]
fn test_outlier_number_three() {
    let word_list = load_test_wordlist();
    assert_eq!(decimal_to_word(&word_list, u32::MAX.try_into().unwrap_or(usize::MAX)), None); // Extreme value
}

#[test]
fn test_invalid_binary_string() {
    let word_list = load_test_wordlist();
    assert_eq!(binary_to_word(&word_list, "invalid_binary"), None);
}

#[test]
fn test_invalid_word() {
    let word_list = load_test_wordlist();
    assert_eq!(word_to_binary(&word_list, "notaword"), None);
}

pub fn is_valid_word(word_list: &WordList, word: &str) -> bool {
    word_list.wordlist.iter().any(|entry| entry.word == word)
}

