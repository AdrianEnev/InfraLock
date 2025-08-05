/**
 * ValidationError represents an error that occurs when input validation fails
 */
export class ValidationError extends Error {
  isRetryable: boolean;
  context: Record<string, unknown>;
  field?: string;
  code?: string;

  constructor(
    message: string,
    field?: string,
    code: string = 'VALIDATION_ERROR',
    context: Record<string, unknown> = {},
    isRetryable: boolean = false
  ) {
    super(message);
    this.name = 'ValidationError';
    this.field = field;
    this.code = code;
    this.isRetryable = isRetryable;
    this.context = context;

    // Set the prototype explicitly to ensure instanceof works correctly
    Object.setPrototypeOf(this, ValidationError.prototype);

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
      code: this.code,
      field: this.field,
      isRetryable: this.isRetryable,
      context: this.context,
      stack: this.stack
    };
  }

  /**
   * Creates a user-friendly error message
   */
  public toUserFriendlyMessage(): string {
    const fieldPrefix = this.field ? `${this.field}: ` : '';
    
    // If we have specific validation constraints, include them
    if (this.context.constraints) {
      const constraints = Object.values(this.context.constraints).join('; ');
      return `${fieldPrefix}${constraints}`;
    }
    
    // If we have type information, include it in the message
    if (this.context.expectedType && this.context.receivedType) {
      return `${fieldPrefix}Expected ${this.context.expectedType}, but got ${this.context.receivedType}`;
    }
    
    // Fall back to the basic message
    return `${fieldPrefix}${this.message}`;
  }
  
  /**
   * Creates a new ValidationError for type mismatches
   */
  public static typeMismatch(
    field: string,
    expectedType: string,
    receivedValue: any
  ): ValidationError {
    return new ValidationError(
      `Expected ${expectedType}`,
      field,
      'TYPE_MISMATCH',
      {
        field,
        value: receivedValue,
        expectedType,
        receivedType: typeof receivedValue,
      }
    );
  }
  
  /**
   * Creates a new ValidationError for required fields
   */
  public static requiredField(field: string): ValidationError {
    return new ValidationError(
      'This field is required',
      field,
      'REQUIRED_FIELD',
      {
        field,
        constraints: { isNotEmpty: 'should not be empty' },
      }
    );
  }
  
  /**
   * Creates a new ValidationError for invalid format
   */
  public static invalidFormat(field: string, format: string, value: any): ValidationError {
    return new ValidationError(
      `Invalid ${format} format`,
      field,
      'INVALID_FORMAT',
      {
        field,
        value,
        constraints: { [format]: `must be a valid ${format}` },
      }
    );
  }
}