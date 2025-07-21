import { Request, Response, NextFunction } from 'express';
import { cache } from '../utils/redis';
import prisma from '../prisma';
import { Unauthorized } from '../errors/Unauthorized';

// Extend the Express Request type to include the user property
declare global {
  namespace Express {
    interface Request {
      user?: {
        id: string;
        email: string;
        role: string;
        apiKey: string;
      };
    }
  }
}

// Cache TTL in seconds (5 minutes)
const API_KEY_CACHE_TTL = 300;

export const cachedApiKeyAuth = async (req: Request, res: Response, next: NextFunction) => {
  try {
    const apiKey = req.header('x-api-key');
    if (!apiKey) {
      return next(new Unauthorized('API key is required'));
    }

    // Check cache first
    const cacheKey = `api_key:${apiKey}`;
    const cachedUser = await cache.get<{
      id: string;
      email: string;
      role: string;
      apiKey: string;
    }>(cacheKey);

    if (cachedUser) {
      // Add user to request and continue
      req.user = cachedUser;
      return next();
    }

    // Not in cache, check database
    const user = await prisma.user.findUnique({ 
      where: { apiKey },
      select: {
        id: true,
        email: true,
        role: true,
        apiKey: true,
        isActive: true,
      }
    });

    if (!user || !user.isActive) {
      return next(new Unauthorized('Invalid or inactive API key'));
    }

    // Cache the user for future requests
    await cache.set(
      cacheKey, 
      { ...user, isActive: undefined }, // Don't store isActive in the cached object
      { ttl: API_KEY_CACHE_TTL }
    );

    // Add user to request and continue
    const { isActive: _, ...userData } = user; // Exclude isActive from the user object
    req.user = userData;
    next();
  } catch (error) {
    console.error('Error in cachedApiKeyAuth:', error);
    next(error);
  }
};

// Invalidate API key cache (call this when rotating or revoking keys)
export const invalidateApiKeyCache = async (apiKey: string): Promise<void> => {
  try {
    const cacheKey = `api_key:${apiKey}`;
    await cache.delete(cacheKey);
  } catch (error) {
    console.error('Error invalidating API key cache:', error);
    // Don't throw, as this is a non-critical operation
  }
};
