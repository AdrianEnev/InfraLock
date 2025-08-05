import { ApiError, NetworkError } from '../errors/index.js';

// Re-export the makeRequest function from httpClient
export * from './httpClient.js';

// Re-export urlBuilder functions
export * from './urlBuilder.js';

// src/utils/httpClient.js
export interface HttpClientOptions {
  headers?: Record<string, string>;
  signal?: AbortSignal;
}

export async function makeRequest<T>(
  url: string,
  options: HttpClientOptions = {}
): Promise<T> {
  const response = await fetch(url, {
    headers: {
      'Content-Type': 'application/json',
      ...options.headers,
    },
    signal: options.signal,
  });

  if (!response.ok) {
    const errorData = await response.json().catch(() => ({}));
    
    if (response.status >= 400 && response.status < 500) {
      throw new ApiError(
        errorData.message || 'API request failed',
        response.status,
        errorData
      );
    } else if (response.status >= 500) {
      throw new NetworkError('Server error occurred', response.status);
    }
  }

  return response.json() as Promise<T>;
}