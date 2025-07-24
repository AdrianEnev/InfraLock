import { Router } from 'express';
import { lookupIpAddress } from '../controllers/lookupController';
import { extractClientIp } from '../middleware/ipExtraction';

const router = Router();

/**
 * @route   GET /api/lookup
 * @desc    Lookup geolocation and threat data for the client's IP address
 * @access  Public (for demo purposes)
 * @middleware extractClientIp - Extracts and validates the client IP
 */
router.get('/', extractClientIp, lookupIpAddress);

/**
 * @route   GET /api/lookup/:ip
 * @desc    Lookup geolocation and threat data for a specific IP address
 * @access  Public (for demo purposes)
 * @param   {string} ip - The IP address to look up
 * @middleware extractClientIp - Extracts and validates the client IP
 */
router.get('/:ip', extractClientIp, lookupIpAddress);

export default router;
