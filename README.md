# DBX

![DBX Banner](banner.png)

A minimal API layer for all types of databases, portable across Workers, Raspberry Pi, and RISC-V boards. Written in Rust with bindings for TypeScript and other languages.

## Features

- 🚀 Fast and lightweight database abstraction layer
- 🔄 Currently implements Redis adapter with more databases planned
- 🔢 Robust Redis primitives with support for pipeline, transaction, and Lua scripts
- 🧰 Well-documented API with comprehensive examples
- 🛠️ Modern Rust implementation with configurable features
- 🧩 Modular architecture for easy extension

## Project Structure

```
dbx/
├── crates/            # Main crate containing all modules
│   ├── adapter/       # Database adapters
│   │   └── redis/     # Redis adapter implementation
│   │       ├── client.rs          # Redis client functionality
│   │       └── primitives/        # Redis primitive data types
│   │           └── string.rs      # Redis string operations
├── Cargo.toml         # Workspace configuration
└── Cargo.lock         # Dependency lock file
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

# Run doctests
cargo test --doc
```

## Development Status

This project is currently in early development. See [ROADMAP.md](ROADMAP.md) for the detailed development plan and future goals.

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
