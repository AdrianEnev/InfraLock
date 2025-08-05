# API Reference

This document provides detailed information about all public APIs available in the Geolocation SDK.

## Table of Contents

- [GeolocationClient](#geolocationclient)
  - [Constructor](#constructor)
  - [Methods](#methods)
    - [lookup](#lookup)
    - [batchLookup](#batchlookup)
- [Types](#types)
  - [LookupResponse](#lookupresponse)
  - [ClientConfig](#clientconfig)
- [Errors](#errors)
  - [ApiError](#apierror)
  - [NetworkError](#networkerror)
  - [ValidationError](#validationerror)

## GeolocationClient

The main class for interacting with the API.

### Constructor

```typescript
new GeolocationClient(apiKey: string, config?: ClientConfig)
```

**Parameters:**
- `apiKey`: Your API key for authentication
- `config`: Optional configuration object (see [ClientConfig](#clientconfig))

**Example:**
```typescript
const client = new GeolocationClient('your-api-key', {
  timeout: 5000,
  maxRetries: 3
});
```

### Methods

#### lookup

Look up geolocation data for an IP address.

```typescript
lookup(ip?: string): Promise<LookupResponse>
```

**Parameters:**
- `ip`: Optional IP address to look up. If not provided, looks up the client's IP.

**Returns:** Promise that resolves to a [LookupResponse](#lookupresponse) object.

**Throws:**
- [ValidationError](#validationerror) if the IP format is invalid
- [ApiError](#apierror) if the API returns an error response
- [NetworkError](#networkerror) if a network error occurs

**Example:**
```typescript
// Look up a specific IP
const result = await client.lookup('8.8.8.8');

// Look up the current IP
const currentIpInfo = await client.lookup();
```

#### batchLookup

Look up multiple IP addresses in a single batch request.

```typescript
batchLookup(ips: string[]): Promise<Array<LookupResponse | Error>>
```

**Parameters:**
- `ips`: Array of IP addresses to look up

**Returns:** Promise that resolves to an array of [LookupResponse](#lookupresponse) objects or Error objects for failed lookups.

**Example:**
```typescript
const results = await client.batchLookup(['8.8.8.8', '1.1.1.1', 'invalid-ip']);

results.forEach((result, index) => {
  if (result instanceof Error) {
    console.error(`Error for IP ${ips[index]}:`, result.message);
  } else {
    console.log(`${result.ip}: ${result.city}, ${result.country}`);
  }
});
```

## Types

### LookupResponse

```typescript
interface LookupResponse {
  ip: string;                    // The IP address that was looked up
  country?: string;              // Country name
  countryCode?: string;          // ISO 3166-1 alpha-2 country code
  region?: string;               // Region/state name
  regionCode?: string;           // Region/state code (ISO 3166-2)
  city?: string;                 // City name
  postalCode?: string;           // Postal/ZIP code
  latitude?: number;             // Geographic latitude
  longitude?: number;            // Geographic longitude
  timezone?: string;             // Timezone (e.g., "America/New_York")
  isp?: string;                  // Internet Service Provider
  organization?: string;         // Organization name
  asn?: {                       // Autonomous System Number information
    asn: number;                // AS number
    name: string;               // Organization name
    route: string;              // IP block
    domain: string;             // Organization domain
    type: string;               // ISP, HOSTING, etc.
  };
  isVpn: boolean;               // Whether the IP is a VPN
  isProxy: boolean;             // Whether the IP is a proxy
  isTor: boolean;               // Whether the IP is a Tor exit node
  threatScore: number;          // Threat score (0-100)
  threatLevel: 'low' | 'medium' | 'high'; // Threat level
  threatTypes: string[];        // Array of threat indicators
  lastSeen?: string;            // ISO timestamp when the IP was last seen
  isCrawler: boolean;           // Whether the IP is a known web crawler
  isBot: boolean;               // Whether the IP is a known bot
  mobile: {
    isMobile: boolean;          // Whether the IP is a mobile connection
    network: string;            // Mobile network name
  };
  security: {
    isTorExit: boolean;         // Whether the IP is a Tor exit node
    isAnonymous: boolean;       // Whether the IP is anonymous
    isKnownAttacker: boolean;   // Whether the IP is a known attacker
    isThreat: boolean;          // Whether the IP is considered a threat
  };
  location: {
    accuracyRadius: number;     // Accuracy radius in kilometers
    metroCode: number;          // Metro code (US only)
    averageIncome: number;      // Average income in the area (USD)
    populationDensity: number;  // Population density
    timezone: string;           // Timezone name
    inEu: boolean;              // Whether the location is in the EU
    inRu: boolean;              // Whether the location is in Russia
  };
  currency: {
    code: string;               // Currency code (e.g., "USD")
    name: string;               // Currency name
    symbol: string;             // Currency symbol
  };
  connection: {
    asn: number;                // AS number
    isp: string;                // ISP name
    organization: string;       // Organization name
  };
  lastUpdated: string;          // ISO timestamp of when the data was last updated
}
```

### ClientConfig

```typescript
interface ClientConfig {
  baseUrl?: string;             // Base URL for API requests
  timeout?: number;             // Request timeout in milliseconds
  maxRetries?: number;          // Maximum number of retry attempts
  enableCircuitBreaker?: boolean; // Whether to enable the circuit breaker
  circuitBreakerThreshold?: number; // Number of failures before opening the circuit
  circuitBreakerTimeout?: number; // Time in milliseconds to keep the circuit open
  logger?: Logger;              // Custom logger implementation
  fetchOptions?: RequestInit;   // Additional fetch options
}

interface Logger {
  debug(message: string, meta?: any): void;
  info(message: string, meta?: any): void;
  warn(message: string, meta?: any): void;
  error(message: string, meta?: any): void;
}
```

## Errors

### ApiError

Thrown when the API returns an error response.

**Properties:**
- `name`: `'ApiError'`
- `message`: Error message
- `statusCode`: HTTP status code
- `code`: Optional error code
- `isRetryable`: Whether the request can be retried
- `context`: Additional context about the error

**Example:**
```typescript
try {
  await client.lookup('invalid-ip');
} catch (error) {
  if (error instanceof ApiError) {
    console.error(`API Error (${error.statusCode}):`, error.message);
    if (error.code) {
      console.error('Error code:', error.code);
    }
  }
}
```

### NetworkError

Thrown when a network error occurs.

**Properties:**
- `name`: `'NetworkError'`
- `message`: Error message
- `code`: Error code (e.g., 'ETIMEDOUT', 'ENETWORK')
- `isRetryable`: Whether the request can be retried
- `context`: Additional context about the error

**Example:**
```typescript
try {
  await client.lookup('8.8.8.8');
} catch (error) {
  if (error instanceof NetworkError) {
    console.error('Network error:', error.message);
    if (error.code === 'ETIMEDOUT') {
      console.error('Request timed out');
    }
  }
}
```

### ValidationError

Thrown when input validation fails.

**Properties:**
- `name`: `'ValidationError'`
- `message`: Error message
- `field`: The field that failed validation
- `context`: Additional validation context

**Example:**
```typescript
try {
  await client.lookup('invalid-ip');
} catch (error) {
  if (error instanceof ValidationError) {
    console.error(`Validation error in ${error.field}:`, error.message);
  }
}
```

## Utility Functions

### isApiError

Type guard to check if an error is an `ApiError`.

```typescript
function isApiError(error: unknown): error is ApiError;
```

### isNetworkError

Type guard to check if an error is a `NetworkError`.

```typescript
function isNetworkError(error: unknown): error is NetworkError;
```

### isValidationError

Type guard to check if an error is a `ValidationError`.

```typescript
function isValidationError(error: unknown): error is ValidationError;
```

### createErrorResponse

Create a standardized error response object.

```typescript
function createErrorResponse(
  error: unknown,
  requestId?: string
): {
  success: false;
  error: {
    message: string;
    code?: string;
    statusCode?: number;
    requestId?: string;
    timestamp: number;
    details?: any;
  };
};
```

## Versioning

This SDK follows [Semantic Versioning](https://semver.org/).

## Changelog

See [CHANGELOG.md](./CHANGELOG.md) for a history of changes to this SDK.
