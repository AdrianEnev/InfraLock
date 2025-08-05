import { ApiError, NetworkError, ValidationError, createErrorResponse } from '../errors/index.js';

describe('Error Handling', () => {
  describe('ApiError', () => {
    it('should create an ApiError with default values', () => {
      const error = new ApiError('Test error');
      
      expect(error).toBeInstanceOf(Error);
      expect(error).toBeInstanceOf(ApiError);
      expect(error.name).toBe('ApiError');
      expect(error.message).toBe('Test error');
      expect(error.statusCode).toBe(500);
      expect(error.code).toBe('API_ERROR');
      expect(error.isRetryable).toBe(false);
      expect(error.context).toEqual({});
    });

    it('should create a rate limit error', () => {
      const error = ApiError.rateLimited(60);
      
      expect(error.statusCode).toBe(429);
      expect(error.code).toBe('RATE_LIMIT_EXCEEDED');
      expect(error.isRetryable).toBe(true);
      expect(error.context).toEqual({ retryAfter: 60 });
    });
  });

  describe('NetworkError', () => {
    it('should create a NetworkError with default values', () => {
      const error = new NetworkError('Connection failed');
      
      expect(error).toBeInstanceOf(Error);
      expect(error).toBeInstanceOf(NetworkError);
      expect(error.name).toBe('NetworkError');
      expect(error.message).toBe('Connection failed');
      expect(error.isRetryable).toBe(true);
      expect(error.statusCode).toBe(503);
      expect(error.context).toEqual({});
    });
  });

  describe('ValidationError', () => {
    it('should create a ValidationError with default values', () => {
      const error = new ValidationError('Invalid input', 'username');
      
      expect(error).toBeInstanceOf(Error);
      expect(error).toBeInstanceOf(ValidationError);
      expect(error.name).toBe('ValidationError');
      expect(error.message).toBe('Invalid input');
      expect(error.field).toBe('username');
      expect(error.code).toBe('VALIDATION_ERROR');
      expect(error.isRetryable).toBe(false);
      expect(error.context).toEqual({});
    });

    it('should create a required field error', () => {
      const error = ValidationError.requiredField('email');
      
      expect(error.message).toBe('This field is required');
      expect(error.field).toBe('email');
      expect(error.code).toBe('REQUIRED_FIELD');
    });
  });

  describe('createErrorResponse', () => {
    it('should create an error response from an ApiError', () => {
      const error = new ApiError('Not found', 404, 'NOT_FOUND', { resource: 'user' });
      const response = createErrorResponse(error, 'req-123');
      
      expect(response).toEqual({
        success: false,
        error: {
          message: 'Not found',
          code: 'NOT_FOUND',
          details: {
            errorId: expect.any(String),
            timestamp: expect.any(Number),
            requestId: 'req-123',
            resource: 'user'
          }
        }
      });
    });

    it('should create an error response from a generic Error', () => {
      const error = new Error('Something went wrong');
      const response = createErrorResponse(error);
      
      expect(response).toEqual({
        success: false,
        error: {
          message: 'Something went wrong',
          code: 'INTERNAL_ERROR',
          details: {
            errorId: expect.any(String),
            timestamp: expect.any(Number)
          }
        }
      });
    });
  });
});
