use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

fn load_vocab(path: &str) -> HashMap<String, u32> {
    if let Ok(data) = fs::read_to_string(path) {
        serde_json::from_str(&data).unwrap_or_else(|_| HashMap::new())
    } else {
        HashMap::new()
    }
}

fn save_vocab(path: &str, vocab: &HashMap<String, u32>) {
    let json = serde_json::to_string_pretty(vocab).unwrap();
    fs::write(path, json).expect("Failed to save vocab");
    println!("Saved vocab ({} words)", vocab.len());
}

fn tokenize_and_update(text: &str, vocab: &mut HashMap<String, u32>) -> Vec<u32> {
    let mut next_id = vocab.values().max().copied().unwrap_or(0);
    if !vocab.is_empty() {
        next_id += 1;
    }

    let mut tokens = Vec::new();

    for word in text.split_whitespace() {
        let id = if let Some(&existing) = vocab.get(word) {
            existing
        } else {
            vocab.insert(word.to_string(), next_id);
            let current = next_id;
            next_id += 1;
            current
        };
        tokens.push(id);
    }

    tokens
}

fn main() {
    let path = "vocab.json";
    let mut vocab = load_vocab(path);

    println!("Vocabulary currently has {} words.", vocab.len());
    println!("Type a sentence and press Enter (Ctrl+C to quit):");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let tokens = tokenize_and_update(input, &mut vocab);
        println!("Tokens: {:?}", tokens);

        save_vocab(path, &vocab);
    }
}