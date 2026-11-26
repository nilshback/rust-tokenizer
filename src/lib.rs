use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod rust_tokenizer {
    use pyo3::prelude::*;

    /// Punctuation removed from the start and end of each token.
    const EDGE_PUNCTUATION: &[char] = &[
        '.', ',', '!', '?', ';', ':', '\'', '"', '`', '(', ')', '[', ']', '{', '}', '…', '-',
    ];

    fn is_edge_punctuation(c: char) -> bool {
        EDGE_PUNCTUATION.contains(&c)
    }

    /// Split `text` on whitespace, strip common punctuation from both edges, and lowercase each token.
    #[pyfunction]
    fn tokenize(text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|token| token.trim_matches(is_edge_punctuation).to_lowercase())
            .filter(|token| !token.is_empty())
            .collect()
    }
}
