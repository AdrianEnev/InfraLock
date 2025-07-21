import { Request, Response, NextFunction } from 'express';
import { Unauthorized } from '../errors/Unauthorized';

const INTERNAL_SERVICE_TOKEN = process.env.INTERNAL_SERVICE_TOKEN;

if (!INTERNAL_SERVICE_TOKEN) {
  console.warn('WARNING: INTERNAL_SERVICE_TOKEN is not set. Internal endpoints will be insecure!');
}

/**
 * Middleware to authenticate internal service requests
 * Validates the presence and correctness of the service token
 */
export const authenticateService = (
  req: Request,
  res: Response,
  next: NextFunction
) => {
  // Skip auth in development if no token is set (for local development only)
  if (process.env.NODE_ENV === 'development' && !INTERNAL_SERVICE_TOKEN) {
    return next();
  }

  const authHeader = req.headers.authorization;
  
  if (!authHeader || !authHeader.startsWith('Bearer ')) {
    return next(new Unauthorized('Service token required'));
  }

  const token = authHeader.split(' ')[1];
  
  if (token !== INTERNAL_SERVICE_TOKEN) {
    return next(new Unauthorized('Invalid service token'));
  }

  next();
};
