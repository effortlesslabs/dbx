import { DbxClient } from '../client';

// Create a new DBX client
const client = new DbxClient({
  baseUrl: 'http://localhost:3000',
  timeout: 5000,
});

async function basicUsage() {
  try {
    // Health check
    const health = await client.health();
    console.log('Health:', health);

    // Set a value
    await client.setString('my-key', 'my-value', 3600); // 1 hour TTL
    console.log('Value set successfully');

    // Get a value
    const value = await client.getString('my-key');
    console.log('Retrieved value:', value);

    // Check if key exists
    const exists = await client.exists('my-key');
    console.log('Key exists:', exists);

    // Increment a counter
    const counter = await client.incr('my-counter');
    console.log('Counter value:', counter);

    // Batch operations
    await client.batchSet({
      'batch-key-1': 'value-1',
      'batch-key-2': 'value-2',
      'batch-key-3': 'value-3',
    });

    const batchValues = await client.batchGet(['batch-key-1', 'batch-key-2', 'batch-key-3']);
    console.log('Batch values:', batchValues);

    // List keys
    const keys = await client.listKeys('batch-key-*');
    console.log('Keys matching pattern:', keys);

    // Rate limiting
    const allowed = await client.rateLimiter('user:123', 5, 60);
    console.log('Rate limit check:', allowed);

  } catch (error) {
    console.error('Error:', error);
  }
}

basicUsage(); 