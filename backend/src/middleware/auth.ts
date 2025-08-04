import { Request, Response, NextFunction } from 'express';
import { verifyJWT, getTokenFromRequest } from '../utils/authUtils';
import prisma from '../prisma';
import { Unauthorized } from '../errors/Unauthorized';
import { cache } from '../utils/redis';
import { RateLimiterMemory, RateLimiterRes } from 'rate-limiter-flexible';

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

export interface AuthenticatedRequest extends Request {
    user?: {
        id: string;
        email: string;
        role: string;
        apiKey: string;
    };
    jwtUser?: any;
}

// Rate limiter configuration
const rateLimiter = new RateLimiterMemory({
  points: 100, // 100 requests
  duration: 60, // per 60 seconds by IP
  keyPrefix: 'api_key_auth',
});

export const authenticateApiKey = async (req: Request, res: Response, next: NextFunction) => {
    try {
        const apiKey = req.header('x-api-key');
        if (!apiKey) {
            return next(new Unauthorized('API key is required'));
        }

        // Rate limiting by IP
        const clientIp = req.ip || req.connection.remoteAddress || 'unknown-ip';
        try {
            await rateLimiter.consume(clientIp);
        } catch (error) {
            const rateLimiterRes = error as RateLimiterRes;
            res.set('Retry-After', String(Math.ceil(rateLimiterRes.msBeforeNext / 1000)));
            return next(new Unauthorized('Too many requests'));
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

        // Cache the user for future requests (1 hour TTL)
        await cache.set(
            cacheKey, 
            { ...user, isActive: undefined }, // Don't store isActive in the cached object
            { ttl: 3600 } // 1 hour
        );
        
        // Add user to request and continue
        const { isActive: _, ...userData } = user; // Exclude isActive from the user object
        req.user = userData;
        next();
    } catch (error: unknown) {
        console.error('Error in authenticateApiKey:', error);
        next(error);
    }
};

export const authenticateJWT = async (req: Request, res: Response, next: NextFunction) => {
    try {
        const token = getTokenFromRequest(req);
        
        if (!token) {
            return next(new Unauthorized('Authentication required'));
        }
        
        const decoded = verifyJWT(token) as { userId: string; email: string; role: string };
        
        // Verify user still exists and is active
        const user = await prisma.user.findUnique({
            where: { id: decoded.userId },
            select: {
                id: true,
                email: true,
                role: true,
                apiKey: true
            }
        });
        
        if (!user) {
            return next(new Unauthorized('User not found'));
        }
        
        // Attach user to request
        const authReq = req as AuthenticatedRequest;
        authReq.user = user;
        authReq.jwtUser = decoded;
        
        next();
    } catch (error: unknown) {
        if (error instanceof Error) {
            if (error.name === 'TokenExpiredError') {
                return next(new Unauthorized('Token has expired'));
            }
            if (error.name === 'JsonWebTokenError') {
                return next(new Unauthorized('Invalid token'));
            }
        }
        next(error);
    }
};