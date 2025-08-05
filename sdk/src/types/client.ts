import { HttpClientOptions } from "@/utils/index.js";

// Client configuration interface
export interface ClientConfig extends Omit<HttpClientOptions, 'apiKey'> {
  /** Base URL for the API (default: 'https://api.geolocation.com') */
  baseUrl?: string;
  
  /** Request timeout in milliseconds (default: 10000) */
  timeout?: number;
  
  /** Maximum number of retries for failed requests (default: 3) */
  maxRetries?: number;
  
  /** Additional headers to include in all requests */
  headers?: Record<string, string>;
  
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
