import { Router } from 'express';
import { validateApiKey } from '../controllers/internalController';
import { authenticateService } from '../middleware/serviceAuth';

const router = Router();

/**
 * @route POST /internal/validate-key
 * @description Validate an API key (internal use only)
 * @access Internal services only
 * @body {string} apiKey - The API key to validate
 * @returns {Object} 200 - API key validation result
 * @returns {Error}  401 - Unauthorized (invalid service token or API key)
 */
router.post('/validate-key', authenticateService, validateApiKey);

export default router;
