# `mnemonic`

`mnemonic` is a Rust library designed for working with BIP39 wordlists. It provides convenient functions for converting between words, binary values, and decimal indices, ensuring seamless integration with Bitcoin-related projects.

---

## Features

- **Word to Binary Conversion**: Quickly retrieve the binary representation of a given BIP39 word.
- **Word to Decimal Conversion**: Fetch the index (decimal) value of a BIP39 word.
- **Binary to Word Conversion**: Convert binary values to their corresponding BIP39 word.
- **Decimal to Word Conversion**: Retrieve a BIP39 word using its index.
- **JSON Wordlist Integration**: Load a BIP39 wordlist from a JSON file.
- **Validation**: Check for invalid or out-of-range values.

---

## Installation

Once published to [crates.io](https://crates.io/), add this to your `Cargo.toml`:

```toml
[dependencies]
mnemonic = "0.1.0"
```

---

## Usage

### Load a Wordlist

```rust
use mnemonic::WordList;

let word_list = WordList::load("data/bip39list.json").expect("Failed to load the wordlist");
```

### Perform Conversions

```rust
use mnemonic::{word_to_binary, word_to_decimal, binary_to_word, decimal_to_word};

// Convert word to binary
let binary = word_to_binary(&word_list, "zoo"); // Some("11111111111")

// Convert word to decimal index
let decimal = word_to_decimal(&word_list, "zoo"); // Some(2047)

// Convert binary to word
let word = binary_to_word(&word_list, "00000000000"); // Some("abandon")

// Convert decimal index to word
let word = decimal_to_word(&word_list, 0); // Some("abandon")
```

---

## Wordlist Format

The BIP39 wordlist is expected to be in a JSON format like this:

```json
{
  "wordlist": [
    {"binary": "00000000000", "word": "abandon"},
    {"binary": "00000000001", "word": "ability"},
    {"binary": "00000000010", "word": "able"}
  ]
}
```

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Contribution

Contributions are welcome! Feel free to submit a pull request or open an issue on [GitHub](https://github.com/yourusername/mnemonic).


