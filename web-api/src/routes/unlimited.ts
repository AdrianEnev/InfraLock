import { Router } from 'express';
import { lookupIpAddress } from '../controllers/lookupController';
import { extractClientIp } from '../middleware/ipExtraction';
import { apiKeyAuth } from '../middleware/apiKeyAuth';

const router = Router();

// Apply API key authentication to all unlimited routes
router.use(apiKeyAuth);

/**
 * @route   GET /unlimited/lookup
 * @desc    Lookup geolocation and threat data for the client's IP address (unlimited)
 * @access  Protected by API key
 * @header  x-api-key: Your API key
 * @middleware extractClientIp - Extracts and validates the client IP
 * @response 200 - Successful response with geolocation data
 * @response 401 - Unauthorized (missing or invalid API key)
 */
router.get('/lookup', extractClientIp, lookupIpAddress);

/**
 * @route   GET /unlimited/lookup/:ip
 * @desc    Lookup geolocation and threat data for a specific IP address (unlimited)
 * @access  Protected by API key
 * @param   {string} ip - The IP address to look up
 * @header  x-api-key: Your API key
 * @response 200 - Successful response with geolocation data
 * @response 400 - Bad request (invalid IP format)
 * @response 401 - Unauthorized (missing or invalid API key)
 */
router.get('/lookup/:ip', extractClientIp, lookupIpAddress);

export default router;
