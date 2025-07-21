# Redis Integration

## Overview
Redis is used in the application as a high-performance in-memory data store to improve performance and enable key features.

## Purpose

### 1. Caching
- **API Responses**: Frequently accessed data is cached to reduce database load
- **User Sessions**: Session data is stored for authentication
- **Rate Limiting**: Tracks API usage per user/IP
- **API Key Validation**: Caches valid API keys to reduce database queries

### 2. Performance
- Reduces database load by serving cached data
- Enables faster response times for frequently accessed data
- Handles high traffic more efficiently

## Implementation

The Redis client is implemented in `src/utils/redis.ts` with the following features:

- **Connection Handling**: Automatic reconnection with exponential backoff
- **Error Handling**: Graceful degradation if Redis is unavailable
- **Type Safety**: Strong TypeScript types for all operations
- **Namespace Management**: Keys are properly namespaced to prevent collisions

## Configuration

### Environment Variables
```env
# Required for production
REDIS_URL=redis://username:password@host:port/db

# Development defaults
# REDIS_URL=redis://localhost:6379
```

### Development Setup
1. Install Redis: `brew install redis`
2. Start Redis: `brew services start redis`
3. Verify: `redis-cli ping` (should return "PONG")

## Usage

```typescript
import { cache } from '../utils/redis';

// Set a value with optional TTL (in seconds)
await cache.set('user:123', userData, { ttl: 300 });

// Get a value
const user = await cache.get('user:123');

// Delete a value
await cache.delete('user:123');
```

## Best Practices

1. **Key Naming**: Use colons for namespacing (e.g., `user:123:profile`)
2. **TTL**: Always set appropriate expiration times
3. **Error Handling**: Handle cases where Redis might be unavailable
4. **Monitoring**: Monitor memory usage and hit/miss ratios

## Production Considerations

- Use a managed Redis service in production
- Enable persistence if data durability is required
- Configure proper security (TLS, authentication)
- Set up monitoring and alerts

## Troubleshooting

- **Connection Issues**: Verify Redis server is running and accessible
- **Memory Usage**: Monitor and set appropriate maxmemory policies
- **Performance**: Check for slow queries with `SLOWLOG GET`

## Dependencies

- `redis`: The main Redis client library
- `@types/redis`: TypeScript type definitions
