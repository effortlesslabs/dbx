# DBX Redis Python SDK

A Python SDK for the DBX Redis API, modeled after the TypeScript SDK.

## Installation

Currently, this SDK is not published to PyPI. To use it locally, you can install it using pip from the local path:

```bash
pip install -e path/to/effortless-dbx/sdks/redis_py
```

Replace `path/to/effortless-dbx` with the actual path to this repository on your machine.

## Usage

```python
from redis_py import DbxClient

# Create client
client = DbxClient(base_url="http://localhost:8080", timeout=5)

# String operations
client.string.set("key", "value", ttl=3600)
value = client.string.get("key")
print(value)
```

## Future Work

- Add support for other Redis data types (hash, set, admin)
- Add WebSocket support
- Publish to PyPI for easy installation via `pip install dbx-redis-sdk`

## More Future Work

We aim to support all functionality provided by the DBX Crates Rust library in this Python SDK. This includes full coverage of all Redis data types and advanced features such as connection pooling, pipelines, transactions, Lua scripting, and admin operations.

For reference, the DBX Crates library structure and feature status can be found in the `crates/` directory, which includes:

- Adapter layer with Redis client and primitives for string, hash, set, and admin operations
- Comprehensive error handling and async support
- Planned additions like lists, sorted sets, streams, pub/sub, and cluster support

Our goal is to provide a Python SDK that matches the robustness and completeness of the Rust DBX Crates library, enabling seamless integration and feature parity across languages.
