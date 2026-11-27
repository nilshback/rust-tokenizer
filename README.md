# Rust Tokenizer

A simple word tokenizer written in Rust.  
It builds its vocabulary automatically as you type and saves it to `vocab.json`.

## How it works

- You type a sentence in the terminal.
- The program splits it into words.
- Each new word gets a unique number.
- Words that were seen before reuse their old number.
- The vocabulary is saved to `vocab.json` after every line so it is remembered next time.

## Files

- `src/main.rs` — the tokenizer
- `Cargo.toml` — Rust project settings
- `vocab.json` — the vocabulary (created automatically)
- `README.md` — this file

## How to run

1. Open a terminal in this folder.
2. Run:

```bash
cargo run
```

3. Type a sentence and press Enter.
4. Press `Ctrl+C` to quit.

## Example

```
> hello world
Tokens: [0, 1]
Saved vocab (2 words)

> hello rust
Tokens: [0, 2]
Saved vocab (3 words)
```

## Requirements

- Rust (with Cargo)
- The dependencies in `Cargo.toml` (serde and serde_json)