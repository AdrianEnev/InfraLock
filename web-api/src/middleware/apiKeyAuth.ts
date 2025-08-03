import { Request, Response, NextFunction } from 'express';
import { Unauthorized } from '../errors/Unauthorized';

// Load API key from environment variable
const API_KEYS = new Set(
    (process.env.UNLIMITED_API_KEYS || '')
        .split(',')
        .map(key => key.trim())
        .filter(Boolean)
);

// Add a default key in development if none provided
/*if (process.env.NODE_ENV !== 'production' && API_KEYS.size === 0) {
    API_KEYS.add('dev-unlimited-key');
    console.warn('WARNING: Using default development API key. Set UNLIMITED_API_KEYS in production!');
}*/

/**
 * Middleware to validate API key from header or query parameter
 */
export const apiKeyAuth = (req: Request, res: Response, next: NextFunction) => {
    // Skip API key check for OPTIONS requests (preflight)
    if (req.method === 'OPTIONS') {
        return next();
    }

    // Get API key from header or query parameter
    const apiKey = req.get('x-api-key') || req.query.api_key as string;
    
    if (!apiKey) {
        return res.status(401).json({
            success: false,
            error: 'API key is required. Please provide x-api-key header or api_key query parameter.'
        });
    }
    
    if (!API_KEYS.has(apiKey)) {
        console.warn(`Unauthorized access attempt with API key: ${apiKey.substring(0, 5)}...`);
        return res.status(401).json({
            success: false,
            error: 'Invalid API key. Please check your credentials.'
        });
    }
    
    next();
};

// For testing purposes
export const _testOnly = {
    addKey: (key: string) => API_KEYS.add(key),
    removeKey: (key: string) => API_KEYS.delete(key),
    hasKey: (key: string) => API_KEYS.has(key),
};
