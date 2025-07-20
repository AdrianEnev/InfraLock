import { Request, Response, NextFunction } from 'express';
import { verifyJWT, getTokenFromRequest } from '../utils/authUtils';
import prisma from '../prisma';
import { Unauthorized } from '../errors/Unauthorized';

export interface AuthenticatedRequest extends Request {
    user?: {
        id: string;
        email: string;
        role: string;
        apiKey: string;
    };
    jwtUser?: any;
}

export const authenticateApiKey = async (req: Request, res: Response, next: NextFunction) => {
    try {
        const apiKey = req.header('x-api-key');
        if (!apiKey) {
            return next(new Unauthorized('API key is required'));
        }
        
        const user = await prisma.user.findUnique({ 
            where: { apiKey },
            select: {
                id: true,
                email: true,
                role: true,
                apiKey: true
            }
        });
        
        if (!user) {
            return next(new Unauthorized('Invalid API key'));
        }
        
        (req as AuthenticatedRequest).user = user;
        next();
    } catch (error: unknown) {
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