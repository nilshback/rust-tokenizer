"""Pure-Python tokenizer matching the Rust `tokenize` function."""

EDGE_PUNCTUATION = ".,!?;:'\"`()[]{}…-"


def tokenize(text: str) -> list[str]:
    """Split on whitespace, strip edge punctuation, and lowercase each token."""
    tokens: list[str] = []
    for token in text.split():
        cleaned = token.strip(EDGE_PUNCTUATION).lower()
        if cleaned:
            tokens.append(cleaned)
    return tokens
