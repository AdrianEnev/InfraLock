import { ApiError, NetworkError } from '../errors/index.js';

export interface HttpClientOptions {
  /** API key for authentication */
  apiKey: string;
  
  /** Base URL for all requests */
  baseUrl: string;
  
  /** Request headers */
  headers?: Record<string, string>;
  
  /** Request timeout in milliseconds (default: 10000) */
  timeout?: number;
  
  /** Maximum number of retries for failed requests (default: 3) */
  maxRetries?: number;
  
  /** Additional fetch options */
  fetchOptions?: RequestInit;
  
  /** Enable circuit breaker (default: true) */
  enableCircuitBreaker?: boolean;
  
  /** Time in milliseconds after which to attempt closing the circuit (default: 30000) */
  circuitBreakerTimeout?: number;

  /** Number of failures before opening the circuit (default: 5) */
  circuitBreakerThreshold?: number;
  
  /** Logger instance for debugging */
  logger?: {
    debug: (message: string, meta?: unknown) => void;
    info: (message: string, meta?: unknown) => void;
    warn: (message: string, meta?: unknown) => void;
    error: (message: string, meta?: unknown) => void;
  };
}

interface CircuitBreakerState {
  isOpen: boolean;
  failureCount: number;
  lastFailureTime: number | null;
}

// Global circuit breaker state
const circuitBreakerState: Record<string, CircuitBreakerState> = {};

/**
 * Creates a unique key for the circuit breaker based on the URL
 */
function getCircuitBreakerKey(url: string): string {
  const parsedUrl = new URL(url);
  return `${parsedUrl.hostname}${parsedUrl.pathname}`;
}

/**
 * Checks if the circuit is open for the given URL
 */
function isCircuitOpen(url: string, options: HttpClientOptions): boolean {
  if (!options.enableCircuitBreaker) return false;
  
  const key = getCircuitBreakerKey(url);
  const state = circuitBreakerState[key] || {
    isOpen: false,
    failureCount: 0,
    lastFailureTime: null,
  };
  
  // If circuit is closed, no need to check further
  if (!state.isOpen) return false;
  
  // Check if we should try to close the circuit
  const circuitBreakerTimeout = options.circuitBreakerTimeout || 30000;
  const now = Date.now();
  
  if (state.lastFailureTime && (now - state.lastFailureTime) > circuitBreakerTimeout) {
    // Half-open the circuit to test if the service is back up
    state.isOpen = false;
    options.logger?.info(`Circuit breaker half-opened for ${url}`);
    return false;
  }
  
  return true;
}

/**
 * Records a failure and updates the circuit breaker state
 */
function recordFailure(url: string, options: HttpClientOptions): void {
  if (!options.enableCircuitBreaker) return;
  
  const key = getCircuitBreakerKey(url);
  const state = circuitBreakerState[key] || {
    isOpen: false,
    failureCount: 0,
    lastFailureTime: null,
  };
  
  state.failureCount++;
  state.lastFailureTime = Date.now();
  
  const threshold = options.circuitBreakerThreshold || 5;
  if (state.failureCount >= threshold) {
    state.isOpen = true;
    options.logger?.warn(`Circuit breaker opened for ${url}`, { 
      failureCount: state.failureCount,
      lastFailureTime: state.lastFailureTime 
    });
  }
  
  circuitBreakerState[key] = state;
}

/**
 * Records a success and resets the circuit breaker state
 */
function recordSuccess(url: string): void {
  const key = getCircuitBreakerKey(url);
  // Reset the failure count when requests start succeeding
  circuitBreakerState[key] = {
    isOpen: false,
    failureCount: 0,
    lastFailureTime: null,
  };
}

/**
 * Makes an HTTP request with retry logic and circuit breaker
 */
export async function makeRequest<T>(
  endpoint: string,
  options: HttpClientOptions
): Promise<T> {
  const { 
    apiKey, 
    baseUrl, 
    headers: customHeaders = {},
    timeout = 10000,
    maxRetries = 3,
    fetchOptions = {},
    logger,
  } = options;

  const url = new URL(endpoint, baseUrl).toString();
  const method = fetchOptions.method || 'GET';
  const requestId = `req_${Math.random().toString(36).substring(2, 10)}`;
  
  // Log the request
  logger?.debug(`Making request`, { 
    url,
    method,
    requestId,
    headers: fetchOptions.headers,
  });

  // Check circuit breaker
  if (isCircuitOpen(url, options)) {
    const error = new NetworkError(
      'Service unavailable due to circuit breaker',
      undefined,
      { 
        url,
        method,
        code: 'ECIRCUITBREAKER',
        requestId,
      }
    );
    
    logger?.error('Request blocked by circuit breaker', { 
      url,
      method,
      requestId,
      error: error.message,
    });
    
    throw error;
  }

  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), timeout);

  // Create headers with proper typing
  const headers = new Headers(fetchOptions.headers);
  
  // Set default headers
  headers.set('x-api-key', apiKey);
  headers.set('Content-Type', 'application/json');
  headers.set('Accept', 'application/json');
  headers.set('x-request-id', requestId);
  
  // Add custom headers
  Object.entries(customHeaders).forEach(([key, value]) => {
    if (value) {
      headers.set(key, value);
    }
  });

  let lastError: Error | null = null;
  let attempt = 0;

  while (attempt <= maxRetries) {
    try {
      const response = await fetch(url, {
        ...fetchOptions,
        headers,
        signal: controller.signal,
      });

      clearTimeout(timeoutId);
      
      // Log the response
      logger?.debug(`Received response`, {
        url,
        method,
        status: response.status,
        statusText: response.statusText,
        requestId,
        attempt: attempt + 1,
      });

      const data = await response.json().catch(() => ({}));

      if (!response.ok) {
        throw new ApiError(
          data.message || 'API request failed',
          response.status,
          data.code || 'API_ERROR',
          data.details
        );
      }

      // Request was successful, reset circuit breaker
      recordSuccess(url);
      
      return data as T;
    } catch (error) {
      clearTimeout(timeoutId);
      attempt++;
      lastError = error as Error;

      // Log the error
      logger?.error(`Request failed (attempt ${attempt}/${maxRetries + 1})`, {
        url,
        method,
        requestId,
        error: lastError.message,
        stack: lastError.stack,
      });

      // Record failure for circuit breaker
      recordFailure(url, options);

      // Don't retry on 4xx errors (except 429 - Too Many Requests)
      if (
        lastError instanceof ApiError && 
        lastError.statusCode >= 400 && 
        lastError.statusCode < 500 &&
        lastError.statusCode !== 429
      ) {
        break;
      }

      // Don't retry on network errors after the first attempt
      if (lastError instanceof NetworkError) {
        break;
      }

      // Add a small delay before retrying
      if (attempt <= maxRetries) {
        const delay = Math.min(1000 * Math.pow(2, attempt), 30000);
        await new Promise(resolve => setTimeout(resolve, delay));
      }
    }
  }

  // If we get here, all retries failed
  if (lastError instanceof ApiError) {
    throw lastError;
  }

  if (lastError?.name === 'AbortError') {
    throw new NetworkError(
      'Request timed out',
      504, // Gateway Timeout
      { url, method, requestId, code: 'ETIMEDOUT' }
    );
  }

  throw new NetworkError(
    lastError?.message || 'Network request failed',
    500, // Internal Server Error
    { 
      url, 
      method, 
      requestId,
      code: 'ENETWORKERROR',
      originalError: lastError ? {
        name: lastError.name,
        message: lastError.message,
        stack: lastError.stack
      } : undefined
    }
  );
}
