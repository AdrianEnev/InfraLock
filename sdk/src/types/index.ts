// src/types/index.ts
export * from './requests.js';
export * from './responses.js';
export * from './client.js';

// src/types/client.ts
export interface ClientConfig {
  baseUrl?: string;
  timeout?: number;
  maxRetries?: number;
  fetchOptions?: RequestInit;
}

// src/types/requests.ts
export interface LookupRequest {
  ip?: string;
  // Add other request parameters as needed
}

// src/types/responses.ts
export interface LookupResponse {
  ip: string;
  country?: string;
  city?: string;
  isp?: string;
  // Add other response fields
}

export interface ErrorResponse {
  statusCode: number;
  message: string;
  code?: string;
}