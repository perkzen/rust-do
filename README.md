# RUST DO

RUST DO is a simple CLI todo list application written in Rust.

## Features

- [x] Add a todo
- [x] List all todos
- [x] Mark a todo as done
- [X] Delete a todo
- [X] Delete all todos


## Project structure

```
├── .github
├── migrations
├── src/
│   ├── commands
│   ├── db
│   ├── todos
│   ├── traits
│   ├── ui
│   └── main.rs
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── README.md
└── rustdo.db
```

### Running tests

```bash
cargo test
```

### Running the application

```bash
cargo run
```

### Building the application

```bash
cargo build
```
