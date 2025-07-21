import { Router } from 'express';
import { lookupIpAddress } from '../controllers/lookupController';

const router = Router();

/**
 * @route   GET /api/lookup
 * @desc    Lookup geolocation and threat data for the client's IP address
 * @access  Public (for demo purposes)
 */
router.get('/', lookupIpAddress);

/**
 * @route   GET /api/lookup/:ip
 * @desc    Lookup geolocation and threat data for a specific IP address
 * @access  Public (for demo purposes)
 * @param   {string} ip - The IP address to look up
 */
router.get('/:ip', lookupIpAddress);

export default router;
