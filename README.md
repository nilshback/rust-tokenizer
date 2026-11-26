# rust_tokenizer

A simple tokenizer implemented in Rust, exposed to Python with PyO3 and maturin, with a pure-Python version of the same logic and a benchmark.

`tokenize` splits text on whitespace, lowercases each token, and strips common punctuation from both edges. Tokens that are empty after stripping are dropped.

## Setup

From this directory:

```bash
python3 -m venv .venv
source .venv/bin/activate
python -m pip install maturin
maturin develop --release
python benchmark.py
```

`maturin develop --release` builds the Rust extension and installs it into the active virtual environment. `benchmark.py` checks that both implementations return the same tokens, then times each one over 20 runs.
