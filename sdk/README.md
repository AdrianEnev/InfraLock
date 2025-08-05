# Geolocation SDK

Official SDK for IP geolocation and intelligence. This SDK provides a robust interface to look up geolocation information for IP addresses.

## Features

- **IP Geolocation**: Get detailed location information for any IP address
- **Robust Error Handling**: Comprehensive error types and recovery mechanisms
- **Automatic Retries**: Configurable retry logic with exponential backoff
- **Circuit Breaker**: Prevent cascading failures with built-in circuit breaking
- **TypeScript Support**: Full TypeScript definitions included

## Installation

Install the package using npm:

```bash
npm install infralock
```

## Quick Start

```typescript
import { GeolocationClient } from 'infralock';

// Initialize the client with your API key
const client = new GeolocationClient({
  apiKey: 'your-api-key',
  timeout: 5000,
  maxRetries: 3,
  enableCircuitBreaker: true
});

// Look up geolocation for an IP address
try {
  const result = await client.lookup('8.8.8.8');
  console.log('IP Location:', result);
  
  // Example of accessing geolocation data
  console.log('Country:', result.country);
  console.log('City:', result.city);
  console.log('Coordinates:', `${result.latitude}, ${result.longitude}`);
  
} catch (error) {
  console.error('Error looking up IP:', error.message);
}
```

## Documentation

### Client Configuration

The `GeolocationClient` accepts the following configuration options:

```typescript
interface ClientConfig {
  /** Your API key (required) */
  apiKey: string;
  
  /** Request timeout in milliseconds (default: 10000) */
  timeout?: number;
  
  /** Maximum number of retry attempts (default: 3) */
  maxRetries?: number;
  
  /** Enable circuit breaker (default: true) */
  enableCircuitBreaker?: boolean;
  
  /** Custom headers to include with requests */
  headers?: Record<string, string>;
  
  /** Custom fetch options */
  fetchOptions?: RequestInit;
  
  /** Logger implementation */
  logger?: {
    info: (message: string, meta?: any) => void;
    error: (message: string, meta?: any) => void;
    debug: (message: string, meta?: any) => void;
  };
}
```

### Methods

#### `lookup(ip?: string): Promise<LookupResponse>`

Look up geolocation information for an IP address.

**Parameters:**
- `ip` (optional): IP address to look up. If not provided, the client's IP will be used.

**Returns:** A promise that resolves to the geolocation data.

**Throws:**
- `ValidationError`: If the IP address is invalid
- `ApiError`: If the API returns an error
- `NetworkError`: If there is a network error

### Error Handling

```typescript
import { ValidationError, ApiError, NetworkError } from 'infralock';

try {
  const result = await client.lookup('8.8.8.8');
} catch (error) {
  if (error instanceof ValidationError) {
    console.error('Validation error:', error.message);
  } else if (error instanceof ApiError) {
    console.error('API error:', error.statusCode, error.message);
  } else if (error instanceof NetworkError) {
    console.error('Network error:', error.message);
  } else {
    console.error('Unexpected error:', error);
  }
}
```

## License

MIT