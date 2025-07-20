import { Router } from 'express';
import { registerUser, loginUser, getApiKey, createApiKey } from '../controllers/userController';
import { authenticateJWT } from '../middlewares/auth';

const router = Router();

// Register a new user
router.post('/register', registerUser);

// Login user
router.post('/login', loginUser);

// Get API key for authenticated user (JWT required)
router.get('/apikey', authenticateJWT, getApiKey);

// Create a new API key for authenticated user (JWT required)
router.post('/apikey', authenticateJWT, createApiKey);

// Get current user info from JWT
router.get('/me', authenticateJWT, (req, res) => {
  res.status(200).json({ user: (req as any).jwtUser });
});

export default router; 