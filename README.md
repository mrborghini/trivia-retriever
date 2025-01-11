# Trivia retriever

It retrieves Trivia from [Open Trivia DB](https://opentdb.com/) and writes it to
this json format:

```json
[
    {
        "question": "We need water. True or False",
        "answer": "True",
        "difficulty": "easy"
    }
]
```

## Usage

[Download the binary for your operating system](https://github.com/mrborghini/dolly_parton/releases/latest)
and then run it.

You will see all the trivia appear inside of `trivia.json`

## Building it yourself

You need to install [Rust](https://rustup.rs/).

And then run:

```bash
cargo run --release
```
