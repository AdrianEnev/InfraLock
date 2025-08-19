import { makeRequest } from './utils/httpClient.js';
import { isValidIp } from './utils/urlBuilder.js';
import type { ClientConfig } from './types/client.js';
import type { LookupResponse } from './types/responses.js';
import { ValidationError } from './errors/validationError.js';
import type { HttpClientOptions } from './utils/httpClient.js';

/**
 * Client for interacting with the API
 */
export class GeolocationClient {
  private readonly config: HttpClientOptions;

  /**
   * Create a new GeolocationClient instance
   * @param config Configuration for the client
   */
  constructor(config: ClientConfig & { apiKey: string }) {
    if (!config.apiKey) {
      throw new ValidationError('API key is required', 'apiKey');
    }
    
    this.config = {
      apiKey: config.apiKey,
      // Allow overriding baseUrl; default to production URL. Strip trailing slashes.
      baseUrl: (config.baseUrl || 'https://api.geolocation.com').replace(/\/+$/, ''),
      timeout: config.timeout,
      maxRetries: config.maxRetries,
      headers: config.headers,
      fetchOptions: config.fetchOptions,
      logger: config.logger,
      enableCircuitBreaker: config.enableCircuitBreaker,
      circuitBreakerTimeout: config.circuitBreakerTimeout,
      circuitBreakerThreshold: config.circuitBreakerThreshold
    };
  }

  /**
   * Look up geolocation information for an IP address
   * @param ip IP address to look up (optional, will use the client's IP if not provided)
   * @returns Promise that resolves to the geolocation data
   * @throws {ValidationError} If the IP address is invalid
   * @throws {ApiError} If the API returns an error
   * @throws {NetworkError} If there is a network error
   */
  async lookup(ip?: string): Promise<LookupResponse> {
    try {
      if (ip && !isValidIp(ip)) {
        throw new ValidationError('Invalid IP address format', 'ip');
      }

      // Align with backend routes mounted under /api/lookup
      const endpoint = ip ? `/api/lookup/${encodeURIComponent(ip)}` : '/api/lookup';
      
      return await makeRequest<LookupResponse>(endpoint, this.config);
    } catch (error) {
      // Re-throw validation errors as-is
      if (error instanceof ValidationError) {
        throw error;
      }
      
      // For other errors, log them if a logger is available
      this.config.logger?.error('Lookup failed', { 
        ip,
        error: error instanceof Error ? error.message : String(error),
        stack: error instanceof Error ? error.stack : undefined,
      });
      
      // Re-throw the error
      throw error;
    }
  }

  /**
   * Get the current configuration (useful for debugging)
   */
  getConfig(): Omit<HttpClientOptions, 'apiKey'> & { apiKey: string } {
    return {
      ...this.config,
      apiKey: '[REDACTED]',
    };
  }
}