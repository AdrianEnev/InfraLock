# Error Handling Guide

This guide covers how to handle errors in the Geolocation SDK, including error types, recovery strategies, and best practices.

## Error Types

The SDK provides several error types that you can use to handle different error scenarios:

### `ApiError`

Thrown when the API returns an error response (4xx or 5xx status codes).

**Properties:**
- `statusCode`: HTTP status code (e.g., 400, 404, 500)
- `code`: Optional error code for programmatic handling
- `isRetryable`: Boolean indicating if the request can be retried
- `context`: Additional context about the error
  - `url`: The requested URL
  - `method`: HTTP method used
  - `statusText`: HTTP status text
  - `responseData`: Parsed response body (if available)
  - `requestId`: Unique request identifier for support
  - `retryAfter`: Suggested retry delay in seconds (for 429 responses)
  - `rateLimitRemaining`: Remaining rate limit count
  - `rateLimitReset`: Timestamp when rate limit resets

### `NetworkError`

Thrown when a network-related error occurs.

**Properties:**
- `code`: Error code (e.g., 'ETIMEDOUT', 'ENETWORK')
- `isRetryable`: Boolean indicating if the request can be retried
- `context`: Additional context about the error
  - `url`: The requested URL
  - `method`: HTTP method used
  - `code`: System error code
  - `syscall`: System call that triggered the error
  - `address`: Network address that was attempted
  - `port`: Port that was attempted
  - `requestId`: Unique request identifier for support

### `ValidationError`

Thrown when input validation fails.

**Properties:**
- `field`: The field that failed validation (if applicable)
- `context`: Additional validation context
  - `field`: Field that failed validation
  - `value`: The invalid value
  - `expectedType`: Expected type (for type mismatches)
  - `receivedType`: Actual type received
  - `constraints`: Validation constraints that failed
  - `children`: Nested validation errors

## Error Handling Patterns

### Basic Error Handling

```typescript
try {
  const result = await client.lookup('8.8.8.8');
  // Handle successful response
} catch (error) {
  // Handle specific error types
  if (error instanceof ApiError) {
    console.error(`API Error (${error.statusCode}):`, error.message);
    
    // Check if the error is retryable
    if (error.isRetryable) {
      console.log('This error is retryable');
    }
    
    // Access rate limit information
    if (error.context.rateLimitRemaining !== undefined) {
      console.log(`Rate limit remaining: ${error.context.rateLimitRemaining}`);
    }
    
  } else if (error instanceof NetworkError) {
    console.error('Network Error:', error.message);
    
    // Check if the error is retryable
    if (error.isRetryable) {
      console.log('This network issue might be temporary');
    }
    
  } else if (error instanceof ValidationError) {
    console.error('Validation Error:', error.message);
    
    // Show which field failed validation
    if (error.field) {
      console.error(`Field: ${error.field}`);
    }
    
  } else {
    // Handle unexpected errors
    console.error('Unexpected error:', error);
  }
}
```

### Retry Logic

The SDK includes built-in retry logic with exponential backoff, but you can implement custom retry logic when needed:

```typescript
async function withRetry<T>(
  operation: () => Promise<T>,
  maxRetries = 3,
  baseDelay = 1000
): Promise<T> {
  let lastError: Error;
  
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      return await operation();
    } catch (error) {
      lastError = error;
      
      // Only retry on retryable errors
      if (!(error instanceof ApiError) || !error.isRetryable) {
        break;
      }
      
      // Calculate delay with exponential backoff and jitter
      const delay = Math.min(baseDelay * Math.pow(2, attempt - 1), 30000);
      const jitter = Math.random() * 1000;
      const waitTime = delay + jitter;
      
      console.log(`Attempt ${attempt} failed. Retrying in ${Math.round(waitTime)}ms...`);
      await new Promise(resolve => setTimeout(resolve, waitTime));
    }
  }
  
  throw lastError;
}

// Usage
try {
  const result = await withRetry(() => client.lookup('8.8.8.8'));
  console.log('Success:', result);
} catch (error) {
  console.error('All retry attempts failed:', error);
}
```

## Best Practices

1. **Always Check `isRetryable`**
   Before implementing custom retry logic, check the `isRetryable` flag to determine if retrying is appropriate.

2. **Respect Rate Limits**
   When receiving 429 (Too Many Requests) responses, respect the `retryAfter` value before making new requests.

3. **Log Request IDs**
   Include the `requestId` in your error logs to help with debugging and support requests.

4. **Handle Network Unreliability**
   Implement appropriate timeouts and circuit breakers in your application to handle network issues gracefully.

5. **Validate Input Early**
   Perform input validation before making API calls to reduce unnecessary network requests.

## Common Error Scenarios

### Rate Limiting

```typescript
try {
  const result = await client.lookup('8.8.8.8');
} catch (error) {
  if (error instanceof ApiError && error.statusCode === 429) {
    const retryAfter = error.context.retryAfter || 60; // Default to 60 seconds
    console.log(`Rate limited. Please wait ${retryAfter} seconds before trying again.`);
  }
}
```

### Network Timeout

```typescript
try {
  const result = await client.lookup('8.8.8.8');
} catch (error) {
  if (error instanceof NetworkError && error.context.code === 'ETIMEDOUT') {
    console.error('Request timed out. Please check your internet connection.');
  }
}
```

### Invalid Input

```typescript
try {
  const result = await client.lookup('invalid-ip');
} catch (error) {
  if (error instanceof ValidationError) {
    console.error(`Validation failed: ${error.message}`);
    if (error.context.expectedType) {
      console.error(`Expected type: ${error.context.expectedType}`);
    }
  }
}
```

## Debugging Tips

1. Enable debug logging when initializing the client:

```typescript
const client = new GeolocationClient('your-api-key', {
  logger: {
    debug: console.debug,
    info: console.info,
    warn: console.warn,
    error: console.error
  }
});
```

2. Look for the `requestId` in error responses when contacting support.

3. Check the `context` property of errors for additional debugging information.
