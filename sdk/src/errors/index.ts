import { NetworkError } from './networkError.js';
import { ApiError } from './apiError.js';
import { ValidationError } from './validationError.js';

export * from './apiError.js';
export * from './validationError.js';
export * from './networkError.js';

export const errors = {
  NetworkError,
  ApiError,
  ValidationError,
} as const;

export interface ErrorResponse {
  success: false;
  error: {
    message: string;
    code?: string;
    details?: Record<string, unknown>;
  };
}

type ErrorWithContext = Error & {
  context?: Record<string, unknown>;
  isRetryable?: boolean;
  statusCode?: number;
};

/**
 * Type guard to check if an error is an instance of NetworkError
 */
export function isNetworkError(error: unknown): error is NetworkError {
  return error instanceof Error && 
         'isRetryable' in error && 
         error.name === 'NetworkError';
}

/**
 * Type guard to check if an error is an instance of ApiError
 */
export function isApiError(error: unknown): error is ApiError {
  return error instanceof Error && 
         'statusCode' in error && 
         error.name === 'ApiError';
}

/**
 * Type guard to check if an error is an instance of ValidationError
 */
export function isValidationError(error: unknown): error is ValidationError {
  return error instanceof Error && 
         'isRetryable' in error && 
         error.name === 'ValidationError';
}

/**
 * Creates a standardized error response object
 */
export function createErrorResponse(
  error: unknown,
  requestId?: string
): ErrorResponse {
  const timestamp = Date.now();
  const errorId = `err_${Math.random().toString(36).substring(2, 10)}`;
  
  // Default error response
  const response: ErrorResponse = {
    success: false,
    error: {
      message: 'An unexpected error occurred',
      code: 'INTERNAL_ERROR',
      details: {
        errorId,
        timestamp,
        ...(requestId && { requestId })
      }
    }
  };

  if (error instanceof Error) {
    const err = error as ErrorWithContext;
    response.error.message = err.message;
    
    if (isApiError(err)) {
      const statusCode = err.statusCode || 500;
      response.error.code = err.code || (statusCode >= 400 && statusCode < 500 
        ? 'BAD_REQUEST' 
        : 'INTERNAL_ERROR');
      
      if (err.context) {
        response.error.details = {
          ...response.error.details,
          ...err.context
        };
      }
    }
    else if (isNetworkError(err)) {
      response.error.code = 'NETWORK_ERROR';
      response.error.details = {
        ...response.error.details,
        isRetryable: err.isRetryable,
        ...(err.context || {})
      };
    }
    else if (isValidationError(err)) {
      response.error.code = 'VALIDATION_ERROR';
      response.error.details = {
        ...response.error.details,
        isRetryable: err.isRetryable,
        ...(err.context || {})
      };
    }
  }

  return response;
}