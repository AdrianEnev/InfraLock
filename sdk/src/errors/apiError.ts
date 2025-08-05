/**
 * ApiError represents an error returned by the API
 */
export class ApiError extends Error {
  statusCode: number;
  code: string;
  context: Record<string, unknown>;
  isRetryable: boolean;

  constructor(
    message: string,
    statusCode: number = 500,
    code: string = 'API_ERROR',
    context: Record<string, unknown> = {},
    isRetryable: boolean = false
  ) {
    super(message);
    this.name = 'ApiError';
    this.statusCode = statusCode;
    this.code = code;
    this.context = context;
    this.isRetryable = isRetryable;

    // Set the prototype explicitly to ensure instanceof works correctly
    Object.setPrototypeOf(this, ApiError.prototype);

    // Capture stack trace, excluding constructor call from it
    if (typeof Error.captureStackTrace === 'function') {
      Error.captureStackTrace(this, this.constructor);
    } else {
      this.stack = (new Error(message)).stack;
    }
  }

  toJSON() {
    return {
      name: this.name,
      message: this.message,
      statusCode: this.statusCode,
      code: this.code,
      isRetryable: this.isRetryable,
      context: this.context,
      stack: this.stack
    };
  }

  /**
   * Creates a user-friendly error message
   */
  toUserFriendlyMessage(): string {
    const statusText = this.statusCode ? ` (${this.statusCode})` : '';
    return `API Error${statusText}: ${this.message}`;
  }

  /**
   * Creates a new ApiError for rate limiting
   */
  static rateLimited(retryAfter?: number): ApiError {
    return new ApiError(
      'Rate limit exceeded',
      429,
      'RATE_LIMIT_EXCEEDED',
      { retryAfter },
      true
    );
  }

  /**
   * Creates a new ApiError for authentication failures
   */
  static unauthorized(message: string = 'Authentication required'): ApiError {
    return new ApiError(
      message,
      401,
      'UNAUTHORIZED',
      {},
      false
    );
  }

  /**
   * Creates a new ApiError for forbidden access
   */
  static forbidden(message: string = 'Access denied'): ApiError {
    return new ApiError(
      message,
      403,
      'FORBIDDEN',
      {},
      false
    );
  }

  /**
   * Creates a new ApiError for not found resources
   */
  static notFound(resource: string, id?: string): ApiError {
    const message = id 
      ? `${resource} with ID ${id} not found`
      : `${resource} not found`;
      
    return new ApiError(
      message,
      404,
      'NOT_FOUND',
      { resource, id },
      false
    );
  }
}