import { Router } from 'express';
import { 
  registerUser, 
  loginUser, 
  logoutUser, 
  getApiKey, 
  createApiKey, 
  getCurrentUser 
} from '../controllers/userController';
import { authenticateJWT } from '../middleware/auth';

const router = Router();

// Public routes
router.post('/register', registerUser);
router.post('/login', loginUser);

// Protected routes (require JWT authentication)
router.use(authenticateJWT);

router.get('/me', getCurrentUser);
router.post('/logout', logoutUser);
router.get('/apikey', getApiKey);
router.post('/apikey', createApiKey);

export default router;