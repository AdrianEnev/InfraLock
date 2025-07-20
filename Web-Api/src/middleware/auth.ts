import { Request, Response, NextFunction } from 'express';
import { verifyJWT } from '../utils/authUtils';
import prisma from '../prisma';

export const authenticateApiKey = async (req: Request, res: Response, next: NextFunction) => {
    const apiKey = req.header('x-api-key');
    if (!apiKey) {
        return res.status(401).json({ error: 'API key missing' });
    }
    const user = await prisma.user.findUnique({ where: { apiKey } });
    if (!user) {
        return res.status(401).json({ error: 'Invalid API key' });
    }
    (req as any).user = user;
    next();
};

export const authenticateJWT = (req: Request, res: Response, next: NextFunction) => {
    const authHeader = req.header('Authorization');
    if (!authHeader || !authHeader.startsWith('Bearer ')) {
        return res.status(401).json({ error: 'JWT token missing' });
    }
    const token = authHeader.replace('Bearer ', '');
    try {
        const decoded = verifyJWT(token);
        (req as any).jwtUser = decoded;
        next();
    } catch (err) {
        return res.status(401).json({ error: 'Invalid or expired JWT token' });
    }
}; 