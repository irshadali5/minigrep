# minigrep

A minimal `grep`-like CLI tool built while working through [The Rust Book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) (Chapter 12). The goal isn't to replace real grep — it's to have a small, readable codebase for learning how Rust CLI tools are structured from the ground up.

Most CLI tools in the ecosystem are large and jump straight into `clap`, `anyhow`, and other abstractions. This one starts bare: argument parsing by hand, error handling with `Result` and `Box<dyn Error>`, and environment variable checks with `std::env`. Every line is meant to be read and understood.

## What it does

Searches for a query string in a file and prints matching lines.

```
minigrep <query> <file_path>
```

Case-insensitive search is controlled via an environment variable:

```bash
IGNORE_CASE=1 minigrep the poem.txt
```

## Usage

```bash
# Build
cargo build --release

# Search
./target/release/minigrep to poem.txt

# Case-insensitive
IGNORE_CASE=1 ./target/release/minigrep To poem.txt
```

## Project structure

```
src/
├── main.rs   # Entry point: parses args, calls run(), prints errors
└── lib.rs    # Config struct, run(), search(), search_case_insensitive()
```

The logic lives in `lib.rs` so it can be unit tested directly. `main.rs` stays thin — it owns I/O and process exit, nothing else.

## What this teaches

- Structuring a CLI project with `main.rs` + `lib.rs`
- Returning `Box<dyn Error>` for flexible error propagation
- Reading environment variables with `std::env::var`
- Writing unit tests inside the module they test
- Separating concerns before reaching for a framework

## Roadmap

The plan is to grow this incrementally as a learning reference — each addition is a chance to see what the abstraction is actually solving:

- [ ] **`clap`** — replace manual argument parsing with a derive-based CLI
- [ ] **`anyhow`** — replace `Box<dyn Error>` with `anyhow::Result` and contextual errors
- [ ] **`sigpipe`** — handle broken pipe gracefully (e.g. `minigrep ... | head`)

## License

MIT
