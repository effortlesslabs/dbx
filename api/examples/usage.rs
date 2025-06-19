//! Example usage of the DBX API
//!
//! This file contains examples of how to interact with the DBX API using curl commands.
//! Make sure the API server is running on localhost:3000 before trying these examples.

fn main() {
    println!("DBX API Usage Examples");
    println!("======================");
    println!("Make sure the API server is running on localhost:3000 before trying these examples.");
    println!();

    run_all_examples();
}

/// Example 1: Basic string operations
pub fn basic_string_operations() {
    println!("=== Basic String Operations ===");

    // Set a key
    println!("1. Set a key:");
    println!("curl -X POST http://localhost:3000/api/v1/redis/strings/mykey \\");
    println!("  -H \"Content-Type: application/json\" \\");
    println!("  -d '{{\"value\": \"hello world\", \"ttl\": 3600}}'");
    println!();

    // Get a key
    println!("2. Get a key:");
    println!("curl http://localhost:3000/api/v1/redis/strings/mykey");
    println!();

    // Check if key exists
    println!("3. Check if key exists:");
    println!("curl http://localhost:3000/api/v1/redis/strings/mykey/exists");
    println!();

    // Get TTL
    println!("4. Get TTL:");
    println!("curl http://localhost:3000/api/v1/redis/strings/mykey/ttl");
    println!();

    // Delete a key
    println!("5. Delete a key:");
    println!("curl -X DELETE http://localhost:3000/api/v1/redis/strings/mykey");
    println!();
}

/// Example 2: Counter operations
pub fn counter_operations() {
    println!("=== Counter Operations ===");

    // Increment a counter
    println!("1. Increment a counter:");
    println!("curl -X POST http://localhost:3000/api/v1/redis/strings/counter/incr");
    println!();

    // Increment by specific amount
    println!("2. Increment by specific amount:");
    println!("curl -X POST http://localhost:3000/api/v1/redis/strings/counter/incrby \\");
    println!("  -H \"Content-Type: application/json\" \\");
    println!("  -d '{{\"increment\": 5}}'");
    println!();
}

/// Example 3: Batch operations
pub fn batch_operations() {
    println!("=== Batch Operations ===");

    // Set multiple keys
    println!("1. Set multiple keys:");
    println!("curl -X POST http://localhost:3000/api/v1/redis/strings/batch/set \\");
    println!("  -H \"Content-Type: application/json\" \\");
    println!("  -d '{{");
    println!("    \"key_values\": {{");
    println!("      \"key1\": \"value1\",");
    println!("      \"key2\": \"value2\",");
    println!("      \"key3\": \"value3\"");
    println!("    }},");
    println!("    \"ttl\": 3600");
    println!("  }}'");
    println!();

    // Get multiple keys
    println!("2. Get multiple keys:");
    println!("curl -X POST http://localhost:3000/api/v1/redis/strings/batch/get \\");
    println!("  -H \"Content-Type: application/json\" \\");
    println!("  -d '[\"key1\", \"key2\", \"key3\"]'");
    println!();

    // Delete multiple keys
    println!("3. Delete multiple keys:");
    println!("curl -X POST http://localhost:3000/api/v1/redis/strings/batch/delete \\");
    println!("  -H \"Content-Type: application/json\" \\");
    println!("  -d '[\"key1\", \"key2\", \"key3\"]'");
    println!();
}

/// Example 4: Advanced operations
pub fn advanced_operations() {
    println!("=== Advanced Operations ===");

    // Set if not exists (SETNX)
    println!("1. Set if not exists:");
    println!("curl -X POST http://localhost:3000/api/v1/redis/strings/unique_key/setnx \\");
    println!("  -H \"Content-Type: application/json\" \\");
    println!("  -d '{{\"value\": \"unique value\", \"ttl\": 3600}}'");
    println!();

    // Compare and set
    println!("2. Compare and set:");
    println!("curl -X POST http://localhost:3000/api/v1/redis/strings/mykey/cas \\");
    println!("  -H \"Content-Type: application/json\" \\");
    println!("  -d '{{");
    println!("    \"expected_value\": \"old value\",");
    println!("    \"new_value\": \"new value\",");
    println!("    \"ttl\": 3600");
    println!("  }}'");
    println!();
}

/// Example 5: Lua script operations
pub fn lua_script_operations() {
    println!("=== Lua Script Operations ===");

    // Rate limiter
    println!("1. Rate limiter:");
    println!("curl -X POST http://localhost:3000/api/v1/redis/scripts/rate-limiter \\");
    println!("  -H \"Content-Type: application/json\" \\");
    println!("  -d '{{");
    println!("    \"key\": \"rate_limit:user:123\",");
    println!("    \"limit\": 10,");
    println!("    \"window\": 60");
    println!("  }}'");
    println!();

    // Multi-counter
    println!("2. Multi-counter:");
    println!("curl -X POST http://localhost:3000/api/v1/redis/scripts/multi-counter \\");
    println!("  -H \"Content-Type: application/json\" \\");
    println!("  -d '{{");
    println!("    \"counters\": [");
    println!("      [\"counter1\", 5],");
    println!("      [\"counter2\", 10],");
    println!("      [\"counter3\", 15]");
    println!("    ]");
    println!("  }}'");
    println!();
}

/// Example 6: Health and info endpoints
pub fn health_and_info() {
    println!("=== Health and Info Endpoints ===");

    // Health check
    println!("1. Health check:");
    println!("curl http://localhost:3000/health");
    println!();

    // Server info
    println!("2. Server info:");
    println!("curl http://localhost:3000/info");
    println!();
}

/// Run all examples
pub fn run_all_examples() {
    health_and_info();
    println!();
    basic_string_operations();
    println!();
    counter_operations();
    println!();
    batch_operations();
    println!();
    advanced_operations();
    println!();
    lua_script_operations();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_compile() {
        // This test ensures that all example functions compile
        basic_string_operations();
        counter_operations();
        batch_operations();
        advanced_operations();
        lua_script_operations();
        health_and_info();
        run_all_examples();
    }
}
