import { Router } from 'express';
import { registerUser, loginUser, getApiKey } from '../controllers/userController';
import { authenticateApiKey, authenticateJWT } from '../middlewares/auth';

const router = Router();

// Register a new user
router.post('/register', registerUser);

// Login user
router.post('/login', loginUser);

// Get API key for authenticated user
router.get('/apikey', authenticateApiKey, getApiKey);

// Example: Get current user info from JWT
router.get('/me', authenticateJWT, (req, res) => {
  res.status(200).json({ user: (req as any).jwtUser });
});

export default router; 