import { Router } from 'express';
import { lookupIpAddress } from '../controllers/lookupController';
import { extractClientIp } from '../middleware/ipExtraction';
import { apiKeyAuth } from '../middleware/apiKeyAuth';

const router = Router();

// Apply API key authentication to all lookup routes
router.use(apiKeyAuth);

/**
 * @route   GET /api/lookup
 * @desc    Lookup geolocation and threat data for the client's IP address
 * @access  Protected by API key (rate limited)
 * @header  x-api-key: Your API key
 * @middleware extractClientIp - Extracts and validates the client IP
 * @response 200 - Successful response with geolocation data
 * @response 401 - Unauthorized (missing or invalid API key)
 * @response 429 - Too Many Requests (rate limit exceeded)
 */
router.get('/', extractClientIp, lookupIpAddress);

/**
 * @route   GET /api/lookup/:ip
 * @desc    Lookup geolocation and threat data for a specific IP address
 * @access  Protected by API key (rate limited)
 * @param   {string} ip - The IP address to look up
 * @header  x-api-key: Your API key
 * @response 200 - Successful response with geolocation data
 * @response 400 - Bad request (invalid IP format)
 * @response 401 - Unauthorized (missing or invalid API key)
 * @response 429 - Too Many Requests (rate limit exceeded)
 */
router.get('/:ip', extractClientIp, lookupIpAddress);

export default router;
