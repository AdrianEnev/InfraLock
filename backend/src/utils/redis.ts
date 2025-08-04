import { createClient, RedisClientType, RedisClientOptions } from 'redis';
import dotenv from 'dotenv';

dotenv.config();

// Types for our Redis client
type RedisClient = ReturnType<typeof createClient> & {
  get: (key: string) => Promise<string | null>;
  set: (key: string, value: string, options?: { EX?: number }) => Promise<string | null>;
  del: (key: string) => Promise<number>;
  quit: () => Promise<void>;
};

// Create Redis client
const createRedisClient = (): RedisClient => {
  const redisUrl = process.env.REDIS_URL || 'redis://localhost:6379';
  const clientOptions: RedisClientOptions = {
    url: redisUrl,
    socket: {
      reconnectStrategy: (retries: number): number | Error => {
        if (retries > 5) {
          console.error('Too many retries on Redis. Connection terminated');
          return new Error('Connection failed after max retries');
        }
        // Exponential backoff: 100ms, 200ms, 400ms, etc.
        return Math.min(retries * 100, 5000);
      },
    },
  };
  
  const client = createClient(clientOptions) as unknown as RedisClient;

  // Error handling
  client.on('error', (err: Error) => {
    console.error('Redis Client Error:', err);
  });

  return client;
};

// Create and connect the Redis client
const redisClient = createRedisClient();

// Cache interface
export interface CacheOptions {
  ttl?: number; // Time to live in seconds
}

// Cache service
class CacheService {
  private client: RedisClient;
  private isConnected: boolean = false;

  constructor() {
    this.client = redisClient;
    this.initialize();
  }

  private async initialize() {
    try {
      await this.client.connect();
      this.isConnected = true;
      console.log('Redis client connected');
    } catch (error) {
      console.error('Failed to connect to Redis:', error);
      this.isConnected = false;
    }
  }

  public async get<T>(key: string): Promise<T | null> {
    if (!this.isConnected) return null;

    try {
      const data = await this.client.get(key);
      return data ? JSON.parse(data) : null;
    } catch (error) {
      console.error('Cache get error:', error);
      return null;
    }
  }

  public async set<T>(key: string, value: T, options: CacheOptions = {}): Promise<boolean> {
    if (!this.isConnected) return false;

    try {
      const { ttl = 300 } = options; // Default 5 minutes TTL
      const serialized = JSON.stringify(value);
      
      if (ttl > 0) {
        await this.client.set(key, serialized, { EX: ttl });
      } else {
        await this.client.set(key, serialized);
      }
      
      return true;
    } catch (error) {
      console.error('Cache set error:', error);
      return false;
    }
  }

  public async delete(key: string): Promise<boolean> {
    if (!this.isConnected) return false;

    try {
      const result = await this.client.del(key);
      return result > 0;
    } catch (error) {
      console.error('Cache delete error:', error);
      return false;
    }
  }

  public async close(): Promise<void> {
    if (this.isConnected) {
      try {
        await this.client.quit();
        this.isConnected = false;
      } catch (error) {
        console.error('Error closing Redis connection:', error);
      }
    }
  }
}

// Export a singleton instance
export const cache = new CacheService();

// Graceful shutdown
process.on('SIGINT', async () => {
  await cache.close();
  process.exit(0);
});

process.on('SIGTERM', async () => {
  await cache.close();
  process.exit(0);
});
