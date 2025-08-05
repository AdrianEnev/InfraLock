import { Request, Response, NextFunction } from 'express';
import { Unauthorized } from '../errors/Unauthorized';

// Load API keys from environment variables
const UNLIMITED_API_KEYS = new Set(
    (process.env.UNLIMITED_API_KEYS || '')
        .split(',')
        .map(key => key.trim())
        .filter(Boolean)
);

// This would be replaced with a database check in production
const VALID_API_KEYS = new Set<string>();

/**
 * Middleware to validate regular API keys (with rate limiting)
 */
export const apiKeyAuth = (req: Request, res: Response, next: NextFunction) => {
    // Skip API key check for OPTIONS requests (preflight)
    if (req.method === 'OPTIONS') {
        return next();
    }

    // Get API key from header or query parameter
    const apiKey = req.get('x-api-key') || req.query.api_key as string;
    
    if (!apiKey) {
        throw new Unauthorized('API key is required. Please provide x-api-key header or api_key query parameter.', 'api/missing-api-key');
    }
    
    // Reject unlimited keys in regular endpoints
    if (UNLIMITED_API_KEYS.has(apiKey)) {
        throw new Unauthorized('Unlimited API keys cannot be used with this endpoint. Use the /unlimited endpoints instead.', 'api/invalid-api-key');
    }
    
    // TODO: Replace with actual database check
    if (!VALID_API_KEYS.has(apiKey)) {
        console.warn(`Unauthorized access attempt with API key: ${apiKey.substring(0, 5)}...`);
        throw new Unauthorized('Invalid API key. Please check your credentials.', 'api/invalid-api-key');
    }
    
    next();
};

/**
 * Middleware to validate unlimited API keys (no rate limiting)
 */
export const unlimitedApiKeyAuth = (req: Request, res: Response, next: NextFunction) => {
    try {
        // Skip API key check for OPTIONS requests (preflight)
        if (req.method === 'OPTIONS') {
            return next();
        }

        console.log('[unlimitedApiKeyAuth] Checking API key...');
        
        // Get API key from header or query parameter
        const apiKey = req.get('x-api-key') || req.query.api_key as string;
        
        console.log(`[unlimitedApiKeyAuth] Received API key: ${apiKey ? `${apiKey.substring(0, 5)}...` : 'none'}`);
        
        if (!apiKey) {
            console.warn('[unlimitedApiKeyAuth] No API key provided');
            throw new Unauthorized('API key is required. Please provide x-api-key header or api_key query parameter.', 'api/missing-api-key');
        }
        
        console.log(`[unlimitedApiKeyAuth] Loaded UNLIMITED_API_KEYS: ${Array.from(UNLIMITED_API_KEYS).map(k => k.substring(0, 5) + '...').join(', ') || 'none'}`);
        
        // Only accept unlimited keys
        if (!UNLIMITED_API_KEYS.has(apiKey)) {
            console.warn(`[unlimitedApiKeyAuth] Invalid unlimited API key: ${apiKey.substring(0, 5)}...`);
            throw new Unauthorized('Invalid unlimited API key. Please check your credentials.', 'api/invalid-unlimited-key');
        }
        
        console.log('[unlimitedApiKeyAuth] API key validated successfully');
        next();
    } catch (error) {
        console.error('[unlimitedApiKeyAuth] Error:', error);
        next(error);
    }
};

// For testing purposes
export const _testOnly = {
    addKey: (key: string, isUnlimited = false) => {
        if (isUnlimited) {
            UNLIMITED_API_KEYS.add(key);
        } else {
            VALID_API_KEYS.add(key);
        }
    },
    removeKey: (key: string) => {
        UNLIMITED_API_KEYS.delete(key);
        VALID_API_KEYS.delete(key);
    },
    hasKey: (key: string, isUnlimited = false) => {
        return isUnlimited ? UNLIMITED_API_KEYS.has(key) : VALID_API_KEYS.has(key);
    },
};
