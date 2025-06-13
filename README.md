# DBX

![DBX Banner](banner.png)

A minimal API layer for all types of databases, portable across Workers, Raspberry Pi, and RISC-V boards. Written in Rust with bindings for TypeScript and other languages.

## Features

- 🚀 Fast and lightweight database abstraction layer
- 🔄 Support for multiple databases (SQLite, PostgreSQL, MongoDB, Redis)
- 🌐 WASM/Worker compatibility
- 📦 TypeScript bindings
- 🛠️ Simple CLI tool
- 🔌 Language bindings for Python, Ruby, C#, and Java

## Project Structure

```
dbx/
├── crates/
│   ├── dbx-core/      # Core traits and types
│   ├── dbx-sqlite/    # SQLite driver
│   ├── dbx-postgres/  # PostgreSQL driver
│   ├── dbx-mongo/     # MongoDB driver
│   └── dbx-redis/     # Redis driver
└── Cargo.toml         # Workspace configuration
```

## Getting Started

### Prerequisites

- Rust 1.75 or later
- Cargo

### Building

```bash
# Clone the repository
git clone https://github.com/effortlesslabs/dbx.git
cd dbx

# Build all crates
cargo build

# Run tests
cargo test
```

## Development Status

This project is currently in early development. See [DBX_TODO.md](DBX_TODO.md) for the current status and roadmap.

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
