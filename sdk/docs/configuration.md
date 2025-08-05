# Configuration Guide

This guide covers all available configuration options for the Geolocation SDK and how to customize its behavior.

## Client Configuration

The `GeolocationClient` accepts the following configuration options:

```typescript
const client = new GeolocationClient('your-api-key', {
  // Base URL for API requests (default: 'https://api.your-service.com')
  baseUrl: 'https://api.your-service.com',
  
  // Request timeout in milliseconds (default: 10000)
  timeout: 10000,
  
  // Maximum number of retry attempts for failed requests (default: 3)
  maxRetries: 3,
  
  // Enable/disable circuit breaker (default: true)
  enableCircuitBreaker: true,
  
  // Number of failures before opening the circuit (default: 5)
  circuitBreakerThreshold: 5,
  
  // Time in milliseconds to keep the circuit open (default: 30000)
  circuitBreakerTimeout: 30000,
  
  // Custom logger implementation
  logger: {
    debug: (message, meta) => console.debug(message, meta),
    info: (message, meta) => console.info(message, meta),
    warn: (message, meta) => console.warn(message, meta),
    error: (message, meta) => console.error(message, meta)
  },
  
  // Additional fetch options (passed to the underlying fetch API)
  fetchOptions: {
    // Any valid RequestInit properties
    headers: {
      'X-Custom-Header': 'value'
    },
    // ...
  }
});
```

## Default Values

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `baseUrl` | string | `'https://api.your-service.com'` | Base URL for all API requests |
| `timeout` | number | `10000` | Request timeout in milliseconds |
| `maxRetries` | number | `3` | Maximum number of retry attempts |
| `enableCircuitBreaker` | boolean | `true` | Whether to enable the circuit breaker |
| `circuitBreakerThreshold` | number | `5` | Number of failures before opening the circuit |
| `circuitBreakerTimeout` | number | `30000` | Time in milliseconds to keep the circuit open |
| `logger` | object | `console` | Logger implementation (see [Logging](#logging)) |
| `fetchOptions` | RequestInit | `{}` | Additional fetch options |

## Logging

The SDK uses a simple logging interface that can be customized. By default, it uses the `console` object, but you can provide your own logger that implements the following interface:

```typescript
interface Logger {
  debug(message: string, meta?: any): void;
  info(message: string, meta?: any): void;
  warn(message: string, meta?: any): void;
  error(message: string, meta?: any): void;
}
```

### Example: Using Winston Logger

```typescript
import winston from 'winston';

const logger = winston.createLogger({
  level: 'debug',
  format: winston.format.combine(
    winston.format.timestamp(),
    winston.format.json()
  ),
  transports: [
    new winston.transports.Console()
  ]
});

const client = new GeolocationClient('your-api-key', {
  logger: {
    debug: (message, meta) => logger.debug(message, meta),
    info: (message, meta) => logger.info(message, meta),
    warn: (message, meta) => logger.warn(message, meta),
    error: (message, meta) => logger.error(message, meta)
  }
});
```

## Circuit Breaker

The SDK includes a circuit breaker pattern to prevent cascading failures when the API is experiencing issues. The circuit breaker has three states:

1. **Closed**: Requests are allowed through normally
2. **Open**: All requests fail immediately without making network calls
3. **Half-Open**: A limited number of test requests are allowed to check if the service has recovered

### Configuration Options

- `enableCircuitBreaker`: Set to `false` to disable the circuit breaker entirely
- `circuitBreakerThreshold`: Number of consecutive failures before opening the circuit
- `circuitBreakerTimeout`: Time in milliseconds to keep the circuit open before moving to half-open state

### Monitoring Circuit State

You can monitor the circuit state using the logger:

```typescript
const client = new GeolocationClient('your-api-key', {
  logger: {
    debug: (message, meta) => {
      if (meta?.circuitState) {
        console.log(`Circuit ${meta.circuitState} for ${meta.url}`);
      }
      console.debug(message, meta);
    },
    // ... other logger methods
  }
});
```

## Custom Headers

You can add custom headers to all requests using the `fetchOptions`:

```typescript
const client = new GeolocationClient('your-api-key', {
  fetchOptions: {
    headers: {
      'X-Custom-Header': 'value',
      'X-Client-Version': '1.0.0'
    }
  }
});
```

## Timeout Configuration

Set a global timeout for all requests:

```typescript
// 5 second timeout
const client = new GeolocationClient('your-api-key', {
  timeout: 5000
});
```

## Retry Configuration

Customize the retry behavior:

```typescript
// Retry up to 5 times with exponential backoff
const client = new GeolocationClient('your-api-key', {
  maxRetries: 5
});
```

## Environment Variables

You can also configure the client using environment variables:

```bash
GEOLOCATION_API_KEY=your-api-key
GEOLOCATION_BASE_URL=https://api.your-service.com
GEOLOCATION_TIMEOUT=10000
GEOLOCATION_MAX_RETRIES=3
GEOLOCATION_ENABLE_CIRCUIT_BREAKER=true
GEOLOCATION_CIRCUIT_BREAKER_THRESHOLD=5
GEOLOCATION_CIRCUIT_BREAKER_TIMEOUT=30000
```

Then in your code:

```typescript
const client = new GeolocationClient(process.env.GEOLOCATION_API_KEY, {
  baseUrl: process.env.GEOLOCATION_BASE_URL,
  timeout: parseInt(process.env.GEOLOCATION_TIMEOUT || '10000', 10),
  maxRetries: parseInt(process.env.GEOLOCATION_MAX_RETRIES || '3', 10),
  enableCircuitBreaker: process.env.GEOLOCATION_ENABLE_CIRCUIT_BREAKER !== 'false',
  circuitBreakerThreshold: parseInt(process.env.GEOLOCATION_CIRCUIT_BREAKER_THRESHOLD || '5', 10),
  circuitBreakerTimeout: parseInt(process.env.GEOLOCATION_CIRCUIT_BREAKER_TIMEOUT || '30000', 10)
});
```

## Best Practices

1. **Production Settings**
   - Always set a reasonable timeout (e.g., 5-10 seconds)
   - Enable the circuit breaker in production
   - Use a proper logger that supports structured logging

2. **Development Settings**
   - You might want to disable the circuit breaker during development
   - Use a lower retry count for faster feedback
   - Enable debug logging for troubleshooting

3. **Performance Tuning**
   - Adjust the circuit breaker threshold based on your error budget
   - Consider increasing timeouts for batch operations
   - Monitor and adjust retry delays based on your network conditions
