/**
 * NetworkError represents an error that occurs during network operations
 */
export class NetworkError extends Error {
  isRetryable: boolean;
  statusCode: number;
  context: Record<string, unknown>;

  constructor(
    message: string,
    statusCode: number = 503,
    context: Record<string, unknown> = {},
    isRetryable: boolean = true
  ) {
    super(message);
    this.name = 'NetworkError';
    this.statusCode = statusCode;
    this.isRetryable = isRetryable;
    this.context = context;

    // Set the prototype explicitly to ensure instanceof works correctly
    Object.setPrototypeOf(this, NetworkError.prototype);

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
      isRetryable: this.isRetryable,
      context: this.context,
      stack: this.stack
    };
  }
}

export interface NetworkErrorContext {
  url?: string | undefined;
  method?: string | undefined;
  code?: string | undefined;
  syscall?: string | undefined;
  address?: string | undefined;
  port?: number | undefined;
  requestId?: string | undefined;
}