"""Compare the Rust and pure-Python tokenizers."""

import timeit

from python_tokenizer import tokenize as tokenize_python
from rust_tokenizer import tokenize as tokenize_rust

RUNS = 20

SAMPLE = (
    "Hello, world! This is a (simple) tokenizer: it splits on whitespace, "
    "lowercases tokens, and strips punctuation... 'It's' ready-to-run."
)


def make_text() -> str:
    return " ".join([SAMPLE] * 4000)


def main() -> None:
    text = make_text()
    rust_tokens = list(tokenize_rust(text))
    python_tokens = tokenize_python(text)
    if rust_tokens != python_tokens:
        raise SystemExit("Rust and Python tokenizers produced different results.")

    rust_seconds = timeit.timeit(lambda: tokenize_rust(text), number=RUNS)
    python_seconds = timeit.timeit(lambda: tokenize_python(text), number=RUNS)
    speedup = python_seconds / rust_seconds

    print(f"Tokens: {len(rust_tokens)} (results match)")
    print(f"Rust:   {rust_seconds:.4f}s for {RUNS} runs ({rust_seconds / RUNS * 1000:.3f} ms/run)")
    print(
        f"Python: {python_seconds:.4f}s for {RUNS} runs ({python_seconds / RUNS * 1000:.3f} ms/run)"
    )
    print(f"Speedup: {speedup:.2f}x")


if __name__ == "__main__":
    main()
