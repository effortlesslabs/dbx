# DBX Redis Driver

Redis driver implementation for DBX database abstraction layer.

## Development Setup

1. Create a `.env` file in the crate root with the following content:

```env
REDIS_URL=redis://default:redispw@localhost:55000
```

2. Make sure you have a Redis server running at the configured URL.

3. Run the tests:

```bash
cargo test
```

## Environment Variables

- `REDIS_URL`: Redis connection URL (default: `redis://default:redispw@localhost:55000`)

## Features

- Connection pooling
- Error handling
- Command execution
- Connection lifecycle management
